#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use vc::parse_unparse;

#[test]
fn test_unparsing_fibonacii_numbers() {
    let input_filepath = "./tests/Parser/input/t57.vc";

    // Capture the standard output from parse unparse, and print it as a string.
    // TODO: Figure out a way to capture standard output in Rust.
    parse_unparse(input_filepath);
}