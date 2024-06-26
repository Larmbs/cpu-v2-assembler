//! Module for parsing dbasm into machine code

use anyhow::Result;
mod instr;
pub use instr::{Program, TextInstr};

/// DroneBoi Assembly file extension
pub const DBASM_EXT: &str = "dbasm";
/// DroneBoi Executable file extension
pub const DBMEXEC_EXT: &str = "dbexec";


/// Parses a dbasm file into a intermediate representation
pub fn parse(text: String) -> Result<Program> {
    let mut instructions = vec![];

    for line in text.lines() {
        if line.starts_with(";") {continue;}
        if line.trim().is_empty() {continue;}
        instructions.push(TextInstr::from_str(line)?)
    }

    Ok(Program { text: instructions })
}   

/// Turns Program into a string representing itself in machine code
pub fn to_code(program: Program) -> Vec<u16> {
    let mut code: Vec<u16> = vec![];

    for instruction in program.text {
        code.extend(instruction.to_code())
    }

    code
}
