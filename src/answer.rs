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
        // TTL: 0x0000012C
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
            Name: vec![],
            q_type: 28,
            q_class: 1,
            TTL: 86400,
            Length: 16,
            Data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        };

        let bytes = answer
            .create_answer_as_array_of_bytes()
            .expect("Failed to create answer bytes");

        let expected: Vec<u8> = vec![
            0, 0, 28, 0, 1, 0, 1, 144, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
        ];

        assert_eq!(bytes, expected);
    }

    // #[test]
    // fn test_parse_answer_valid() {
    //     // Serialized Answer:
    //     // Name: 3www7example3com0
    //     // q_type: 0x0001
    //     // q_class: 0x0001
    //     // TTL: 0x0000012C (300)
    //     // Length: 0x0004
    //     // Data: 192.168.1.1

    //     let bytes: Vec<u8> = vec![
    //         3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
    //         0, 0, 1, 0, 1, 0, 0, 1, 44, 0, 4, 192, 168, 1, 1,
    //     ];

    //     let mut answer = Answer::default();

    //     assert!(answer.parse_answer(&bytes).is_ok());

    //     assert_eq!(
    //         answer.Name,
    //         vec!["www".to_string(), "example".to_string(), "com".to_string()]
    //     );
    //     assert_eq!(answer.q_type, 1);
    //     assert_eq!(answer.q_class, 1);
    //     assert_eq!(answer.TTL, 300);
    //     assert_eq!(answer.Length, 4);
    //     assert_eq!(answer.Data, vec![192, 168, 1, 1]);
    // }

    // #[test]
    // fn test_parse_answer_invalid_length_prefix() {
    //     let bytes: Vec<u8> = vec![
    //         0xZZ, b'w', b'w', b'w',
    //         7, b'e', b'x', b'a', b'm', b'p', b'l', b'e',
    //         3, b'c', b'o', b'm',
    //         0,
    //         0, 1,
    //         0, 1,
    //         0, 0, 1, 44,
    //         0, 4,
    //         192, 168, 1, 1,
    //     ];

    //     let mut answer = Answer::default();

    //     let result = answer.parse_answer(&bytes);

    //     assert!(result.is_err());
    //     assert_eq!(result.unwrap_err(), "Invalid UTF-8 in label");
    // }

    // #[test]
    // fn test_parse_answer_length_mismatch() {
    //     // Serialized Answer with length byte not matching label length
    //     // let bytes: Vec<u8> = vec![
    //     //     4, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
    //     //     0, 0, 1, 0, 1, 0, 0, 1, 44, 0, 4, 192, 168, 1, 1,
    //     // ];

    //     // let mut answer = Answer::default();

    //     // let result = answer.parse_answer(&bytes);

    //     // assert!(result.is_err());
    //     // assert_eq!(result.unwrap_err(), "Length mismatch in question string");
    // }

    // #[test]
    // fn test_parse_answer_insufficient_data() {
    //     // Serialized Answer with insufficient data for q_type
    //     // let bytes: Vec<u8> = vec![
    //     //     3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
    //     //     0, 0, 1, // q_type
    //     //        // Missing q_class, TTL, Length, Data
    //     // ];

    //     // let mut answer = Answer::default();

    //     // let result = answer.parse_answer(&bytes);

    //     // assert!(result.is_err());
    //     // assert_eq!(result.unwrap_err(), "Insufficient data for q_class and TTL");
    // }

    #[test]
    fn test_create_answer_as_array_of_bytes_empty_data() {
        let answer = Answer {
            Name: vec!["".to_string()],
            q_type: 28,
            q_class: 1,
            TTL: 86400,
            Length: 0,
            Data: vec![],
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

        let expected: Vec<u8> = vec![0, 0, 28, 0, 1, 0, 1, 81, 128, 0, 0];

        assert_eq!(bytes, expected);
    }
}
