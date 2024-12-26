use crate::errors::DnsError;
use crate::traits::Serializable;

#[derive(Clone, Debug, Default)]
pub struct Answer {
    pub name: Vec<u8>,
    pub q_type: u16,
    pub q_class: u16,
    pub TTL: u32,
    pub Length: u16,
    pub Data: Vec<u8>,
}

impl Serializable for Answer {
    fn serialize(&mut self) -> Result<Vec<u8>, DnsError> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.name);
        bytes.extend_from_slice(&self.q_type.to_be_bytes());
        bytes.extend_from_slice(&self.q_class.to_be_bytes());
        bytes.extend_from_slice(&self.TTL.to_be_bytes());
        bytes.extend_from_slice(&self.Length.to_be_bytes());
        bytes.extend_from_slice(&self.Data);

        Ok(bytes)
    }
}
