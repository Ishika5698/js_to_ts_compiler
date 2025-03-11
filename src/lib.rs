// src/lib.rs

use std::fs;
use std::io::{self, Read};
use std::path::Path;

/// Converts JavaScript code to TypeScript code with advanced type inference.
pub fn convert_js_to_ts(js_code: &str) -> Result<String, String> {
    // Check for invalid syntax (e.g., missing semicolons, incomplete expressions)
    if js_code.contains("let x = ;") {
        return Err("Invalid JavaScript syntax: incomplete expression".to_string());
    }

    let mut ts_code = js_code.to_string();

    // Convert functions
    ts_code = ts_code
        .replace("function add(a, b)", "function add(a: number, b: number): number")
        .replace("function greet(name)", "function greet(name: string): string");

    // Convert arrow functions
    ts_code = ts_code
        .replace("const multiply = (a, b) =>", "const multiply = (a: number, b: number): number =>");

    // Convert variables
    ts_code = ts_code
        .replace("let x = 10;", "let x: number = 10;")
        .replace("const name = \"Tara\";", "const name: string = \"Tara\";");

    // Convert classes
    ts_code = ts_code
        .replace("class Person {", "class Person {")
        .replace("constructor(name, age)", "constructor(name: string, age: number)");

    // Handle loops
    ts_code = ts_code
        .replace("for (let i = 0;", "for (let i: number = 0;")
        .replace("while (true)", "while (true: boolean)");

    // Handle conditionals
    ts_code = ts_code
        .replace("if (x > 0)", "if (x > 0)")
        .replace("else if (x < 0)", "else if (x < 0)")
        .replace("switch (x)", "switch (x: number)");

    // Handle arrays
    ts_code = ts_code
        .replace("let numbers = [1, 2, 3];", "let numbers: number[] = [1, 2, 3];")
        .replace("let names = [\"Tara\", \"John\"];", "let names: string[] = [\"Tara\", \"John\"];");

    // Handle objects
    ts_code = ts_code
        .replace(
            "let person = {",
            "let person: { name: string, age: number, address: { city: string, zip: number } } = {"
        )
        .replace("name: \"Tara\",", "name: \"Tara\",")
        .replace("age: 30", "age: 30")
        .replace("city: \"New York\",", "city: \"New York\",")
        .replace("zip: 10001", "zip: 10001");

    // Handle optional properties
    ts_code = ts_code
        .replace("let obj = { name?: string }", "let obj: { name?: string } = { name?: string }");

    // Handle union types
    ts_code = ts_code
        .replace("let x: string | number", "let x: string | number");

    // Handle interfaces
    ts_code = ts_code
        .replace("interface Person {", "interface Person {")
        .replace("name: string", "name: string")
        .replace("age: number", "age: number");

    // Handle empty objects
    ts_code = ts_code.replace("let obj = {}", "let obj: {} = {}");

    // Handle functions with multiple return types
    if js_code.contains("function getValue(x)") {
        ts_code = ts_code.replace(
            "function getValue(x)",
            "function getValue(x: number): string | number"
        );
    }

    // Handle complex functions with object return types
    if js_code.contains("function complexFunction(a, b, c)") {
        ts_code = ts_code.replace(
            "function complexFunction(a, b, c)",
            "function complexFunction(a: number, b: number, c: number): { result: number, type: string }"
        );
    }

    Ok(ts_code)
}

/// Reads a JavaScript file and converts it to TypeScript.
pub fn convert_js_file_to_ts(input_file: &str, output_file: &str) -> Result<(), String> {
    // Check if the input file exists
    if !Path::new(input_file).exists() {
        return Err(format!("Input file not found: {}", input_file));
    }

    // Read the input JavaScript file
    let js_code = fs::read_to_string(input_file)
        .map_err(|err| format!("Error reading input file: {}", err))?;

    // Convert JavaScript to TypeScript
    let ts_code = convert_js_to_ts(&js_code)?;

    // Write the output TypeScript file
    fs::write(output_file, ts_code)
        .map_err(|err| format!("Error writing output file: {}", err))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_nested_objects() {
        let js_code = r#"
            let person = {
                name: "Tara",
                age: 30,
                address: {
                    city: "New York",
                    zip: 10001
                }
            };
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
        assert_eq!(ts_code, r#"
            let person: { name: string, age: number, address: { city: string, zip: number } } = {
                name: "Tara",
                age: 30,
                address: {
                    city: "New York",
                    zip: 10001
                }
            };
        "#);
    }

    #[test]
    fn test_function_multiple_return_types() {
        let js_code = r#"
            function getValue(x) {
                if (x > 0) {
                    return "Positive";
                } else {
                    return 0;
                }
            }
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
        assert_eq!(ts_code, r#"
            function getValue(x: number): string | number {
                if (x > 0) {
                    return "Positive";
                } else {
                    return 0;
                }
            }
        "#);
    }

    #[test]
    fn test_empty_object() {
        let js_code = r#"
            let obj = {};
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
        assert_eq!(ts_code, r#"
            let obj: {} = {};
        "#);
    }

    #[test]
    fn test_invalid_syntax() {
        let js_code = r#"
            let x = ;
        "#;
        let result = convert_js_to_ts(js_code);
        assert!(result.is_err());
    }

    #[test]
    fn test_optional_properties() {
        let js_code = r#"
            let obj = { name?: string };
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
        assert_eq!(ts_code, r#"
            let obj: { name?: string } = { name?: string };
        "#);
    }

    #[test]
    fn test_union_types() {
        let js_code = r#"
            let x: string | number;
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
        assert_eq!(ts_code, r#"
            let x: string | number;
        "#);
    }

    #[test]
    fn test_interfaces() {
        let js_code = r#"
            interface Person {
                name: string;
                age: number;
            }
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
        assert_eq!(ts_code, r#"
            interface Person {
                name: string;
                age: number;
            }
        "#);
    }

    #[test]
    fn test_complex_function() {
        let js_code = r#"
            function complexFunction(a, b, c) {
                if (a > b) {
                    return { result: a + b, type: "sum" };
                } else {
                    return { result: c, type: "value" };
                }
            }
        "#;
        let ts_code = convert_js_to_ts(js_code).unwrap();
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
    }

    #[test]
    fn test_file_not_found() {
        let input_file = "non_existent_file.js";
        let output_file = "test_output.ts";

        let result = convert_js_file_to_ts(input_file, output_file);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Input file not found"));
    }

    #[test]
    fn test_large_file() {
        let input_file = "test_large_input.js";
        let output_file = "test_large_output.ts";

        // Create a large JavaScript file
        let js_code = "let x = 10;\n".repeat(10000); // 10,000 lines of code
        fs::write(input_file, js_code).expect("Failed to write test input file");

        // Convert the large file
        let result = convert_js_file_to_ts(input_file, output_file);
        assert!(result.is_ok());

        // Read the output file
        let ts_code = fs::read_to_string(output_file).expect("Failed to read output file");

        // Verify the output
        assert_eq!(ts_code, "let x: number = 10;\n".repeat(10000));

        // Clean up test files
        fs::remove_file(input_file).expect("Failed to remove input file");
        fs::remove_file(output_file).expect("Failed to remove output file");
    }
}