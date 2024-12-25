use std::result::Result;

use crate::errors::DnsError;

use super::{answer::Answer, header::Header, question::Question};

#[derive(Debug, Default)]
pub struct Message {
    bytes: Vec<u8>,
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn parse_message(&mut self, bytes: &[u8]) -> Result<(), DnsError> {
        self.bytes = bytes.to_vec();

        self.header = self.parse_header()?;
        self.questions = self.parse_questions()?;

        if self.header.OPCODE != 0 {
            self.header.RCODE = 4;
        }

        if self.header.ANCOUNT > 0 {
            self.answers = self.parse_remote_answers()?;
        }

        Ok(())
    }

    pub fn parse_header(&mut self) -> Result<Header, DnsError> {
        if self.bytes.len() < 12 {
            return Err(DnsError::Parse(
                "Header size must be at least 12 bytes".to_string(),
            ));
        }

        let mut header = Header::default();

        header.ID = u16::from_be_bytes([self.bytes[0], self.bytes[1]]);

        let flags_part1 = self.bytes[2];
        let flags_part2 = self.bytes[3];

        header.QR = (flags_part1 & 0x80) >> 7;
        header.OPCODE = (flags_part1 & 0x78) >> 3;
        header.AA = (flags_part1 & 0x04) >> 2;
        header.TC = (flags_part1 & 0x02) >> 1;
        header.RD = flags_part1 & 0x01;

        header.RA = (flags_part2 & 0x80) >> 7;
        header.Z = (flags_part2 & 0x70) >> 4;
        header.RCODE = flags_part2 & 0x0F;

        header.QDCOUNT = u16::from_be_bytes([self.bytes[4], self.bytes[5]]);
        header.ANCOUNT = u16::from_be_bytes([self.bytes[6], self.bytes[7]]);
        header.NSCOUNT = u16::from_be_bytes([self.bytes[8], self.bytes[9]]);
        header.ARCOUNT = u16::from_be_bytes([self.bytes[10], self.bytes[11]]);

        Ok(header)
    }

    pub fn parse_questions(&mut self) -> Result<Vec<Question>, DnsError> {
        let mut questions = Vec::new();
        let mut offset = 12;

        for _ in 0..self.header.QDCOUNT {
            let (question, new_offset) = self.parse_question(offset)?;
            questions.push(question);
            offset = new_offset;
        }

        Ok(questions)
    }

    pub fn parse_question(&mut self, mut offset: usize) -> Result<(Question, usize), DnsError> {
        let (name_bytes, consumed) = self.parse_name(offset)?;
        offset = consumed;

        if offset + 4 > self.bytes.len() {
            return Err(DnsError::Parse(
                "Not enough bytes for QTYPE/QCLASS".to_string(),
            ));
        }

        let q_type = u16::from_be_bytes([self.bytes[offset], self.bytes[offset + 1]]);
        let q_class = u16::from_be_bytes([self.bytes[offset + 2], self.bytes[offset + 3]]);
        offset += 4;

        Ok((
            Question {
                name: name_bytes,
                q_type,
                q_class,
            },
            offset,
        ))
    }

    fn parse_name(&self, mut offset: usize) -> Result<(Vec<u8>, usize), DnsError> {
        let mut result = Vec::new();

        loop {
            if offset >= self.bytes.len() {
                return Err(DnsError::Parse(
                    "Ran out of bytes while parsing name".to_string(),
                ));
            }

            let len = self.bytes[offset];

            if len == 0 {
                result.push(0);
                offset += 1;
                break;
            } else if len & 0xC0 == 0xC0 {
                if offset + 1 >= self.bytes.len() {
                    return Err(DnsError::Parse(
                        "Not enough bytes for name pointer".to_string(),
                    ));
                }
                let pointer_offset =
                    ((((len & 0x3F) as u16) << 8) | (self.bytes[offset + 1] as u16)) as usize;
                offset += 2;

                let (sub_bytes, _) = self.parse_name(pointer_offset)?;
                result.extend_from_slice(&sub_bytes);
                break;
            } else {
                offset += 1;
                if offset + (len as usize) > self.bytes.len() {
                    return Err(DnsError::Parse(
                        "Label extends past end of buffer".to_string(),
                    ));
                }
                let label = &self.bytes[offset..offset + (len as usize)];
                offset += len as usize;

                result.push(len);
                result.extend_from_slice(label);
            }
        }

        Ok((result, offset))
    }

