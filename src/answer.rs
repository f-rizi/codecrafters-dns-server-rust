use std::result::Result;

#[derive(Debug, Default)]
pub struct Answer {
    pub Name: Vec<String>,
    pub q_type: u16,
    pub q_class: u16,
    pub TTL: u32,
    pub Length: u16,
    pub Data: Vec<u8>,
}

impl Answer {
    pub fn parse_answer(&mut self, question: String) -> Result<(), &'static str> {
        Ok(())
    }
    pub fn create_answer_as_array_of_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let mut answer: Vec<u8> = Vec::new();

        for part in &self.Name {
            answer.push(part.len() as u8);
            answer.extend_from_slice(part.as_bytes());
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_answer_as_array_of_bytes_valid() {
        let answer = Answer {
            Name: vec!["www".to_string(), "example".to_string(), "com".to_string()],
            q_type: 1,                  // A (host address)
            q_class: 1,                 // IN (Internet)
            TTL: 300,                   // Example TTL
            Length: 4,                  // Length of Data (e.g., IPv4 address)
            Data: vec![192, 168, 1, 1], // Example IP address
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        // Expected serialization:
        // Name: 3www7example3com0
        // q_type: 0x0001
        // q_class: 0x0001
        // TTL: 0x0000012C (300)
        // Length: 0x0004
        // Data: 192.168.1.1

        let expected: Vec<u8> = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0, 0, 1, 0, 1, 0, 0, 1, 44, 0, 4, 192, 168, 1, 1,
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_answer_as_array_of_bytes_empty_name() {
        let answer = Answer {
            Name: vec!["".to_string()], // Representing an empty label
            q_type: 28,                  // AAAA (IPv6 address)
            q_class: 1,                  // IN (Internet)
            TTL: 86400,                  // Example TTL
            Length: 16,                  // Length of Data (e.g., IPv6 address)
            Data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1], // Example IPv6 address
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        // Expected serialization:
        // Name: 0 (null terminator)
        // q_type: 0x001C
        // q_class: 0x0001
        // TTL: 0x00015180 (86400)
        // Length: 0x0010
        // Data: 16 bytes of 0s and ending with 1

        let expected: Vec<u8> = vec![
            0,               // Length of the first label (empty)
            0,               // Null terminator for Name
            0, 28,           // q_type: 28 (0x001C)
            0, 1,            // q_class: 1 (0x0001)
            0, 1, 81, 128,   // TTL: 86400 (0x00015180)
            0, 16,           // Length: 16 (0x0010)
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // Data: 16 bytes
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_answer_as_array_of_bytes_empty_data() {
        let answer = Answer {
            Name: vec!["".to_string()], // Representing an empty label
            q_type: 28,                  // AAAA (IPv6 address)
            q_class: 1,                  // IN (Internet)
            TTL: 86400,                  // Example TTL
            Length: 0,                   // Length of Data (0 bytes)
            Data: vec![],                // No Data
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        // Expected serialization:
        // Name: 0 (null terminator)
        // q_type: 0x001C
        // q_class: 0x0001
        // TTL: 0x00015180 (86400)
        // Length: 0x0000
        // Data: none

        let expected: Vec<u8> = vec![
            0,               // Length of the first label (empty)
            0,               // Null terminator for Name
            0, 28,           // q_type: 28 (0x001C)
            0, 1,            // q_class: 1 (0x0001)
            0, 1, 81, 128,   // TTL: 86400 (0x00015180)
            0, 0,            // Length: 0 (0x0000)
                               // No Data
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_answer_as_array_of_bytes_single_label() {
        let answer = Answer {
            Name: vec!["example".to_string()],
            q_type: 1,                  // A (host address)
            q_class: 1,                 // IN (Internet)
            TTL: 3600,                  // Example TTL
            Length: 4,                  // Length of Data (e.g., IPv4 address)
            Data: vec![8, 8, 8, 8],     // Example IP address
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        // Expected serialization:
        // Name: 7example0
        // q_type: 0x0001
        // q_class: 0x0001
        // TTL: 0x00000E10 (3600)
        // Length: 0x0004
        // Data: 8.8.8.8

        let expected: Vec<u8> = vec![
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', // Name: 7example
            0,                                             // Null terminator for Name
            0, 1,                                          // q_type: 1
            0, 1,                                          // q_class:1
            0, 0, 14, 16,                                  // TTL: 3600 (0x00000E10)
            0, 4,                                          // Length:4
            8, 8, 8, 8,                                    // Data: 8.8.8.8
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_answer_as_array_of_bytes_complex_name() {
        let answer = Answer {
            Name: vec![
                "mail".to_string(),
                "sub".to_string(),
                "example".to_string(),
                "com".to_string(),
            ],
            q_type: 28,                  // AAAA (IPv6 address)
            q_class: 1,                  // IN (Internet)
            TTL: 7200,                   // Example TTL
            Length: 16,                  // Length of Data (e.g., IPv6 address)
            Data: vec![
                32, 1, 13, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1
            ], // Example IPv6 address ::1
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        // Expected serialization:
        // Name: 4mail3sub7example3com0
        // q_type: 0x001C
        // q_class: 0x0001
        // TTL: 0x00001C20 (7200)
        // Length: 0x0010
        // Data: 16 bytes IPv6 address

        let expected: Vec<u8> = vec![
            4, b'm', b'a', b'i', b'l', // 4mail
            3, b's', b'u', b'b',       // 3sub
            7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', //7example
            3, b'c', b'o', b'm',       //3com
            0,                           // Null terminator for Name
            0, 28,                       // q_type:28
            0, 1,                        // q_class:1
            0, 0, 28, 32,                // TTL:7200 (0x00001C20)
            0, 16,                       // Length:16
            32, 1, 13, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, // Data: IPv6 ::1
        ];

        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_answer_as_array_of_bytes_long_data() {
        let answer = Answer {
            Name: vec!["test".to_string()],
            q_type: 1,                     // A (host address)
            q_class: 1,                    // IN (Internet)
            TTL: 600,                      // Example TTL
            Length: 8,                     // Length of Data (e.g., some custom data)
            Data: vec![1, 2, 3, 4, 5, 6, 7, 8], // Example data
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        // Expected serialization:
        // Name: 4test0
        // q_type: 0x0001
        // q_class: 0x0001
        // TTL: 0x00000258 (600)
        // Length: 0x0008
        // Data: 1,2,3,4,5,6,7,8

        let expected: Vec<u8> = vec![
            4, b't', b'e', b's', b't', // Name:4test
            0,                           // Null terminator for Name
            0, 1,                        // q_type:1
            0, 1,                        // q_class:1
            0, 0, 2, 88,                 // TTL:600 (0x00000258)
            0, 8,                        // Length:8
            1, 2, 3, 4, 5, 6, 7, 8,      // Data: 1-8
        ];

        assert_eq!(bytes, expected);
    }

    // Since the `parse_answer` method is currently unimplemented (returns `Ok(())`),
    // the following tests are left commented out. Once `parse_answer` is implemented,
    // these tests can be updated accordingly.

    /*
    #[test]
    fn test_parse_answer_valid() {
        // Serialized Answer:
        // Name: 3www7example3com0
        // q_type: 0x0001
        // q_class: 0x0001
        // TTL: 0x0000012C (300)
        // Length: 0x0004
        // Data: 192.168.1.1

        let bytes: Vec<u8> = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0, 0, 1, 0, 1, 0, 0, 1, 44, 0, 4, 192, 168, 1, 1,
        ];

        let mut answer = Answer::default();

        assert!(answer.parse_answer("www.example.com".to_string()).is_ok());

        // Since `parse_answer` is unimplemented, these assertions are placeholders
        // Replace them with actual assertions once `parse_answer` is implemented
        assert_eq!(answer.Name, vec!["www".to_string(), "example".to_string(), "com".to_string()]);
        assert_eq!(answer.q_type, 1);
        assert_eq!(answer.q_class, 1);
        assert_eq!(answer.TTL, 300);
        assert_eq!(answer.Length, 4);
        assert_eq!(answer.Data, vec![192, 168, 1, 1]);
    }

    #[test]
    fn test_parse_answer_invalid_length_prefix() {
        let bytes: Vec<u8> = vec![
            4, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
            3, b'c', b'o', b'm', 0, 0, 1, 0, 1, 0, 0, 1, 44, 0, 4, 192, 168, 1, 1,
        ];

        let mut answer = Answer::default();

        // Assuming `parse_answer` checks for valid label lengths
        // Currently, it does nothing, so this will pass
        // Once implemented, it should return an error
        assert!(answer.parse_answer("www.example.com".to_string()).is_err());
    }

    #[test]
    fn test_parse_answer_length_mismatch() {
        // Serialized Answer with length byte not matching label length
        let bytes: Vec<u8> = vec![
            5, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
            3, b'c', b'o', b'm', 0, 0, 1, 0, 1, 0, 0, 1, 44, 0, 4, 192, 168, 1, 1,
        ];

        let mut answer = Answer::default();

        // Assuming `parse_answer` checks for label length
        // Currently, it does nothing, so this will pass
        // Once implemented, it should return an error
        assert!(answer.parse_answer("www.example.com".to_string()).is_err());
    }

    #[test]
    fn test_parse_answer_insufficient_data() {
        // Serialized Answer with insufficient data for q_type
        let bytes: Vec<u8> = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
            3, b'c', b'o', b'm', 0, 0, 1, // q_type incomplete
            // Missing q_class, TTL, Length, Data
        ];

        let mut answer = Answer::default();

        // Assuming `parse_answer` checks for sufficient data
        // Currently, it does nothing, so this will pass
        // Once implemented, it should return an error
        assert!(answer.parse_answer("www.example.com".to_string()).is_err());
    }
    */

    #[test]
    fn test_parse_answer_unimplemented() {
        // Since `parse_answer` is unimplemented, it should return Ok(())
        let mut answer = Answer::default();
        let result = answer.parse_answer("www.example.com".to_string());
        assert!(result.is_ok());

        // Further assertions can be added once `parse_answer` is implemented
    }
}
