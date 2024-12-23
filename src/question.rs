// question.rs
use std::result::Result;

#[derive(Debug, Default)]
pub struct Question {
    pub labels: Vec<String>,
    pub q_type: u16,
    pub q_class: u16,
}

impl Question {
    pub fn parse_question(&mut self, question: String) -> Result<(), &'static str> {
        let label_parts: Vec<&str> = question
            .split("\\x")
            .filter(|part| !part.is_empty())
            .collect();

        for part in label_parts {
            if part.len() >= 2 {
                let (length_part, value_part) = part.split_at(2);
                if let Ok(length) = u8::from_str_radix(length_part, 16) {
                    if length as usize != value_part.len() {
                        println!(
                            "Length mismatch: expected {}, got {}",
                            length,
                            value_part.len()
                        );
                        return Err("Length mismatch in question string");
                    }
                    self.labels.push(value_part.to_string());
                } else {
                    println!("Invalid length prefix: {}", length_part);
                    return Err("Invalid length prefix in question string");
                }
            } else {
                println!("Part too short: {}", part);
                return Err("Part too short in question string");
            }
        }

        Ok(())
    }

    pub fn create_question_as_array_of_bytes(&self) -> Result<Vec<u8>, &'static str> {
        let mut bytes: Vec<u8> = Vec::new();

        for part in &self.labels {
            bytes.push(part.len() as u8);
            bytes.extend_from_slice(part.as_bytes());
        }

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
    fn test_parse_question_valid() {
        let mut question = Question::default();

        let input = "\\x03www\\x07example\\x03com\\x00".to_string();
        assert!(question.parse_question(input).is_ok());

        // Validate parsed labels
        assert_eq!(question.labels, vec!["www", "example", "com"]);
    }

    #[test]
    fn test_parse_question_invalid_length_prefix() {
        let mut question = Question::default();

        let input = "\\xZZwww\\x07example\\x03com\\x00".to_string();
        let result = question.parse_question(input);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid length prefix in question string"
        );
    }

    #[test]
    fn test_parse_question_too_short_part() {
        let mut question = Question::default();

        let input = "\\x".to_string();
        let result = question.parse_question(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Part too short in question string");
    }

    #[test]
    fn test_parse_question_length_mismatch() {
        let mut question = Question::default();

        let input = "\\x04www\\x07example\\x03com\\x00".to_string();
        let result = question.parse_question(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Length mismatch in question string");
    }

    #[test]
    fn test_create_question_as_array_of_bytes() {
        let question = Question {
            labels: vec!["www".to_string(), "example".to_string(), "com".to_string()],
            q_type: 1,
            q_class: 1,
        };

        let bytes = question
            .create_question_as_array_of_bytes()
            .expect("Failed to create bytes");

        let expected: Vec<u8> = vec![
            3, b'w', b'w', b'w', 7, b'e', b'x', b'a', b'm', b'p', b'l', b'e', 3, b'c', b'o', b'm',
            0, 0, 1, 0, 1,
        ];

        assert_eq!(bytes, expected);
    }
}
