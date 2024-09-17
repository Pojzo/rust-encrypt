use clap::{Arg, Command};

use std::str::FromStr;

use crate::operation_type::OperationType;

#[derive(Debug)]
pub struct Args {
    pub source_file: String,
    pub output_file: String,
    pub operation: OperationType,
    pub use_compression: bool, // this specifieds whether we should use compression or
                               // decompression, based on the opeartion type
}

impl Args {
    pub fn parse_args() -> Args {
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
            .arg(
                Arg::new("use_compression")
                    .required(false)
                    .short('c')
                    .long("compress")
                    .default_value("false")
                    .possible_values(&["true", "false"]),
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
        let use_compression_str = matches
            .get_one::<String>("use_compression")
            .unwrap()
            .to_string();

        Args {
            source_file,
            output_file,
            operation,
            use_compression: use_compression_str == "true",
        }
    }
}
