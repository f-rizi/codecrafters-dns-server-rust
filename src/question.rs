use std::result::Result;

#[derive(Clone, Debug, Default)]
pub struct Question {
    pub name: Vec<u8>, // Vector for storing the domain name
    pub q_type: u16,   // Query type (e.g., A)
    pub q_class: u16,  // Query class (e.g., IN)
}

impl Question {
    pub fn create_question_as_array_of_bytes(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.name);

        bytes.extend_from_slice(&self.q_type.to_be_bytes());

        bytes.extend_from_slice(&self.q_class.to_be_bytes());

        Ok(bytes)
    }
}
