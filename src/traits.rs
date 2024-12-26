use crate::errors::DnsError;

pub trait Serializable {
    fn serialize(&mut self) -> Result<Vec<u8>, DnsError>;
}
