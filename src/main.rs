
use cpu_v2_assembler::{
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

fn main() {
    let mut args = env::args();
    args.next();

    let file_path: path::PathBuf = args.next()
        .expect("You must provide a path to a dbasm file to compile")
        .parse()
        .expect("Path provided was invalid");

    if file_path.extension().unwrap() != DBASM_EXT {
        eprintln!("You must provide a .dbasm file, DroneBoi Assembly")
    }
    
    let output_path: path::PathBuf = args.next()
        .unwrap_or(DEFAULT_OUT.to_string())
        .parse()
        .expect("You provided an invalid path");

    if output_path.extension().unwrap() != DBMEXEC_EXT {
        eprintln!("It is best to end your output file with a .dbexec")
    }

    let text: String = fs::read_to_string(file_path)
        .expect("There was a problem reading from that file");

    let prog = parse(text).expect("An error occurred");
    let res = to_code(prog);

    let output: String = res.iter()
        .enumerate()
        .map(|(ix, num)| format!("| {:^3} | {:7} | {:#018b}\n", ix, num, num))
        .collect();
    let output = format!("| {:^3} | {:7} | {:18}\n", "IDX", "Decimal", "Binary") + &"-".repeat(36) + "\n" + output.as_str();

    fs::write(output_path, output)
        .expect("There was an issue writing into file");
}
