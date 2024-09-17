use std::error::Error;
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

fn decrypt_file(args: &Args, decompress: bool) -> Result<(), Box<dyn Error>> {
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
    let mut data = decrypt_data(&file_content);
    if let Err(e) = data {
        eprintln!("Error decrypting data: {}", e);
        process::exit(1);
    }
    if decompress {
        match crypto::decompress_data(&data.unwrap()) {
            Ok(decompressed_data) => {
                data = Ok(decompressed_data);
            }
            Err(e) => {
                eprintln!("Error decompressing data: {}", e);
                process::exit(1);
            }
        }
    }

    match write_bytes_to_file(&args.output_file, &data.unwrap()) {
        Ok(result) => Ok(()),
        Err(e) => Err(Box::new(e)),
    }
}

fn encrypt_file(args: &Args, compress: bool) -> Result<(), Box<dyn Error>> {
    let source_file = &args.source_file;
    let mut file_content;
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

    if compress {
        println!("compressing data");
        match crypto::compress_data(&file_content) {
            Ok(compressed_data) => {
                file_content = compressed_data;
            }
            Err(e) => {
                eprintln!("Error compressing data: {}", e);
                process::exit(1);
            }
        }
    }

    let original_size = file_content.len() as u32;

    let mut size_buffer = Vec::new();

    size_buffer
        .write_u32::<LittleEndian>(original_size)
        .unwrap();

    let encrypted_data = encrypt_data(&file_content);

    match write_bytes_to_file(&args.output_file, &encrypted_data) {
        Ok(_result) => Ok(()),
        Err(_e) => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Failed to write to file",
        ))),
    }
}

fn main() {
    let args = Args::parse_args();

    match args.operation {
        OperationType::DECRYPT => decrypt_file(&args, args.use_compression),
        OperationType::ENCRYPT => encrypt_file(&args, args.use_compression),
    };
}
