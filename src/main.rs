use std::fs::File;
use std::io::{self, Read, Write};
use std::process;

mod args;
mod config;
mod crypto;
mod operation_type;

use args::Args;
use byteorder::{LittleEndian, WriteBytesExt};
use crypto::{decrypt_data, encrypt_data};
use operation_type::OperationType;

fn write_bytes_to_file(file_path: &str, data: &Vec<u8>) -> Result<(), io::Error> {
    let mut file = File::create(file_path)?;

    file.write_all(data)?;

    Ok(())
}

fn read_file_as_bytes(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn decrypt_file(args: &Args, decompress: bool) {
    let source_file = &args.source_file;
    let file_content;
    let result = read_file_as_bytes(source_file);
    match result {
        Ok(contents) => {
            file_content = contents;
        }
        Err(error) => {
            eprintln!("Error occured while reading the file: {}", error);
            process::exit(1)
        }
    }
    let decrypted_data = decrypt_data(&file_content);
    match decrypted_data {
        Ok(data) => {
            if let Err(e) = write_bytes_to_file(&args.output_file, &data) {
                println!("Failed to write to file: {}", e);
            }
        }
        Err(_e) => {
            println!("Error");
        }
    }
}

fn encrypt_file(args: &Args, compress: bool) {
    let source_file = &args.source_file;
    let file_content;
    let result = read_file_as_bytes(source_file);
    match result {
        Ok(contents) => {
            file_content = contents;
        }
        Err(error) => {
            eprintln!("Error occured while reading the file: {}", error);
            process::exit(1)
        }
    }
    let original_size = file_content.len() as u32;

    let mut size_buffer = Vec::new();

    size_buffer
        .write_u32::<LittleEndian>(original_size)
        .unwrap();

    let encrypted_data = encrypt_data(&file_content);

    match write_bytes_to_file(&args.output_file, &encrypted_data) {
        Ok(_result) => {}
        Err(_e) => {}
    }
}

fn main() {
    let args = Args::parse_args();

    match args.operation {
        OperationType::DECRYPT => decrypt_file(&args, args.use_compression),
        OperationType::ENCRYPT => encrypt_file(&args, args.use_compression),
    };
}
