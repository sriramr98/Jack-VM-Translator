use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use anyhow::Result;

use crate::command::Command;

pub struct Lexer {
    file: BufReader<File>,
}

impl Lexer {
    pub fn new(input_path: &Path) -> Result<Self> {
        let file = File::open(input_path)?;
        Ok(Self {
            file: BufReader::new(file),
        })
    }
}

pub struct LexedResult {
    pub command: Option<Command>,
    // This denotes if converter should skip this entry.. For e.x. comments, empty lines
    pub skippable: bool,
}

impl Iterator for Lexer {
    type Item = Result<LexedResult>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = String::new();
        let res = self.file.read_line(&mut line);

        match res {
            std::result::Result::Ok(size) => {
                if size > 0 {
                    let line = line.trim();

                    if line.is_empty() {
                        return Some(Ok(LexedResult {
                            command: None,
                            skippable: true,
                        }));
                    }

                    if line.starts_with("//") {
                        return Some(Ok(LexedResult {
                            command: None,
                            skippable: true,
                        }));
                    }

                    let command = Command::from(line);

                    let res = command.map(|cmd| LexedResult {
                        command: Some(cmd),
                        skippable: false,
                    });
                    Some(res)
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
