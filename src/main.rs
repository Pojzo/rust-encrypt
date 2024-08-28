use std::fs::read_to_string;
use std::process;

mod args;
mod encrypt;
mod operation_type;

const CHUNK_SIZE: usize = 16;

use args::Args;
use encrypt::split_to_chunks;

fn main() {
    let args = Args::parse_args();

    format!("{:?}", args);
    let Args {
        source_file,
        output_file,
        operation,
    } = args;
    println!("{}, {}, {:?}", source_file, output_file, operation);
    let file_content;

    let result = read_to_string(source_file);
    match result {
        Ok(contents) => {
            file_content = contents;
        }
        Err(error) => {
            eprintln!("Error occured while reading the file: {}", error);
            process::exit(1)
        }
    }
    let chunks = split_to_chunks(&file_content, CHUNK_SIZE);
    println!("{}", chunks.len());
    for chunk in chunks.iter() {
        println!("{}", chunk);
    }

    // println!("With text:\n{}", file_content);
}
