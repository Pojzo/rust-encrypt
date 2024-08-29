use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};
use std::process;

mod args;
mod config;
mod encrypt;
mod operation_type;

use args::Args;
use byteorder::{LittleEndian, WriteBytesExt};
use encrypt::{decrypt_data, encrypt_data};
use operation_type::OperationType;

fn write_bytes_to_file(file_path: &str, data: &Vec<u8>) -> Result<(), io::Error> {
    // Open the file in write mode
    let mut file = File::create(file_path)?;

    // Write the byte data to the file
    file.write_all(data)?;

    Ok(())
}

fn write_string_to_file(file_path: &str, data: &str) -> Result<(), io::Error> {
    let bytes = data.as_bytes();
    let mut file = File::create(file_path)?;

    file.write(bytes);

    Ok(())
}

fn read_file_as_bytes(path: &str) -> io::Result<Vec<u8>> {
    // Open the file
    let mut file = File::open(path)?;

    // Create a buffer to store the file contents
    let mut buffer = Vec::new();

    // Read the file contents into the buffer
    file.read_to_end(&mut buffer)?;

    // Return the buffer containing the file bytes
    Ok(buffer)
}

fn decrypt_file(args: &Args) {
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

fn encrypt_file(args: &Args) {
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
        OperationType::DECRYPT => decrypt_file(&args),
        OperationType::ENCRYPT => encrypt_file(&args),
    };
}
