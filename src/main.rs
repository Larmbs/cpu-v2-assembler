
use std::env;


const DEFAULT_OUT: &str = "a.dbmc";

fn main() {
    let mut args = env::args();
    args.next();

    let file_path = args.next().expect("You must provide a path to a dbasm file to compile");
    if !file_path.ends_with(".dbasm") {
        eprintln!("You must provide a .dbasm file, DroneBoi Assembly")
    }
    
    let output_path = args.next().unwrap_or(String::from(DEFAULT_OUT));

    println!("Hello, world!");
}
