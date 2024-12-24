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
    fn test_parse_question_valid_single_label() {
        let mut question = Question::default();
        let input = "03abc".to_string(); // length_part = "03", value_part = "abc", len == 3
        let result = question.parse_question(input);

        assert!(result.is_ok());
        assert_eq!(question.labels.len(), 1);
        assert_eq!(question.labels[0], "abc");
    }

    #[test]
    fn test_parse_question_valid_multiple_labels() {
        let mut question = Question::default();
        // Split on `\x`: ["03abc", "05hello"]
        let input = "03abc\\x05hello".to_string();
        let result = question.parse_question(input);

        assert!(result.is_ok());
        assert_eq!(question.labels.len(), 2);
        assert_eq!(question.labels[0], "abc");
        assert_eq!(question.labels[1], "hello");
    }

    #[test]
    fn test_parse_question_length_mismatch() {
        let mut question = Question::default();
        // "05hell" => length_part = "05", value_part = "hell", actual length is 4 but expected 5
        let input = "03abc\\x05hell".to_string();
        let result = question.parse_question(input);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_question_invalid_hex_prefix() {
        let mut question = Question::default();
        // "0gabc" => "0g" is not valid hex
        let input = "0gabc".to_string();
        let result = question.parse_question(input);

        assert!(result.is_err());
        assert!(
            question.labels.is_empty(),
            "Labels should not be populated on error"
        );
    }

    #[test]
    fn test_parse_question_part_too_short() {
        let mut question = Question::default();
        // The part after splitting on `\x` is just "0", which is < 2 characters
        let input = "0\\x".to_string();
        let result = question.parse_question(input);

        assert!(result.is_err());
        assert!(
            question.labels.is_empty(),
            "Labels should not be populated on error"
        );
    }

    #[test]
    fn test_create_question_as_array_of_bytes_single_label() {
        let mut question = Question::default();
        question.labels = vec!["abc".to_string()];
        question.q_type = 0x0001;
        question.q_class = 0x0001;

        let result = question.create_question_as_array_of_bytes();
        assert!(result.is_ok());
        let bytes = result.unwrap();

        // Breakdown of expected bytes:
        // - Label "abc": length = 3, then 'a', 'b', 'c'
        // - Null terminator (0)
        // - q_type (0x0001 big-endian -> 00 01)
        // - q_class (0x0001 big-endian -> 00 01)
        // => [3, b'a', b'b', b'c', 0, 0, 1, 0, 1]
        let expected = vec![3, b'a', b'b', b'c', 0, 0, 1, 0, 1];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_question_as_array_of_bytes_multiple_labels() {
        let mut question = Question::default();
        question.labels = vec!["abc".to_string(), "hello".to_string()];
        question.q_type = 1;
        question.q_class = 1;

        let result = question.create_question_as_array_of_bytes();
        assert!(result.is_ok());
        let bytes = result.unwrap();

        // Breakdown of expected bytes:
        // - Label "abc": length = 3, then "abc"
        // - Label "hello": length = 5, then "hello"
        // - Null terminator (0)
        // - q_type = 1 (big-endian -> [0, 1])
        // - q_class = 1 (big-endian -> [0, 1])
        // => [3, b'a', b'b', b'c', 5, b'h', b'e', b'l', b'l', b'o', 0, 0, 1, 0, 1]
        let expected = vec![
            3, b'a', b'b', b'c', 5, b'h', b'e', b'l', b'l', b'o', 0, 0, 1, 0, 1,
        ];
        assert_eq!(bytes, expected);
    }

    #[test]
    fn test_create_question_as_array_of_bytes_empty_labels() {
        let question = Question {
            labels: vec![],
            q_type: 0x000F,  // for example
            q_class: 0x000F, // for example
        };

        let result = question.create_question_as_array_of_bytes();
        assert!(result.is_ok());
        let bytes = result.unwrap();

        // Breakdown of expected bytes with no labels:
        // - Null terminator (0)
        // - q_type = 0x000F => [0, 0x0F]
        // - q_class = 0x000F => [0, 0x0F]
        // => [0, 0, 15, 0, 15]
        let expected = vec![0, 0, 15, 0, 15];
        assert_eq!(bytes, expected);
    }
}
