//! Module for parsing dbasm into machine code

use anyhow::Result;
mod instr;
pub use instr::{Program, TextInstr};

/// DroneBoi Assembly file extension
pub const DBASM_FILE: &str = ".dbasm";
/// DroneBoi Executable file extension
pub const DBMEXEC: &str = ".dbexec";


/// Parses a dbasm file into a intermediate representation
pub fn parse(text: String) -> Result<Program> {
    let mut instructions = vec![];

    for line in text.lines() {
        if line.starts_with(";") {continue;}
        instructions.push(TextInstr::from_str(line)?)
    }
    Ok(Program { text: instructions })
}   

/// Turns Program into a string representing itself in machine code
pub fn to_code(program: Program) -> String {
    let mut sections: Vec<u16> = vec![];

    for instruction in program.text {
        sections.extend(instruction.to_code())
    }

    let str_sections: Vec<String> = sections.iter().map(|num| num.to_string()).collect();
    let text = str_sections.join("\n");
    text
}
