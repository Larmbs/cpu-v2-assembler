//! Module which defines DBASM instructions

use super::{BoolVal, ParseAble, RAMAddr, Reg, Val};
use anyhow::Result;

pub enum TextInstr {
    // *** ALU ops ***
    SHL(Reg, Reg, Reg), // Shift Left
    SHR(Reg, Reg, Reg), // Shift Right
    AND(Reg, Reg, Reg), // Binary AND
    NOT(Reg, Reg),      // Binary NOT
    XOR(Reg, Reg, Reg), // Binary XOR
    OR(Reg, Reg, Reg),  // Binary OR
    ADD(Reg, Reg, Reg), // Addition
    SUB(Reg, Reg, Reg), // Subtraction
    INC(Reg, Reg),      // Increment
    DEC(Reg, Reg),      // Decrement

    // *** Storage ops ***
    STR(RAMAddr, Reg), // Store
    LDR(Reg, RAMAddr), // Load
    LDI(Reg, Val),     // Load a constant
    HALT,              // Stop CPU

    // *** Control Flow ops ***
    JMP(BoolVal, Val),          // Jump
    JG(BoolVal, Val, Reg, Reg), // Jump if greater
    JL(BoolVal, Val, Reg, Reg), // Jump if less
    JE(BoolVal, Val, Reg, Reg), // Jump if equal
    CMP(Reg, Reg),              // Compares Regs
}
impl ParseAble for TextInstr {
    /// Converts the asm instruction into its machine code
    fn to_code(&self) -> u16 {
        todo!()
    }
    /// Gets value from text
    fn from_str(line: &str) -> Result<Self> {
        let mut iterator = line.split_whitespace().into_iter();
        // For a line to be passed it must have at least some command
        let command = iterator.next().unwrap();
        let arg1 = iterator.next();
        let arg2 = iterator.next();
        let arg3 = iterator.next();

        match command {
            "SHL" | "shl" => Ok(TextInstr::SHL(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),
            "SHR" | "shr" => Ok(TextInstr::SHR(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),

            _ => panic!("Instruction provided was invalid"),
        }
    }
}
