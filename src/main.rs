use std::{
    env::args,
    ffi::OsStr,
    fs::{self, File},
    io::BufWriter,
    path::{Path, PathBuf},
    process::exit,
};

use anyhow::{Result, anyhow};
use std::io::Write;

use crate::{
    command::Command,
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
        println!("Usage: vtranslate <input_path>")
    }

    let input_path = args.nth(1).expect("input path not found");
    println!("Input path: {}", &input_path);

    match fs::exists(&input_path) {
        Ok(exists) => {
            if exists {
                println!("Input path found")
            } else {
                println!("Input path {} not found", &input_path);
                exit(1)
            }
        }
        Err(_) => {
            println!("Error checking path: {}", &input_path);
            exit(1)
        }
    }

    let input_path = Path::new(input_path.as_str());

    match translate(input_path) {
        Ok(_) => {
            println!("Translation completed...")
        }
        Err(e) => {
            println!("Error during translation: {}", e);
        }
    }
}

fn translate(input_path: &Path) -> Result<()> {
    if input_path.is_dir() {
        translate_directory(input_path)
    } else {
        translate_single_file(input_path)
    }
}

fn translate_single_file(input_path: &Path) -> Result<()> {
    let file_stem = input_path.file_stem().unwrap_or(OsStr::new("output"));
    let file_name = file_stem
        .to_os_string()
        .into_string()
        .map_err(|e| anyhow!("Failed to convert OsString to String: {:?}", e))?;

    let input_dir = input_path.parent().unwrap_or(Path::new("."));
    let output_path = input_dir.join(Path::new(file_stem).with_extension("asm"));

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut converter = HackConverter::new(file_name);

    translate_vm_file(input_path, &mut writer, &mut converter)?;
    writer.flush()?;
    Ok(())
}

fn translate_directory(dir_path: &Path) -> Result<()> {
    let dir_stem = dir_path.file_stem().unwrap_or(OsStr::new("output"));
    let dir_name = dir_stem
        .to_os_string()
        .into_string()
        .map_err(|e| anyhow!("Failed to convert OsString to String: {:?}", e))?;

    let output_path = dir_path.join(format!("{}.asm", dir_name));
    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut converter = HackConverter::new(dir_name.clone());

    // Collect and sort .vm files
    let mut vm_files: Vec<PathBuf> = fs::read_dir(dir_path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "vm" {
                Some(path)
            } else {
                None
            }
        })
        .collect();
    vm_files.sort();

    // Bootstrap code: SP = 256, then call Sys.init 0
    writeln!(writer, "// Bootstrap code")?;
    writeln!(writer, "@256")?;
    writeln!(writer, "D=A")?;
    writeln!(writer, "@SP")?;
    writeln!(writer, "M=D")?;
    let bootstrap_call = converter.convert(Command::Call {
        name: "Sys.init".to_string(),
        num_args: 0,
    })?;
    writeln!(writer, "{}", bootstrap_call)?;

    // Translate each .vm file
    for vm_file in &vm_files {
        let file_stem = vm_file.file_stem().unwrap_or(OsStr::new("unknown"));
        let file_name = file_stem
            .to_os_string()
            .into_string()
            .map_err(|e| anyhow!("Failed to convert OsString to String: {:?}", e))?;
        converter.set_file_name(file_name);
        translate_vm_file(vm_file, &mut writer, &mut converter)?;
    }

    writer.flush()?;
    Ok(())
}

fn translate_vm_file(
    input_path: &Path,
    writer: &mut BufWriter<File>,
    converter: &mut HackConverter,
) -> Result<()> {
    let lexer = Lexer::new(input_path)?;
    for result in lexer {
        let lexed_res = result?;
        if lexed_res.skippable {
            continue;
        }
        let command = lexed_res
            .command
            .ok_or_else(|| anyhow!("Command not found"))?;
        let converted = converter.convert(command)?;
        writeln!(writer, "{}", converted)?;
    }
    Ok(())
}
