use std::str::FromStr;

use strum::{Display, EnumString};

#[derive(EnumString)]
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

#[derive(Display)]
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
