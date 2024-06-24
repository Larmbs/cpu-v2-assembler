//! Module for parsing dbasm into machine code

use anyhow::Result;
mod instr;
pub use instr::Program;

/// DroneBoi Assembly file extension
pub const DBASM_FILE: &str = ".dbasm";
/// DroneBoi Executable file extension
pub const DBMEXEC: &str = ".dbexec";


/// Parses a dbasm file into a intermediate representation
pub fn parse() -> Result<Program> {
    todo!()
}   

/// Turns Program into a string representing itself in machine code
pub fn to_code(program: Program) -> String {
    todo!()
}
