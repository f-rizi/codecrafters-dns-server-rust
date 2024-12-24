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
        let mut header = Header::default();

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

    // Additional Tests

    #[test]
    fn test_parse_header_flags() {
        // Construct header bytes with specific flags set
        // Example:
        // QR = 1, OPCODE = 2, AA = 1, TC = 0, RD = 1
        // RA = 0, Z = 5, RCODE = 3
        // Binary for flags_part1 (byte 2): QR=1, OPCODE=010 (2), AA=1, TC=0, RD=1 => 1 010 1 0 1 => 1010 1010 => 0xAA
        // Binary for flags_part2 (byte 3): RA=0, Z=101 (5), RCODE=0011 (3) => 0 101 0011 => 0101 0011 => 0x53
        let bytes: [u8; 12] = [
            0xAB,
            0xCD,        // ID
            0b1010_1010, // flags_part1: QR=1, OPCODE=010 (2), AA=1, TC=0, RD=1
            0b0101_0011, // flags_part2: RA=0, Z=5, RCODE=3
            0x00,
            0x01, // QDCOUNT
            0x00,
            0x02, // ANCOUNT
            0x00,
            0x03, // NSCOUNT
            0x00,
            0x04, // ARCOUNT
        ];
        let mut header = Header::default();

        assert!(header.parse_header(&bytes).is_ok());
        assert_eq!(header.ID, 0xABCD);
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
    fn test_create_header_flags() {
        // debugging
        let mut header = Header::default();
        header.ID = 0xABCD;
        header.QR = 1;
        header.OPCODE = 2;
        header.AA = 1;
        header.TC = 0;
        header.RD = 1;
        header.RA = 0;
        header.Z = 5;
        header.RCODE = 3;
        header.QDCOUNT = 1;
        header.ANCOUNT = 2;
        header.NSCOUNT = 3;
        header.ARCOUNT = 4;

        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to serialize header");

        assert_eq!(
            bytes,
            [
                0xAB,
                0xCD,        // ID
                0b1001_0101, // flags_part1
                0b0101_0011, // flags_part2
                0x00,
                0x01, // QDCOUNT
                0x00,
                0x02, // ANCOUNT
                0x00,
                0x03, // NSCOUNT
                0x00,
                0x04, // ARCOUNT
            ]
        );
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
    fn test_create_header_with_max_values() {
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

        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        assert_eq!(
            bytes,
            [
                0xFF,
                0xFF,        // ID
                0b1111_1111, // flags_part1: QR=1, OPCODE=1111 (15), AA=1, TC=1, RD=1 => 0xFF
                0b1111_1111, // flags_part2: RA=1, Z=111 (7), RCODE=1111 (15) => 0xFF
                0xFF,
                0xFF, // QDCOUNT
                0xFF,
                0xFF, // ANCOUNT
                0xFF,
                0xFF, // NSCOUNT
                0xFF,
                0xFF, // ARCOUNT
            ]
        );
    }

    #[test]
    fn test_create_header_with_min_values() {
        let mut header = Header::default(); // All fields are set to 0

        // Manually set fields to their minimum values where applicable
        header.QR = 0;
        header.OPCODE = 0;
        header.AA = 0;
        header.TC = 0;
        header.RD = 0;
        header.RA = 0;
        header.Z = 0;
        header.RCODE = 0;
        header.QDCOUNT = 0;
        header.ANCOUNT = 0;
        header.NSCOUNT = 0;
        header.ARCOUNT = 0;

        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        assert_eq!(
            bytes,
            [
                0x00,
                0x00,        // ID
                0b0000_0000, // All flags are 0
                0b0000_0000, // All flags are 0
                0x00,
                0x00, // QDCOUNT
                0x00,
                0x00, // ANCOUNT
                0x00,
                0x00, // NSCOUNT
                0x00,
                0x00, // ARCOUNT
            ]
        );
    }

    #[test]
    fn test_parse_header_with_invalid_z_bits() {
        // Example header with Z=8 (invalid, since Z is supposed to be 3 bits and max is 7)
        let bytes: [u8; 12] = [
            0x00,
            0x01,        // ID
            0b0000_0000, // QR=0, OPCODE=0, AA=0, TC=0, RD=0
            0b1000_0000, // RA=1, Z=8 (invalid if Z is supposed to be <=7), RCODE=0
            0x00,
            0x01, // QDCOUNT
            0x00,
            0x00, // ANCOUNT
            0x00,
            0x00, // NSCOUNT
            0x00,
            0x00, // ARCOUNT
        ];
        let mut header = Header::default();

        // Currently, the parser does not enforce Z's valid range
        // Depending on requirements, you might want to add a validation step
        // For now, it will parse Z as 8 without error
        assert!(header.parse_header(&bytes).is_ok());
        assert_eq!(header.Z, 8);
    }

    #[test]
    fn test_parse_header_with_random_data() {
        let bytes: [u8; 12] = [
            0xDE,
            0xAD,        // ID
            0b0110_1010, // QR=0, OPCODE=110 (6), AA=1, TC=0, RD=1
            0b1010_0101, // RA=1, Z=010 (2), RCODE=5
            0x12,
            0x34, // QDCOUNT
            0x56,
            0x78, // ANCOUNT
            0x9A,
            0xBC, // NSCOUNT
            0xDE,
            0xF0, // ARCOUNT
        ];
        let mut header = Header::default();

        assert!(header.parse_header(&bytes).is_ok());
        assert_eq!(header.ID, 0xDEAD);
        assert_eq!(header.QR, 0);
        assert_eq!(header.OPCODE, 6);
        assert_eq!(header.AA, 1);
        assert_eq!(header.TC, 0);
        assert_eq!(header.RD, 1);
        assert_eq!(header.RA, 1);
        assert_eq!(header.Z, 2);
        assert_eq!(header.RCODE, 5);
        assert_eq!(header.QDCOUNT, 0x1234);
        assert_eq!(header.ANCOUNT, 0x5678);
        assert_eq!(header.NSCOUNT, 0x9ABC);
        assert_eq!(header.ARCOUNT, 0xDEF0);
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
                OPCODE: 15,
                AA: 1,
                TC: 1,
                RD: 1,
                RA: 1,
                Z: 7,
                RCODE: 15,
                QDCOUNT: 65535,
                ANCOUNT: 65535,
                NSCOUNT: 65535,
                ARCOUNT: 65535,
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
    fn test_parse_header_with_excess_data() {
        let bytes: Vec<u8> = vec![
            0x00,
            0x01,        // ID
            0b1000_0000, // QR=1, OPCODE=0000 (0), AA=0, TC=0, RD=0
            0b0000_0000, // RA=0, Z=0, RCODE=0
            0x00,
            0x01, // QDCOUNT
            0x00,
            0x00, // ANCOUNT
            0x00,
            0x00, // NSCOUNT
            0x00,
            0x00, // ARCOUNT
            0xFF,
            0xFF, // Excess data
            0xAA,
            0xAA, // Excess data
        ];

        let mut header = Header::default();
        // The parser should only consider the first 12 bytes
        assert!(header.parse_header(&bytes).is_ok());

        assert_eq!(header.ID, 0x0001);
        assert_eq!(header.QR, 1);
        assert_eq!(header.OPCODE, 0);
        assert_eq!(header.AA, 0);
        assert_eq!(header.TC, 0);
        assert_eq!(header.RD, 0);
        assert_eq!(header.RA, 0);
        assert_eq!(header.Z, 0);
        assert_eq!(header.RCODE, 0);
        assert_eq!(header.QDCOUNT, 1);
        assert_eq!(header.ANCOUNT, 0);
        assert_eq!(header.NSCOUNT, 0);
        assert_eq!(header.ARCOUNT, 0);
    }

    #[test]
    fn test_create_header_with_non_zero_z() {
        let mut header = Header::default();
        header.ID = 0x1234;
        header.QR = 1;
        header.OPCODE = 0;
        header.AA = 0;
        header.TC = 0;
        header.RD = 0;
        header.RA = 0;
        header.Z = 3; // Non-zero reserved bits
        header.RCODE = 0;
        header.QDCOUNT = 1;
        header.ANCOUNT = 0;
        header.NSCOUNT = 0;
        header.ARCOUNT = 0;

        let bytes = header
            .create_header_as_array_of_bytes()
            .expect("Failed to create byte array");

        // flags_part1 = QR=1, OPCODE=0000, AA=0, TC=0, RD=0 => 1000_0000 => 0x80
        // flags_part2 = RA=0, Z=011, RCODE=0000 => 0011_0000 => 0x30
        assert_eq!(
            bytes,
            [
                0x12,
                0x34,        // ID
                0b1000_0000, // flags_part1
                0b0011_0000, // flags_part2
                0x00,
                0x01, // QDCOUNT
                0x00,
                0x00, // ANCOUNT
                0x00,
                0x00, // NSCOUNT
                0x00,
                0x00, // ARCOUNT
            ]
        );
    }
}
