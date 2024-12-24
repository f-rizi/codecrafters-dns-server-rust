use std::result::Result;

#[derive(Debug, Default)]
pub struct Question {
    pub name: Vec<u8>, // Vector for storing the domain name
    pub q_type: u16,   // Query type (e.g., A)
    pub q_class: u16,  // Query class (e.g., IN)
}

impl Question {
    pub fn parse_questions(&mut self, bytes: &Vec<u8>, offset: usize) -> Result<(), String> {
        if bytes.len() == 0 {
            return Err("Header size is zero, can not get a question".to_string());
        }

        let mut index: usize = offset;

        while bytes[index] != 0 {
            if bytes[index] < 192 {
                let temp: &[u8] = &bytes[index..index + bytes[index] as usize];
                self.name.extend_from_slice(temp);
                index = index + bytes[index] as usize;
            } else {
                let temp = self.get_offset(bytes, index);
                index = index + temp.len();
                self.name.extend(temp);
            }
        }

        self.q_type = u16::from_be_bytes([bytes[index], bytes[index + 1]]);
        self.q_class = u16::from_be_bytes([bytes[index + 2], bytes[index + 3]]);

        Ok(())
    }

    fn get_offset(&mut self, bytes: &Vec<u8>, start: usize) -> Vec<u8> {
        let mut end = start;

        while bytes[end] != 0 {
            end += 1
        }

        return bytes[start..end].to_vec();
    }

    pub fn create_question_as_array_of_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.extend_from_slice(&self.name);

        bytes.push(0);

        bytes.extend_from_slice(&self.q_type.to_be_bytes());
        bytes.extend_from_slice(&self.q_class.to_be_bytes());

        Ok(bytes)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_questions() {
        // Example DNS question:
        // Name: "www.example.com."
        // Type: A (0x0001)
        // Class: IN (0x0001)
        let bytes: Vec<u8> = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0, 1, 0, 1,
        ];
        let mut question = Question::default();

        // Call parse_questions with offset 0
        assert!(question.parse_questions(&bytes, 0).is_ok());

        // Verify the parsed question
        assert_eq!(
            question.name,
            vec![3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm']
        ); // Name in raw DNS format
        assert_eq!(question.q_type, 1); // Type A
        assert_eq!(question.q_class, 1); // Class IN
    }

    #[test]
    fn test_parse_questions_with_compression() {
        // Example DNS questions with compression:
        // Question 1: Name: "www.example.com."
        // Question 2: Name: "api.example.com." using compression pointer
        let bytes: Vec<u8> = vec![
            // Question 1
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm', 0,
            0, 1, 0, 1,
            // Question 2 (compressed)
            3, b'a', b'p', b'i', 192, 0, 0, 1, 0, 1,
        ];

        // Parse the first question
        let mut question1 = Question::default();
        assert!(question1.parse_questions(&bytes, 0).is_ok());
        assert_eq!(
            question1.name,
            vec![3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm']
        );
        assert_eq!(question1.q_type, 1); // Type A
        assert_eq!(question1.q_class, 1); // Class IN

        // Parse the second question
        let mut question2 = Question::default();
        assert!(question2.parse_questions(&bytes, 17).is_ok());
        assert_eq!(
            question2.name,
            vec![3, b'a', b'p', b'i', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm']
        );
        assert_eq!(question2.q_type, 1); // Type A
        assert_eq!(question2.q_class, 1); // Class IN
    }

    // #[test]
    // fn test_parse_questions_invalid_offset() {
    //     let bytes: Vec<u8> = vec![3, b'w', b'w', b'w', 0, 0, 1, 0, 1]; // Shortened example
    //     let mut question = Question::default();

    //     // Provide an invalid offset
    //     assert!(question.parse_questions(&bytes, 100).is_err()); // Offset out of bounds
    // }
}
