use std::{
    env::args,
    ffi::OsStr,
    fs::{self, File},
    io::BufWriter,
    path::Path,
    process::exit,
};

use anyhow::{Result, anyhow};
use std::io::Write;

use crate::{
    converter::{Converter, HackConverter},
    lexer::Lexer,
};

mod command;
mod converter;
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

    match translate(input_file_path) {
        Ok(_) => {
            println!("Translation completed...")
        }
        Err(e) => {
            println!("Error during translation: {}", e);
        }
    }
}

fn translate(input_path: &Path) -> Result<()> {
    let input_file_stem = input_path.file_stem().unwrap_or(OsStr::new("output"));
    let input_file_name = input_file_stem
        .to_os_string()
        .into_string()
        .map_err(|e| anyhow!("Failed to convert OsString to String: {:?}", e))?;

    let input_dir = input_path.parent().unwrap_or(Path::new("."));
    let output_path = input_dir.join(Path::new(input_file_stem).with_extension("asm"));

    let lexer = Lexer::new(input_path)?;
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut converter = HackConverter::new(input_file_name);

    for result in lexer {
        let lexed_res = result?;
        if lexed_res.skippable {
            continue;
        }

        let command = lexed_res
            .command
            .ok_or_else(|| anyhow!("Command not found"))?;
        let converted = converter.convert(command)?;
        writeln!(writer, "{}", converted)?
    }

    writer.flush()?;
    Ok(())
}
