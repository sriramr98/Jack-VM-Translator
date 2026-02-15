use std::{env::args, ffi::OsStr, fs, path::Path, process::exit};

use anyhow::{Result, anyhow};

use crate::lexer::Lexer;

mod command;
mod lexer;

fn main() {
    println!("Starting translation...");
    let mut args = args();

    if args.len() < 2 {
        println!("Usage: vtranslate <input_file_path>")
    }

    let input_file_path = args.nth(1).expect("input file path not found");
    println!("Input file path: {}", &input_file_path);

    match fs::exists(&input_file_path) {
        Ok(exists) => {
            if exists {
                println!("Input file found")
            } else {
                println!("Input file {} not found", &input_file_path);
                exit(1)
            }
        }
        Err(_) => {
            println!("Error checking file: {}", &input_file_path);
            exit(1)
        }
    }

    let input_file_path = Path::new(input_file_path.as_str());
    let output_file_name = input_file_path
        .file_name()
        .unwrap_or(OsStr::new("output.vm"));

    let output_file_path = Path::new(output_file_name);

    match translate(input_file_path, output_file_path) {
        Ok(_) => {
            println!("Translation completed...")
        }
        Err(e) => {
            println!("Error during translation: {}", e);
        }
    }
}

fn translate(input_path: &Path, output_path: &Path) -> Result<()> {
    let lexer = Lexer::new(input_path)?;

    for result in lexer {
        let lexed_res = result?;
        if lexed_res.skippable {
            continue;
        }

        let command = lexed_res
            .command
            .ok_or_else(|| anyhow!("Command not found"))?;

        println!("Command: {}", command)
    }
    Ok(())
}
