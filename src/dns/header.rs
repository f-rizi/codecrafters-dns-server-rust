use crate::errors::DnsError;
use crate::traits::Serializable;

#[derive(Clone, Debug, Default)]
pub struct Header {
    pub ID: u16,
    pub QR: u8,
    pub OPCODE: u8,
    pub AA: u8,
    pub TC: u8,
    pub RD: u8,
    pub RA: u8,
    pub Z: u8,
    pub RCODE: u8,
    pub QDCOUNT: u16,
    pub ANCOUNT: u16,
    pub NSCOUNT: u16,
    pub ARCOUNT: u16,
}

impl Serializable for Header {
    fn serialize(&mut self) -> Result<Vec<u8>, DnsError> {
        let mut bytes: Vec<u8> = vec![0; 12];

        bytes[0] = self.ID.to_be_bytes()[0];
        bytes[1] = self.ID.to_be_bytes()[1];

        let flag_part_1 =
            (self.QR << 7) | (self.OPCODE << 3) | (self.AA << 2) | (self.TC << 1) | self.RD;
        let flag_part_2 = (self.RA << 7) | (self.Z << 4) | self.RCODE;

        bytes[2] = flag_part_1;
        bytes[3] = flag_part_2;

        bytes[4] = self.QDCOUNT.to_be_bytes()[0];
        bytes[5] = self.QDCOUNT.to_be_bytes()[1];
        bytes[6] = self.ANCOUNT.to_be_bytes()[0];
        bytes[7] = self.ANCOUNT.to_be_bytes()[1];
        bytes[8] = self.NSCOUNT.to_be_bytes()[0];
        bytes[9] = self.NSCOUNT.to_be_bytes()[1];
        bytes[10] = self.ARCOUNT.to_be_bytes()[0];
        bytes[11] = self.ARCOUNT.to_be_bytes()[1];

        Ok(bytes)
    }
}
