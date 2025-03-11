// src/main.rs

use clap::{Arg, Command};
use std::fs;
use std::process;
use js_to_ts_compiler::{convert_js_to_ts, convert_js_file_to_ts}; // Import the conversion functions

fn main() {
    // Parse command-line arguments
    let matches = Command::new("js_to_ts_compiler")
        .version("1.0")
        .author("Your Name")
        .about("Converts JavaScript to TypeScript")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input JavaScript file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output TypeScript file")
                .required(true),
        )
        .get_matches();

    // Get the input and output file paths
    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output").unwrap();

    // Convert the JavaScript file to TypeScript
    if let Err(err) = convert_js_file_to_ts(input_file, output_file) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }

    println!("Conversion complete! Output written to {}", output_file);
}