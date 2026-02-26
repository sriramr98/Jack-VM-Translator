use std::str::FromStr;

use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, EnumString, PartialEq, Eq, Display, Hash)]
#[strum(serialize_all = "lowercase")]
pub enum Segment {
    Argument,
    Local,
    Static,
    Constant,
    This,
    That,
    Pointer,
    Temp,
}

#[derive(Debug, Display, PartialEq, Eq, Hash)]
pub enum Command {
    Push { segment: Segment, index: u16 },
    Pop { segment: Segment, index: u16 },
    Add,
    Sub,
    Neg,
    Eq,
    Gt,
    Lt,
    And,
    Or,
    Not,
}

impl Command {
    pub fn from(input: &str) -> anyhow::Result<Self> {
        let mut parts = input.split_ascii_whitespace();
        let command_str = parts.next().ok_or_else(|| anyhow::anyhow!("Empty line"))?;

        match command_str {
            "push" => {
                let segment_str = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("push missing segment"))?;
                let index_str = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("push missing index"))?;
                let segment = Segment::from_str(segment_str)
                    .map_err(|_| anyhow::anyhow!("Unknown segment: {}", segment_str))?;
                let index = index_str
                    .parse::<u16>()
                    .map_err(|_| anyhow::anyhow!("Invalid index: {}", index_str))?;
                Ok(Command::Push { segment, index })
            }
            "pop" => {
                let segment_str = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("pop missing segment"))?;
                let index_str = parts
                    .next()
                    .ok_or_else(|| anyhow::anyhow!("pop missing index"))?;
                let segment = Segment::from_str(segment_str)
                    .map_err(|_| anyhow::anyhow!("Unknown segment: {}", segment_str))?;
                let index = index_str
                    .parse::<u16>()
                    .map_err(|_| anyhow::anyhow!("Invalid index: {}", index_str))?;
                Ok(Command::Pop { segment, index })
            }
            "add" => Ok(Command::Add),
            "sub" => Ok(Command::Sub),
            "neg" => Ok(Command::Neg),
            "eq" => Ok(Command::Eq),
            "gt" => Ok(Command::Gt),
            "lt" => Ok(Command::Lt),
            "and" => Ok(Command::And),
            "or" => Ok(Command::Or),
            "not" => Ok(Command::Not),
            _ => Err(anyhow::anyhow!("Unknown command: {}", command_str)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_push_commands() {
        let test_cases = [
            ("push argument 2", Segment::Argument, 2),
            ("push local 0", Segment::Local, 0),
            ("push static 5", Segment::Static, 5),
            ("push constant 10", Segment::Constant, 10),
            ("push this 3", Segment::This, 3),
            ("push that 7", Segment::That, 7),
            ("push pointer 1", Segment::Pointer, 1),
            ("push temp 6", Segment::Temp, 6),
        ];

        for (input, expected_segment, expected_index) in test_cases {
            let result = Command::from(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            assert_eq!(
                result.unwrap(),
                Command::Push {
                    segment: expected_segment,
                    index: expected_index
                },
                "Mismatch for: {}",
                input
            );
        }
    }

    #[test]
    fn test_parse_pop_commands() {
        let test_cases = [
            ("pop argument 2", Segment::Argument, 2),
            ("pop local 0", Segment::Local, 0),
            ("pop static 8", Segment::Static, 8),
            ("pop this 4", Segment::This, 4),
            ("pop that 9", Segment::That, 9),
            ("pop pointer 0", Segment::Pointer, 0),
            ("pop temp 3", Segment::Temp, 3),
        ];

        for (input, expected_segment, expected_index) in test_cases {
            let result = Command::from(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            assert_eq!(
                result.unwrap(),
                Command::Pop {
                    segment: expected_segment,
                    index: expected_index
                },
                "Mismatch for: {}",
                input
            );
        }
    }

    #[test]
    fn test_parse_arithmetic_commands() {
        let test_cases = [
            ("add", Command::Add),
            ("sub", Command::Sub),
            ("neg", Command::Neg),
            ("eq", Command::Eq),
            ("gt", Command::Gt),
            ("lt", Command::Lt),
            ("and", Command::And),
            ("or", Command::Or),
            ("not", Command::Not),
        ];

        for (input, expected) in test_cases {
            let result = Command::from(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            assert_eq!(result.unwrap(), expected, "Mismatch for: {}", input);
        }
    }

    #[test]
    fn test_parse_errors() {
        let test_cases = [
            ("", "Empty line"),
            ("   ", "Empty line"),
            ("invalid", "Unknown command"),
            ("push", "push missing segment"),
            ("push local", "push missing index"),
            ("push invalid 5", "Unknown segment"),
            ("push local abc", "Invalid index"),
            ("push local -1", "Invalid index"),
            ("pop", "pop missing segment"),
            ("pop argument", "pop missing index"),
            ("pop unknown 3", "Unknown segment"),
            ("pop local xyz", "Invalid index"),
            ("push constant 65536", "Invalid index"),
        ];

        for (input, expected_error) in test_cases {
            let result = Command::from(input);
            assert!(result.is_err(), "Expected error for: {}", input);
            assert!(
                result.unwrap_err().to_string().contains(expected_error),
                "Wrong error message for: {}",
                input
            );
        }
    }

    #[test]
    fn test_parse_whitespace_handling() {
        let test_cases = [
            ("  push   local   5  ", Segment::Local, 5),
            ("push\tlocal\t5", Segment::Local, 5),
            ("  add  ", Segment::Local, 0), // Will verify command type separately
        ];

        // Test push commands with various whitespace
        for (input, expected_segment, expected_index) in &test_cases[..2] {
            let result = Command::from(input);
            assert!(result.is_ok(), "Failed to parse: {}", input);
            assert_eq!(
                result.unwrap(),
                Command::Push {
                    segment: *expected_segment,
                    index: *expected_index
                },
                "Mismatch for: {}",
                input
            );
        }

        // Test arithmetic command with whitespace
        let result = Command::from("  add  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Command::Add);
    }

    #[test]
    fn test_parse_boundary_values() {
        // Test max u16 value
        let result = Command::from("push constant 65535");
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            Command::Push {
                segment: Segment::Constant,
                index: 65535
            }
        );

        // Test overflow
        let result = Command::from("push constant 65536");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid index"));
    }
}
