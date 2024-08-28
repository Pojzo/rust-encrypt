use std::fs::{read_to_string, File};
use std::io::{self, Read, Write};
use std::process;

mod args;
mod config;
mod encrypt;
mod operation_type;

use args::Args;
use encrypt::{decrypt_data, encrypt_data};
use operation_type::OperationType;

fn write_bytes_to_file(file_path: &str, data: &[u8]) -> Result<(), io::Error> {
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
    let file_vec = &file_content.to_vec();
    let decrypted_data = decrypt_data(file_vec);
    match write_string_to_file(&args.output_file, &decrypted_data) {
        Ok(_result) => {}
        Err(_e) => eprintln!("Error writing data to file"),
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
    let file_vec = &file_content.to_vec();
    let encrypted_data = encrypt_data(&file_content);
    println!("This is encrypted data: {:?}", encrypted_data);
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
