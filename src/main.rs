mod parser;
mod simple_log;

use colored::Colorize;
use core::panic;
use parser::tokenize_text;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;
use std::sync::OnceLock;
use std::{env, fs};

// Use OnceCell if you want:
// A global value
// Assigned at runtime
// Immutable after assignment (like const)
static INPUT_FILE_PATH: std::sync::OnceLock<String> = OnceLock::new();
static OUTPUT_FILE_PATH: std::sync::OnceLock<String> = OnceLock::new();
const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");

fn read_lines_file(path: &PathBuf) -> Result<impl Iterator<Item = String>, ()> {
    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok))
}

fn write_result(html_lines: Vec<String>) {
    let mut working_path: PathBuf;
    if OUTPUT_FILE_PATH.get().is_none() {
        working_path = env::current_dir().unwrap();
        let folder_name = String::from("output");
        working_path.push(folder_name);
    } else {
        match PathBuf::from_str(OUTPUT_FILE_PATH.get().unwrap()) {
            Ok(p) => working_path = p,
            Err(e) => panic!("Error: {}", e),
        }
    }

    log!(info, "Creating folders for {}", working_path.display());

    // Create folder path
    if let Err(err) = fs::create_dir_all(&(working_path.clone())) {
        panic!(
            "Error: Could not create output directory {}\nReason:{}",
            working_path.display(),
            err
        );
    }

    // Create the file
    let filename_out: String = String::from("out.html");
    working_path.push(filename_out);

    log!(info, "Writing file {}", working_path.display());

    let mut file = match File::create(&working_path) {
        Err(err) => panic!(
            "Error: Could not create file {}\nReason:{}",
            working_path.display(),
            err
        ),
        Ok(file) => file,
    };

    // Write inside the file
    for mut line in html_lines {
        line.push('\n');
        let bytes_written = match file.write(line.as_bytes()) {
            Err(_) => panic!(
                "Error: Could not write bytes to file {}",
                working_path.display()
            ),
            Ok(bytes_written) => bytes_written,
        };

        println!("Written {}bytes", bytes_written);
    }
}

fn main() {
    process_args();
    let mut working_path;
    let path_str: String;
    if INPUT_FILE_PATH.get().is_none() {
        working_path = env::current_dir().unwrap();
        path_str = format!("input{}in.md", std::path::MAIN_SEPARATOR_STR).to_string();
        working_path.push(Path::new(&path_str));
    } else {
        match PathBuf::from_str(INPUT_FILE_PATH.get().unwrap()) {
            Ok(p) => working_path = p,
            Err(e) => panic!("Error: {}", e),
        }
    }

    println!("Starting conversion of {}", working_path.display());

    let str_iter = match read_lines_file(&working_path) {
        Err(_) => panic!("Error: Could not read lines of file"),
        Ok(str_iter) => str_iter,
    };

    let tokenized_text_lines = tokenize_text(str_iter);
    write_result(tokenized_text_lines);

    // for line in tokenized_text_lines {
    //     println!("{}", line);
    // }
}

fn process_args() {
    let mut args: env::Args = env::args();
    while let Some(curr) = args.next() {
        // Define here your CLI commands
        match curr.as_str() {
            "--input" => match args.next() {
                Some(param) => match INPUT_FILE_PATH.set(param) {
                    Err(e) => panic!("Error: {}", e),
                    _ => (),
                },
                _ => (),
            },
            "--output" => match args.next() {
                Some(param) => match OUTPUT_FILE_PATH.set(param) {
                    Err(e) => panic!("Error: {}", e),
                    _ => (),
                },
                _ => (),
            },
            "--version" => {
                println!("v{}", SOFTWARE_VERSION);
                exit(0);
            }
            "--help" => {
                print_help();
                exit(0);
            }
            _ => (),
        }
    }
}

fn print_help() {
    const HELP_MESSAGE: &str = r#"
    Markdown to HTML Converter

    USAGE:
        markdown-converter --input <FILE> --output <FILE>

    OPTIONS:
        --input <FILE>       Absolute path to the input Markdown file
        --output <FILE>      Absolute path to the output HTML file
        --help               Show this help message and exit
        --version            Show version information and exit
    "#;
    println!("{}", HELP_MESSAGE);
}
