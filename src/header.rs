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
        // Construct a valid header byte array
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
        let mut header = Header::default();

        // Parse the header
        assert!(header.parse_header(&bytes).is_ok());

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
    fn test_create_header_as_array_of_bytes() {
        // Create a header with specific field values
        let mut header = Header {
            ID: 0x1234,
            QR: 1,
            OPCODE: 2,
            AA: 1,
            TC: 0,
            RD: 1,
            RA: 0,
            Z: 5,
            RCODE: 3,
            QDCOUNT: 1,
            ANCOUNT: 2,
            NSCOUNT: 3,
            ARCOUNT: 4,
        };

        // Serialize the header
        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        // Expected byte array
        let expected_bytes: [u8; 12] = [
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

        // Assert that the serialized bytes match the expected bytes
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_create_header_with_max_values() {
        // Create a header with maximum possible values
        let mut header = Header {
            ID: u16::MAX, // 65535
            QR: 1,
            OPCODE: 15, // Max 4-bit value
            AA: 1,
            TC: 1,
            RD: 1,
            RA: 1,
            Z: 7,              // Max 3-bit value
            RCODE: 15,         // Max 4-bit value
            QDCOUNT: u16::MAX, // 65535
            ANCOUNT: u16::MAX,
            NSCOUNT: u16::MAX,
            ARCOUNT: u16::MAX,
        };

        // Serialize the header
        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        // Expected byte array with maximum values
        let expected_bytes: [u8; 12] = [
            0xFF,
            0xFF,        // ID: 0xFFFF
            0b1111_1111, // Flags_part1: QR=1, OPCODE=15, AA=1, TC=1, RD=1 => 0xFF
            0b1111_1111, // Flags_part2: RA=1, Z=7, RCODE=15 => 0xFF
            0xFF,
            0xFF, // QDCOUNT: 65535
            0xFF,
            0xFF, // ANCOUNT: 65535
            0xFF,
            0xFF, // NSCOUNT: 65535
            0xFF,
            0xFF, // ARCOUNT: 65535
        ];

        // Assert that the serialized bytes match the expected bytes
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_create_header_with_min_values() {
        // Create a header with minimum possible values (all fields set to 0)
        let mut header = Header::default();

        // Serialize the header
        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        // Expected byte array with minimum values
        let expected_bytes: [u8; 12] = [
            0x00,
            0x00,        // ID: 0x0000
            0b0000_0000, // Flags_part1: All flags = 0 => 0x00
            0b0000_0000, // Flags_part2: All flags = 0 => 0x00
            0x00,
            0x00, // QDCOUNT: 0
            0x00,
            0x00, // ANCOUNT: 0
            0x00,
            0x00, // NSCOUNT: 0
            0x00,
            0x00, // ARCOUNT: 0
        ];

        // Assert that the serialized bytes match the expected bytes
        assert_eq!(bytes, expected_bytes);
    }

    #[test]
    fn test_parse_and_serialize_round_trip() {
        let mut original_header = Header {
            ID: 0x1A2B,
            QR: 1,
            OPCODE: 4,
            AA: 1,
            TC: 1,
            RD: 1,
            RA: 1,
            Z: 0,
            RCODE: 0,
            QDCOUNT: 10,
            ANCOUNT: 20,
            NSCOUNT: 30,
            ARCOUNT: 40,
        };

        // Serialize the original header
        let bytes = original_header
            .create_header_as_array_of_bytes()
            .expect("Failed to serialize header");

        // Parse the bytes into a new header
        let mut parsed_header = Header::default();
        assert!(parsed_header.parse_header(&bytes).is_ok());

        // Assert that the original and parsed headers are identical
        assert_eq!(original_header.ID, parsed_header.ID);
        assert_eq!(original_header.QR, parsed_header.QR);
        assert_eq!(original_header.OPCODE, parsed_header.OPCODE);
        assert_eq!(original_header.AA, parsed_header.AA);
        assert_eq!(original_header.TC, parsed_header.TC);
        assert_eq!(original_header.RD, parsed_header.RD);
        assert_eq!(original_header.RA, parsed_header.RA);
        assert_eq!(original_header.Z, parsed_header.Z);
        assert_eq!(original_header.RCODE, parsed_header.RCODE);
        assert_eq!(original_header.QDCOUNT, parsed_header.QDCOUNT);
        assert_eq!(original_header.ANCOUNT, parsed_header.ANCOUNT);
        assert_eq!(original_header.NSCOUNT, parsed_header.NSCOUNT);
        assert_eq!(original_header.ARCOUNT, parsed_header.ARCOUNT);
    }

    #[test]
    fn test_multiple_headers_round_trip() {
        let headers = vec![
            Header {
                ID: 0x0000,
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
            },
            Header {
                ID: 0xFFFF,
                QR: 1,
                OPCODE: 15, // Max 4-bit value
                AA: 1,
                TC: 1,
                RD: 1,
                RA: 1,
                Z: 7,              // Max 3-bit value
                RCODE: 15,         // Max 4-bit value
                QDCOUNT: u16::MAX, // 65535
                ANCOUNT: u16::MAX,
                NSCOUNT: u16::MAX,
                ARCOUNT: u16::MAX,
            },
            Header {
                ID: 0x1A2B,
                QR: 0,
                OPCODE: 4,
                AA: 0,
                TC: 1,
                RD: 0,
                RA: 1,
                Z: 3,
                RCODE: 2,
                QDCOUNT: 100,
                ANCOUNT: 200,
                NSCOUNT: 300,
                ARCOUNT: 400,
            },
        ];

        for mut original_header in headers {
            // Serialize the header
            let bytes = original_header
                .create_header_as_array_of_bytes()
                .expect("Failed to serialize header");

            // Parse the bytes into a new header
            let mut parsed_header = Header::default();
            assert!(parsed_header.parse_header(&bytes).is_ok());

            // Assert that the original and parsed headers are identical
            assert_eq!(original_header.ID, parsed_header.ID);
            assert_eq!(original_header.QR, parsed_header.QR);
            assert_eq!(original_header.OPCODE, parsed_header.OPCODE);
            assert_eq!(original_header.AA, parsed_header.AA);
            assert_eq!(original_header.TC, parsed_header.TC);
            assert_eq!(original_header.RD, parsed_header.RD);
            assert_eq!(original_header.RA, parsed_header.RA);
            assert_eq!(original_header.Z, parsed_header.Z);
            assert_eq!(original_header.RCODE, parsed_header.RCODE);
            assert_eq!(original_header.QDCOUNT, parsed_header.QDCOUNT);
            assert_eq!(original_header.ANCOUNT, parsed_header.ANCOUNT);
            assert_eq!(original_header.NSCOUNT, parsed_header.NSCOUNT);
            assert_eq!(original_header.ARCOUNT, parsed_header.ARCOUNT);
        }
    }

    #[test]
    fn test_create_header_with_non_zero_z() {
        // Create a header with Z=3 (non-zero reserved bits)
        let mut header = Header {
            ID: 0x1234,
            QR: 1,
            OPCODE: 0,
            AA: 0,
            TC: 0,
            RD: 0,
            RA: 0,
            Z: 3, // Non-zero reserved bits
            RCODE: 0,
            QDCOUNT: 1,
            ANCOUNT: 0,
            NSCOUNT: 0,
            ARCOUNT: 0,
        };

        // Serialize the header
        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        // Expected byte array
        let expected_bytes: [u8; 12] = [
            0x12,
            0x34,        // ID: 0x1234
            0b1000_0000, // Flags_part1: QR=1, OPCODE=0, AA=0, TC=0, RD=0 => 0x80
            0b0011_0000, // Flags_part2: RA=0, Z=3, RCODE=0 => 0x30
            0x00,
            0x01, // QDCOUNT: 1
            0x00,
            0x00, // ANCOUNT: 0
            0x00,
            0x00, // NSCOUNT: 0
            0x00,
            0x00, // ARCOUNT: 0
        ];

        // Assert that the serialized bytes match the expected bytes
        assert_eq!(bytes, expected_bytes);
    }
}
