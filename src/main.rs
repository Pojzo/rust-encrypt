use aes::cipher::generic_array::iter;
use clap::{Arg, Command};
use pad::PadStr;
use std::fs::read_to_string;
use std::process;
use std::str::FromStr;

#[derive(Debug)]
enum OperationType {
    ENCRYPT,
    DECRYPT,
}

impl FromStr for OperationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "encrypt" => Ok(OperationType::ENCRYPT),
            "decrypt" => Ok(OperationType::DECRYPT),
            _ => Err(format!("Invalid value for operation type: {}", s)),
        }
    }
}

#[derive(Debug)]
struct Args {
    source_file: String,
    output_file: String,
    operation: OperationType,
}

fn parse_args() -> Args {
    let matches = Command::new("My App")
        .version("1.0")
        .author("Peter Kovac pojzinko8@gmail.com")
        .about("Argument parser")
        .arg(Arg::new("input_file").required(true).index(1))
        .arg(
            Arg::new("output_file")
                .required(false)
                .short('o')
                .long("output")
                .default_value("output"),
        )
        .arg(
            Arg::new("operation")
                .required(false)
                .short('t')
                .long("operation")
                .default_value("encrypt")
                .possible_values(&["encrypt", "decrypt"]),
        )
        .get_matches();

    let source_file = matches.get_one::<String>("input_file").unwrap().to_string();
    let output_file = matches
        .get_one::<String>("output_file")
        .unwrap()
        .to_string();
    let operation_str = matches
        .get_one::<String>("operation")
        .expect("Operation type is required")
        .to_string();
    let operation = OperationType::from_str(&operation_str).expect("Invalid operation type");

    Args {
        source_file,
        output_file,
        operation,
    }
}

const CHUNK_SIZE: usize = 16;

fn pad_string(string: &str, chunk_size: usize) -> String {
    let target_len = if string.len() % chunk_size != 0 {
        ((string.len() / chunk_size) + 1) * chunk_size
    } else {
        string.len()
    };
    let mut padded = string.to_string();

    if string.len() < target_len {
        let pad_len = target_len - string.len();
        padded.push_str(&" ".repeat(pad_len));
    }

    return padded;
}

fn split_to_chunks(input: &str, chunk_size: usize) -> Vec<String> {
    let padded = pad_string(input, chunk_size);
    let iterations = padded.len() / chunk_size;

    let mut output: Vec<String> = Vec::new();

    for i in 0..iterations {
        let start = i * chunk_size;
        let end = (i + 1) * chunk_size;

        let chunk = &padded[start..end];
        println!("Pushing {}", i);
        output.push(chunk.to_string());
    }

    return output;
}

fn main() {
    let args = parse_args();

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