    fn parse_remote_answers(&mut self) -> Result<Vec<Answer>, DnsError> {
        let mut answers = Vec::new();

        let mut offset = 12;
        for _ in 0..self.header.QDCOUNT {
            let (_, new_offset) = self.parse_question(offset)?;
            offset = new_offset;
        }

        for _ in 0..self.header.ANCOUNT {
            let (ans, new_offset) = self.parse_answer(offset)?;
            offset = new_offset;
            answers.push(ans);
        }

        Ok(answers)
    }

    fn parse_answer(&mut self, mut offset: usize) -> Result<(Answer, usize), DnsError> {
        let (name, consumed) = self.parse_name(offset)?;
        offset = consumed;

        if offset + 10 > self.bytes.len() {
            return Err(DnsError::Parse(
                "Not enough bytes to parse answer header".to_string(),
            ));
        }
        let q_type = u16::from_be_bytes([self.bytes[offset], self.bytes[offset + 1]]);
        let q_class = u16::from_be_bytes([self.bytes[offset + 2], self.bytes[offset + 3]]);
        let ttl = u32::from_be_bytes([
            self.bytes[offset + 4],
            self.bytes[offset + 5],
            self.bytes[offset + 6],
            self.bytes[offset + 7],
        ]);
        let rdlength = u16::from_be_bytes([self.bytes[offset + 8], self.bytes[offset + 9]]);
        offset += 10;

        if offset + rdlength as usize > self.bytes.len() {
            return Err(DnsError::Parse("Not enough bytes for RDATA".to_string()));
        }
        let rdata = self.bytes[offset..offset + (rdlength as usize)].to_vec();
        offset += rdlength as usize;

        let answer = Answer {
            name,
            q_type,
            q_class,
            TTL: ttl,
            Length: rdlength,
            Data: rdata,
        };

        Ok((answer, offset))
    }

    pub fn parse_answers(&mut self) -> Result<Vec<Answer>, DnsError> {
        let mut answers = Vec::new();
        for question in &self.questions {
            let mut a = Answer::default();
            a.name = question.name.clone();
            a.q_type = question.q_type;
            a.q_class = question.q_class;
            a.TTL = 40;
            a.Length = 4;
            a.Data = vec![8, 8, 8, 8];
            answers.push(a);
        }
        Ok(answers)
    }

    pub fn create_response_bytes(&mut self) -> Result<Vec<u8>, DnsError> {
        let header_bytes = self
            .header
            .create_header_as_array_of_bytes()
            .map_err(|_| DnsError::Serialization("Failed to serialize header".to_string()))?;
        let question_bytes = self
            .create_questions_as_array_of_bytes()
            .map_err(|_| DnsError::Serialization("Failed to serialize questions".to_string()))?;

        let answer_bytes = self
            .create_answers_as_array_of_bytes()
            .map_err(|_| DnsError::Serialization("Failed to serialize answers".to_string()))?;

        let mut combined = Vec::new();
        combined.extend_from_slice(&header_bytes);
        combined.extend_from_slice(&question_bytes);
        combined.extend_from_slice(&answer_bytes);

        Ok(combined)
    }

    pub fn create_questions_as_array_of_bytes(&mut self) -> Result<Vec<u8>, DnsError> {
        let mut bytes = Vec::new();
        for q in &mut self.questions {
            let question_bytes = q.create_question_as_array_of_bytes()?;
            bytes.extend_from_slice(&question_bytes);
        }
        Ok(bytes)
    }

    pub fn create_answers_as_array_of_bytes(&mut self) -> Result<Vec<u8>, DnsError> {
        let mut bytes = Vec::new();
        for ans in &mut self.answers {
            let answer_bytes = ans.create_answer_as_array_of_bytes()?;
            bytes.extend_from_slice(&answer_bytes);
        }
        Ok(bytes)
    }
}
