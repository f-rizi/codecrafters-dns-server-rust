use std::result::Result;

#[derive(Debug, Default)]
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

impl Header {
    pub fn parse_header(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        if bytes.len() < 12 {
            return Err("Header size must be at least 12 bytes");
        }

        self.ID = u16::from_be_bytes([bytes[0], bytes[1]]);

        // Getting flags:
        let flags_part1 = bytes[2];
        let flags_part2 = bytes[3];

        self.QR = (flags_part1 & 0x80) >> 7;
        self.OPCODE = (flags_part1 & 0x78) >> 3;
        self.AA = (flags_part1 & 0x04) >> 2;
        self.TC = (flags_part1 & 0x02) >> 1;
        self.RD = flags_part1 & 0x01;

        self.RA = (flags_part2 & 0x80) >> 7;
        self.Z = (flags_part2 & 0x70) >> 4;
        self.RCODE = flags_part2 & 0x0F;

        // Getting rest of header items
        self.QDCOUNT = u16::from_be_bytes([bytes[4], bytes[5]]);
        self.ANCOUNT = u16::from_be_bytes([bytes[6], bytes[7]]);
        self.NSCOUNT = u16::from_be_bytes([bytes[8], bytes[9]]);
        self.ARCOUNT = u16::from_be_bytes([bytes[10], bytes[11]]);

        Ok(())
    }

    pub fn create_header_as_array_of_bytes(&mut self) -> Result<[u8; 12], &'static str> {
        let mut bytes = [0u8; 12];

        bytes[0] = self.ID.to_be_bytes()[0];
        bytes[1] = self.ID.to_be_bytes()[1];

        let temp = (self.QR << 7) | (self.OPCODE << 3) | (self.AA << 2) | (self.TC << 1) | self.RD;

        let temp2 = (self.RA << 7) | (self.Z << 4) | self.RCODE;

        bytes[2] = temp;
        bytes[3] = temp2;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_header_valid() {
        let bytes: [u8; 12] = [
            0x12, 0x34, 0x85, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
        ];
        let mut header = Header::default();

        assert!(header.parse_header(&bytes).is_ok());
        assert_eq!(header.ID, 0x1234);
        assert_eq!(header.QDCOUNT, 1);
    }

    #[test]
    fn test_parse_header_invalid_size() {
        let bytes: [u8; 10] = [0x12, 0x34, 0x85, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00];
        let mut header = Header {
            ID: 0,
            QR: 0,
            OPCODE: 0,
            AA: 0,
            TC: 0,
            RD: 0,
            RA: 0,
            Z: 0,
            RCODE: 0,
            QDCOUNT: 0,
            ANCOUNT: 0,
            NSCOUNT: 0,
            ARCOUNT: 0,
        };

        assert!(header.parse_header(&bytes).is_err());
    }

    #[test]
    fn test_create_header_as_array_of_bytes() {
        let mut header = Header {
            ID: 0x1234,
            QR: 1,
            OPCODE: 1,
            AA: 0,
            TC: 0,
            RD: 1,
            RA: 1,
            Z: 0,
            RCODE: 0,
            QDCOUNT: 1,
            ANCOUNT: 0,
            NSCOUNT: 0,
            ARCOUNT: 1,
        };

        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        assert_eq!(
            bytes,
            [0x12, 0x34, 0x89, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]
        );
    }
}
