mod parser;
mod simple_log;

use colored::Colorize;
use core::panic;
use parser::tokenize_text;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::{env, fs};

fn read_lines_file(path: &PathBuf) -> Result<impl Iterator<Item = String>, ()> {
    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok))
}

fn write_result(html_lines: Vec<String>) {
    let mut working_path = env::current_dir().unwrap();
    let folder_name = String::from("output");
    working_path.push(folder_name);
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
    let mut working_path = env::current_dir().unwrap();
    let path_str = format!("input{}in.md", std::path::MAIN_SEPARATOR_STR).to_string();
    working_path.push(Path::new(&path_str));

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
