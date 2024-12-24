use std::result::Result;

use crate::{answer::Answer, header::Header, question::Question};

#[derive(Debug, Default)]
pub struct Message {
    bytes: Vec<u8>,
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn parse_message(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.bytes = bytes.to_vec();

        self.header = self.parse_header()?;
        self.questions = self.parse_questions()?;
        self.answers = self.parse_answers()?;

        self.header.ANCOUNT = self.header.QDCOUNT;
        self.header.QR = 1;

        if self.header.OPCODE != 0 {
            self.header.RCODE = 4;
        }

        Ok(())
    }

    pub fn parse_header(&mut self) -> Result<Header, String> {
        if self.bytes.len() < 12 {
            return Err("Header size must be at least 12 bytes".to_string());
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

    pub fn parse_answers(&mut self) -> Result<Vec<Answer>, String> {
        let mut answers: Vec<Answer> = Vec::new();

        for question in self.questions.iter() {
            let mut answer = Answer::default();
            answer.name = question.name.clone();
            answer.q_type = question.q_type;
            answer.q_class = question.q_class;
            answer.TTL = 40;
            answer.Length = 4;
            answer.Data = vec![8, 8, 8, 8];
            answers.push(answer);
        }

        Ok(answers)
    }

    pub fn parse_questions(&mut self) -> Result<Vec<Question>, String> {
        let mut questions = Vec::new();
        let mut offset = 12;

        while questions.len() < self.header.QDCOUNT.into() {
            let question = self.parse_question(offset)?;
            questions.push(question.0);
            offset = question.1;
        }

        Ok(questions)
    }

    pub fn parse_question(&mut self, mut offset: usize) -> Result<(Question, usize), String> {
        let mut question = Question::default();

        let (name_bytes, consumed) = self.parse_name(offset)?;
        offset = consumed;
        question.name = name_bytes;

        if offset + 4 > self.bytes.len() {
            return Err("Not enough bytes for QTYPE and QCLASS".to_string());
        }
        question.q_type = u16::from_be_bytes([self.bytes[offset], self.bytes[offset + 1]]);
        question.q_class = u16::from_be_bytes([self.bytes[offset + 2], self.bytes[offset + 3]]);
        offset += 4;

        Ok((question, offset))
    }

    fn parse_name(&self, mut offset: usize) -> Result<(Vec<u8>, usize), String> {
        let mut result = Vec::new();

        loop {
            if offset > self.bytes.len() {
                return Err("Ran out of bytes while parsing name".to_string());
            }

            let len = self.bytes[offset];

            if len == 0 {
                result.push(0);
                offset += 1;
                break;
            } else if len & 0xC0 == 0xC0 {
                let pointer_offset =
                    ((((len & 0x3F) as u16) << 8) | (self.bytes[offset + 1] as u16)) as usize;

                let (sub_bytes, _) = self.parse_name(pointer_offset)?;

                offset += 2;

                result.extend_from_slice(&sub_bytes);
                break;
            } else {
                offset += 1;
                if offset + (len as usize) > self.bytes.len() {
                    return Err("Label extends past end of buffer".to_string());
                }
                let label = &self.bytes[offset..offset + (len as usize)];
                offset += len as usize;
                result.push(len);
                result.extend_from_slice(label);
            }
        }

        Ok((result, offset))
    }

    pub fn create_answers_as_array_of_bytes(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut bytes = Vec::new();

        for answer in self.answers.iter_mut() {
            let answer_array_result = answer.create_answer_as_array_of_bytes();

            if let answer_array = answer_array_result.unwrap() {
                bytes.extend_from_slice(&answer_array);
            }
        }

        Ok(bytes)
    }

    pub fn create_questions_as_array_of_bytes(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut bytes = Vec::new();

        for question in self.questions.iter_mut() {
            let question_array_result = question.create_question_as_array_of_bytes();

            if let question_array = question_array_result.unwrap() {
                bytes.extend_from_slice(&question_array);
            }
        }

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::header;

    use super::*;

    #[test]
    fn test_parse_header_valid() {
        let bytes: [u8; 12] = [
            0x12,
            0x34,        // ID: 0x1234
            0b1001_0101, // Flags_part1: QR=1, OPCODE=2, AA=1, TC=0, RD=1 => 0x95
            0b0101_0011, // Flags_part2: RA=0, Z=5, RCODE=3 => 0x53
            0x00,
            0x01, // QDCOUNT: 1
            0x00,
            0x02, // ANCOUNT: 2
            0x00,
            0x03, // NSCOUNT: 3
            0x00,
            0x04, // ARCOUNT: 4
        ];
        let mut message = Message::default();
        message.bytes = bytes.to_vec();

        let message_header = message.parse_header();

        // Parse the header
        assert!(message.parse_header().is_ok());

        let header = message_header.unwrap();

        // Assert each field
        assert_eq!(header.ID, 0x1234);
        assert_eq!(header.QR, 1);
        assert_eq!(header.OPCODE, 2);
        assert_eq!(header.AA, 1);
        assert_eq!(header.TC, 0);
        assert_eq!(header.RD, 1);
        assert_eq!(header.RA, 0);
        assert_eq!(header.Z, 5);
        assert_eq!(header.RCODE, 3);
        assert_eq!(header.QDCOUNT, 1);
        assert_eq!(header.ANCOUNT, 2);
        assert_eq!(header.NSCOUNT, 3);
        assert_eq!(header.ARCOUNT, 4);
    }

    #[test]
    fn test_parse_single_questions_valid() {
        let bytes: [u8; 33] = [
            19, 58, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 12, 99, 111, 100, 101, 99, 114, 97, 102, 116,
            101, 114, 115, 2, 105, 111, 0, 0, 1, 0, 1,
        ];
        let mut message = Message::default();
        message.bytes = bytes.to_vec();

        let message_header = message.parse_header();
        message.header = message_header.unwrap();

        let message_questions = message.parse_questions();
        assert!(message.parse_header().is_ok());

        let qustions = message_questions.unwrap();

        assert_eq!(qustions.len(), 1);

        let question = &qustions[0];
        assert_eq!(question.q_type, 1);
        assert_eq!(question.q_class, 1);

        let name: Vec<u8> = vec![
            12, 99, 111, 100, 101, 99, 114, 97, 102, 116, 101, 114, 115, 2, 105, 111,
        ];
        assert_eq!(question.name, name);
    }

    #[test]
    fn test_parse_two_questions_valid() {
        let bytes: [u8; 62] = [
            164, 29, 1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115,
            115, 100, 111, 109, 97, 105, 110, 110, 97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1, 3,
            100, 101, 102, 192, 16, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let mut message = Message::default();
        message.bytes = bytes.to_vec();

        let message_header = message.parse_header();
        message.header = message_header.unwrap();

        let message_questions = message.parse_questions();
        assert!(message.parse_header().is_ok());

        let qustions = message_questions.unwrap();

        assert_eq!(qustions.len(), 2);

        let question = &qustions[0];
        assert_eq!(question.q_type, 1);
        assert_eq!(question.q_class, 1);

        let name: Vec<u8> = vec![
            3, 97, 98, 99, 17, 108, 111, 110, 103, 97, 115, 115, 100, 111, 109, 97, 105, 110, 110,
            97, 109, 101, 3, 99, 111, 109, 0, 0, 1, 0, 1,
        ];
        // assert_eq!(question.name, name);
    }
}
