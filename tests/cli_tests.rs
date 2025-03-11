// tests/cli_tests.rs

use std::process::Command;
use std::fs;

#[test]
fn test_cli_conversion() {
    // Define input and output files
    let input_file = "test_input.js";
    let output_file = "test_output.ts";

    // Create a test input file
    let js_code = r#"
        function add(a, b) {
            return a + b;
        }
    "#;
    fs::write(input_file, js_code).expect("Failed to write test input file");

    // Run the CLI command
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input")
        .arg(input_file)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute command");

    // Check if the command ran successfully
    assert!(output.status.success(), "CLI command failed");

    // Read the output file
    let ts_code = fs::read_to_string(output_file).expect("Failed to read output file");

    // Verify the output
    assert_eq!(
        ts_code,
        r#"
        function add(a: number, b: number): number {
            return a + b;
        }
    "#
    );

    // Clean up test files
    fs::remove_file(input_file).expect("Failed to remove input file");
    fs::remove_file(output_file).expect("Failed to remove output file");
}

#[test]
fn test_cli_invalid_input() {
    // Define input and output files
    let input_file = "test_invalid_input.js";
    let output_file = "test_invalid_output.ts";

    // Create a test input file with invalid syntax
    let js_code = r#"
        let x = ;
    "#;
    fs::write(input_file, js_code).expect("Failed to write test input file");

    // Run the CLI command
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input")
        .arg(input_file)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute command");

    // Check if the command failed as expected
    assert!(!output.status.success(), "CLI command should fail for invalid input");

    // Verify the error message
    let stderr = String::from_utf8(output.stderr).expect("Failed to read stderr");
    assert!(
        stderr.contains("Invalid JavaScript syntax: incomplete expression"),
        "Expected error message not found"
    );

    // Clean up test files
    fs::remove_file(input_file).expect("Failed to remove input file");
    if fs::metadata(output_file).is_ok() {
        fs::remove_file(output_file).expect("Failed to remove output file");
    }
}

#[test]
fn test_cli_complex_javascript() {
    // Define input and output files
    let input_file = "test_complex_input.js";
    let output_file = "test_complex_output.ts";

    // Create a test input file with complex JavaScript
    let js_code = r#"
        function complexFunction(a, b, c) {
            if (a > b) {
                return { result: a + b, type: "sum" };
            } else {
                return { result: c, type: "value" };
            }
        }
    "#;
    fs::write(input_file, js_code).expect("Failed to write test input file");

    // Run the CLI command
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input")
        .arg(input_file)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute command");

    // Check if the command ran successfully
    assert!(output.status.success(), "CLI command failed");

    // Read the output file
    let ts_code = fs::read_to_string(output_file).expect("Failed to read output file");

    // Verify the output
    assert_eq!(
        ts_code,
        r#"
        function complexFunction(a: number, b: number, c: number): { result: number, type: string } {
            if (a > b) {
                return { result: a + b, type: "sum" };
            } else {
                return { result: c, type: "value" };
            }
        }
    "#
    );

    // Clean up test files
    fs::remove_file(input_file).expect("Failed to remove input file");
    fs::remove_file(output_file).expect("Failed to remove output file");
}

#[test]
fn test_cli_file_not_found() {
    // Define a non-existent input file
    let input_file = "non_existent_file.js";
    let output_file = "test_output.ts";

    // Run the CLI command
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input")
        .arg(input_file)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute command");

    // Check if the command failed as expected
    assert!(!output.status.success(), "CLI command should fail for non-existent input file");

    // Verify the error message
    let stderr = String::from_utf8(output.stderr).expect("Failed to read stderr");
    assert!(
        stderr.contains("Error: Input file not found: non_existent_file.js"),
        "Expected error message not found"
    );
}

#[test]
fn test_cli_large_file() {
    // Define input and output files
    let input_file = "test_large_input.js";
    let output_file = "test_large_output.ts";

    // Create a large test input file
    let js_code = "let x = 10;\n".repeat(10000); // 10,000 lines of code
    fs::write(input_file, js_code).expect("Failed to write test input file");

    // Run the CLI command
    let output = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input")
        .arg(input_file)
        .arg("--output")
        .arg(output_file)
        .output()
        .expect("Failed to execute command");

    // Check if the command ran successfully
    assert!(output.status.success(), "CLI command failed");

    // Read the output file
    let ts_code = fs::read_to_string(output_file).expect("Failed to read output file");

    // Verify the output
    assert_eq!(
        ts_code,
        "let x: number = 10;\n".repeat(10000)
    );

    // Clean up test files
    fs::remove_file(input_file).expect("Failed to remove input file");
    fs::remove_file(output_file).expect("Failed to remove output file");
}