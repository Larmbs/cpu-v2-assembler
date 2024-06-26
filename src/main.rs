
use dbasm::{
    parse,
    to_code,
    DBASM_EXT,
    DBMEXEC_EXT,
};

use std::env;
use std::fs;
use std::path;
use lazy_static::lazy_static;

lazy_static! {
    static ref DEFAULT_OUT: String = format!("o.{}", DBMEXEC_EXT);
}

const INVALID_PATH: &str = "Invalid Path";
const NOT_DBASM_FILE: &str = "File Must End In .dbasm";
const NOT_DBEXEC_FILE: &str = "File Must End In .dbexec";
const MUST_BE_NUM: &str = "You Must Provide A Number As Input";
const INVALID_OUTPUT_TYPE: &str = "Output Type Was Invalid";
const ASSEMBLER_ERROR: &str = "There Was A Problem Assembling";
const READ_ERROR: &str = "Issue Reading File";
const WRITE_ERROR: &str = "Issue Writing To File";

fn main() {
    let mut args = env::args();
    args.next();

    let file_path: path::PathBuf = args.next()
        .expect(INVALID_PATH)
        .parse()
        .expect(INVALID_PATH);
    if file_path.extension().unwrap() != DBASM_EXT {
        eprintln!("{}", NOT_DBASM_FILE);
    }
    
    let output_type: u8 = args.next()
        .unwrap_or(String::from("0"))
        .parse()
        .expect(MUST_BE_NUM);
    
    let output_path: path::PathBuf = args.next()
        .unwrap_or(DEFAULT_OUT.to_string())
        .parse()
        .expect(INVALID_PATH);
    if output_path.extension().unwrap() != DBMEXEC_EXT {
        eprintln!("{}", NOT_DBEXEC_FILE);
    }

    // Converting file into machine code
    let text: String = fs::read_to_string(file_path)
        .expect(READ_ERROR);
    let prog = parse(text)
        .expect(ASSEMBLER_ERROR);
    let code = to_code(prog);

    let output = match output_type {
        0 => as_is(code),
        1 => to_table(code),
        _ => String::from(INVALID_OUTPUT_TYPE)
    };

    fs::write(output_path, output)
        .expect(WRITE_ERROR);
}

/// Just a basic representation of numbers
fn as_is(code: Vec<u16>) -> String {
    code.iter().map(|num| num.to_string()).collect::<Vec<String>>().join("\n")
}
/// Turns code into a nice data for debugging
fn to_table(code: Vec<u16>) -> String {
    let output: String = code.iter()
        .enumerate()
        .map(|(ix, num)| format!("| {:^3} | {:7} | {:#018b}\n", ix, num, num))
        .collect();
    let output = format!("| {:^3} | {:7} | {:18}\n", "IDX", "Decimal", "Binary") + &"-".repeat(36) + "\n" + output.as_str();
    output
}
