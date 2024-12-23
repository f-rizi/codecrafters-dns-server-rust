use std::result::Result::Ok;

#[derive(Debug, Default)]
pub struct Question {
    pub labels: Vec<String>,
    pub q_type: u16,
    pub q_class: u16,
}

impl Question {
    pub fn parse_questin(&mut self, question: String) -> Result<(), &'static str> {
        let label_parts: Vec<&str> = question
            .split("\\x")
            .filter(|part| !part.is_empty())
            .collect();

        for part in label_parts {
            if part.len() >= 2 {
                let (length_part, value_part) = part.split_at(2).into();
                if let Ok(_) = u8::from_str_radix(length_part, 16) {
                    self.labels.push(value_part.to_string());
                } else {
                    println!("Invalid length prefix: {}", length_part);
                    return Err("err");
                }
            } else {
                println!("Part too short: {}", part);
                return Err("err");
            }
        }

        Ok(())
    }

    pub fn create_quetion_as_array_of_bytes(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut bytes: Vec<u8> = Vec::new();

        for part in &self.labels {
            let temp1 = format!("\\x{}{}", part.len(), part);
            bytes.extend_from_slice(temp1.as_bytes());
        }

        bytes.push(0);

        bytes.extend_from_slice(&self.q_type.to_be_bytes());
        bytes.extend_from_slice(&self.q_class.to_be_bytes());

        return Ok(bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_question_valid() {
        let mut question = Question::default();

        let input = "\\x03www\\x07example\\x03com\\x00".to_string();
        assert!(question.parse_questin(input).is_ok());

        // Validate parsed labels
        assert_eq!(question.labels, vec!["www", "example", "com"]);
    }

    #[test]
    fn test_parse_question_invalid_length_prefix() {
        let mut question = Question::default();

        let input = "\\xZZwww\\x07example\\x03com\\x00".to_string();
        let result = question.parse_questin(input);

        // Expect an error due to invalid length prefix
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "err");
    }

    #[test]
    fn test_parse_question_too_short_part() {
        let mut question = Question::default();

        let input = "\\x".to_string(); // Too short to have a valid length
        let result = question.parse_questin(input);

        // Expect an error due to insufficient part length
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "err");
    }

    #[test]
    fn test_create_question_as_array_of_bytes() {
        let mut question = Question {
            labels: vec!["www".to_string(), "example".to_string(), "com".to_string()],
            q_type: 1,  
            q_class: 1,
        };

        let bytes = question
            .create_quetion_as_array_of_bytes()
            .expect("Failed to create bytes");

        // Expected output:
        // Labels: [3, 'w', 'w', 'w', 7, 'e', 'x', 'a', 'm', 'p', 'l', 'e', 3, 'c', 'o', 'm']
        // Null byte: [0]
        // q_type (0x0001) and q_class (0x0001): [0, 1, 0, 1]
        let expected: Vec<u8> = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0, 0, 1, 0, 1,
        ];

        assert_eq!(bytes, expected);
    }
}
