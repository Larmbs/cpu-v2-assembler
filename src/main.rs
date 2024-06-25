
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
    static ref DEFAULT_OUT: String = format!("o{}", DBMEXEC_EXT);
}

fn main() {
    let mut args = env::args();
    args.next();

    let file_path: path::PathBuf = args.next()
        .expect("You must provide a path to a dbasm file to compile")
        .parse()
        .expect("Path provided was invalid");

    if !file_path.ends_with(DBASM_EXT) {
        eprintln!("You must provide a .dbasm file, DroneBoi Assembly")
    }
    
    let output_path: path::PathBuf = args.next()
        .unwrap_or(DEFAULT_OUT.to_string())
        .parse()
        .expect("You provided an invalid path");

    if !output_path.ends_with(DBMEXEC_EXT) {
        eprintln!("It is best to end your output file with a .dbexec")
    }

    let text: String = fs::read_to_string(file_path)
        .expect("There was a problem reading from that file");

    let prog = cpu_v2_assembler::parse(text).expect("An error occurred");
    let res = cpu_v2_assembler::to_code(prog);

    fs::write(output_path, res)
        .expect("There was an issue writing into file");
}
