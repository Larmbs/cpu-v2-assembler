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
    fn from_str(line: &str) -> Result<Self> {
        let line = line.replace(",", " ");
        let mut iterator = line.split_whitespace().into_iter();
        // For a line to be passed it must have at least some command
        let command = iterator.next().unwrap();
        let arg1 = iterator.next();
        let arg2 = iterator.next();
        let arg3 = iterator.next();
        let arg4 = iterator.next();

        match command {
            // ALU
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
            "AND" | "and" => Ok(TextInstr::AND(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),
            "NOT" | "not" => Ok(TextInstr::NOT(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
            )),
            "XOR" | "xor" => Ok(TextInstr::XOR(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),
            "OR" | "or" => Ok(TextInstr::OR(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),
            "ADD" | "add" => Ok(TextInstr::ADD(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),
            "SUB" | "sub" => Ok(TextInstr::SUB(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
            )),
            "INC" | "inc" => Ok(TextInstr::INC(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
            )),
            "DEC" | "dec" => Ok(TextInstr::DEC(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
            )),
            // Data
            "STR" | "str" | "STORE" | "store" => Ok(TextInstr::STR(
                RAMAddr::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
            )),
            "LDR" | "ldr" | "LOAD" | "load" => Ok(TextInstr::LDR(
                Reg::from_str(arg1.unwrap())?,
                RAMAddr::from_str(arg2.unwrap())?,
            )),
            "LDI" | "ldi" => Ok(TextInstr::LDI(
                Reg::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
            )),
            "HALT" | "halt" | "BREAK" | "break" => Ok(TextInstr::HALT),
            // Jump instructions
            "JMP" | "jmp" => Ok(TextInstr::JMP(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
            )),
            "JG" | "jg" => Ok(TextInstr::JG(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
                Reg::from_str(arg4.unwrap())?,
            )),
            "JL" | "jl" => Ok(TextInstr::JL(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
                Reg::from_str(arg4.unwrap())?,
            )),
            "JE" | "je" => Ok(TextInstr::JE(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
                Reg::from_str(arg3.unwrap())?,
                Reg::from_str(arg4.unwrap())?,
            )),
            // Other
            "CMP" | "cmp" => Ok(TextInstr::CMP(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
            )),
            _ => panic!("Instruction provided was invalid"),
        }
    }
    fn to_code(&self) -> u16 {
        match self {
            TextInstr::SHL(_, _, _) => todo!(),
            TextInstr::SHR(_, _, _) => todo!(),
            TextInstr::AND(_, _, _) => todo!(),
            TextInstr::NOT(_, _) => todo!(),
            TextInstr::XOR(_, _, _) => todo!(),
            TextInstr::OR(_, _, _) => todo!(),
            TextInstr::ADD(_, _, _) => todo!(),
            TextInstr::SUB(_, _, _) => todo!(),
            TextInstr::INC(_, _) => todo!(),
            TextInstr::DEC(_, _) => todo!(),
            TextInstr::STR(_, _) => todo!(),
            TextInstr::LDR(_, _) => todo!(),
            TextInstr::LDI(_, _) => todo!(),
            TextInstr::HALT => 6 << 12,
            TextInstr::JMP(_, _) => todo!(),
            TextInstr::JG(_, _, _, _) => todo!(),
            TextInstr::JL(_, _, _, _) => todo!(),
            TextInstr::JE(_, _, _, _) => todo!(),
            TextInstr::CMP(_, _) => todo!(),
        }
    }
}
