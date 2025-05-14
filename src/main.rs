mod parser;

use colored::Colorize;
use core::panic;
use parser::tokenize_text;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, Write};
use std::path::{Path, PathBuf};

//TODO: delete
fn read_and_chunk_file(path: PathBuf) {
    let display: std::path::Display<'_> = path.display();

    let mut file: File = match File::open(&path) {
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
        file.seek(std::io::SeekFrom::Start(total_read as u64))
            .unwrap();
        bytes_read = file.read(&mut buf).unwrap();
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

fn write_result(html_lines: Vec<String>) {
    let mut working_path = env::current_dir().unwrap();
    let path_str = format!("output{}out.html", std::path::MAIN_SEPARATOR_STR).to_string();
    working_path.push(Path::new(&path_str));

    let mut file = match File::create(&working_path) {
        Err(err) => panic!(
            "Error: Could not create file {}\nReason:{}",
            working_path.display(),
            err
        ),
        Ok(file) => file,
    };
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
