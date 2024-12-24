use std::result::Result;

#[derive(Debug, Default)]
pub struct Answer<'a> {
    pub name: &'a [u8],
    pub q_type: u16,
    pub q_class: u16,
    pub TTL: u32,
    pub Length: u16,
    pub Data: Vec<u8>,
}

impl Answer<'_> {
    pub fn parse_answers(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn create_answer_as_array_of_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let mut answer: Vec<u8> = Vec::new();

        answer.extend_from_slice(&self.name);

        answer.push(0);

        answer.extend_from_slice(&self.q_type.to_be_bytes());
        answer.extend_from_slice(&self.q_class.to_be_bytes());
        answer.extend_from_slice(&self.TTL.to_be_bytes());
        answer.extend_from_slice(&self.Length.to_be_bytes());

        for part in &self.Data {
            answer.extend_from_slice(&part.to_be_bytes());
        }

        Ok(answer)
    }
}
