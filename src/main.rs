mod parser;

use colored::Colorize;
use core::panic;
use parser::tokenize_text;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::windows::fs::FileExt;
use std::path::{Path, PathBuf};

//TODO: delete
fn read_and_chunk_file(path: PathBuf) {
    let display: std::path::Display<'_> = path.display();

    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let metadata: std::fs::Metadata = file.metadata().unwrap();
    let size: u64 = metadata.len();

    const SIZE_OF_CHUNK: u64 = 100;

    let mut buf: [u8; SIZE_OF_CHUNK as usize] = [0; SIZE_OF_CHUNK as usize];
    let num_chunks: u64 = size.div_ceil(SIZE_OF_CHUNK).try_into().unwrap();

    let mut bytes_read: usize;
    let mut total_read: usize = 0;

    for _i in 0..num_chunks {
        bytes_read = file.seek_read(&mut buf, total_read as u64).unwrap();
        total_read += bytes_read;
        println!(
            "{} {}/{} {}",
            "Reading".green(),
            total_read.to_string().bold().green(),
            size.to_string().bold().green(),
            "bytes:".green()
        );

        println!("\n{}", String::from_utf8_lossy(&buf));
    }
}

fn read_lines_file(path: &PathBuf) -> Result<impl Iterator<Item = String>, ()> {
    let file: File = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file);
    Ok(reader.lines().filter_map(Result::ok))
}

fn main() {
    let mut working_path = env::current_dir().unwrap();
    working_path.push(Path::new("input\\in.md"));

    let str_iter = match read_lines_file(&working_path) {
        Err(_) => panic!("Error: Could not read lines of file"),
        Ok(str_iter) => str_iter,
    };

    let tokenized_text_lines = tokenize_text(str_iter);
    for line in tokenized_text_lines {
        println!("{}", line);
    }
}
