#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::fs::read_to_string;
use std::process::Command;

use vc::parse_unparse;

fn capture_stdout_from_subprocess(input_filepath: &str) -> String {
    let subprocess_output = Command::new("./target/debug/vc")
        .arg("unparse")
        .arg(input_filepath)
        .output()
        .expect("Failed to execute command.");

    String::from_utf8_lossy(&subprocess_output.stdout).to_string()
}

fn print_actual_expected(actual: &str, expected: &str) {
    if actual != expected {
        eprintln!("Expected:\n{}", expected);
        eprintln!("Actual:\n{}", actual);
    }
}

fn test_unparsing_filepath(input_filepath: &str) {
    let input_file_string = read_to_string(input_filepath).expect("File reading error.");
    let stdout_string = capture_stdout_from_subprocess(input_filepath);
    print_actual_expected(&stdout_string, &input_file_string);

    assert_eq!(stdout_string, input_file_string);
}

#[test]
fn test_unparsing_fibonacii_numbers() {
    let input_filepath = "./tests/Parser/input/t57.vc";
    let input_file_string = read_to_string(input_filepath).expect("File reading error.");
    let stdout_string = capture_stdout_from_subprocess(input_filepath);
    print_actual_expected(&stdout_string, &input_file_string);

    assert_eq!(stdout_string, input_file_string);
}

#[test]
fn test_unparsing_prime_test() {
    let input_filepath = "./tests/Parser/input/t58.vc";
    test_unparsing_filepath(input_filepath);
}

#[test]
fn test_unparsing_bubble_sort() {
    let input_filepath = "./tests/Parser/input/t59.vc";
    test_unparsing_filepath(input_filepath);
}

#[test]
fn test_numerics() {
    let input_filepath = "./tests/Parser/input/tNumerics.vc";
    test_unparsing_filepath(input_filepath);
}
