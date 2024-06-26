//! Module which defines DBASM instructions

use super::{BoolVal, ParseAble, RAMAddr, Reg, Val};
use anyhow::Result;

const ALU: u16 = 0 << 12;
const LOAD: u16 = 1 << 12;
const STORE: u16 = 2 << 12;
const MOVE: u16 = 3 << 12;
const JUMP: u16 = 4 << 12;
const LOADIMD: u16 = 5 << 12;
const HALT: u16 = 6 << 12;

pub enum TextInstr {
    // *** ALU ops ***
    SHL, // Shift Left
    SHR, // Shift Right
    AND, // Binary AND
    NOT, // Binary NOT
    XOR, // Binary XOR
    OR,  // Binary OR
    ADD, // Addition
    SUB, // Subtraction
    INC, // Increment
    DEC, // Decrement

    // *** Storage ops ***
    STR(RAMAddr, Reg), // Store
    LDR(Reg, RAMAddr), // Load
    LDI(Reg, Val),     // Load a constant
    MOV(Reg, Reg),     // Moves between Regs

    // *** Control Flow ops ***
    JMP(BoolVal, Val), // Jump
    JG(BoolVal, Val),  // Jump if greater
    JL(BoolVal, Val),  // Jump if less
    JE(BoolVal, Val),  // Jump if equal

    CMP,  // Compares Regs
    HALT, // Stop CPU
}
impl TextInstr {
    pub fn from_str(line: &str) -> Result<Self> {
        let line = line.replace(",", " ");
        let mut iterator = line.split_whitespace().into_iter();
        // For a line to be passed it must have at least some command
        let command = iterator.next().unwrap();
        let arg1 = iterator.next();
        let arg2 = iterator.next();

        match command {
            // ALU
            "SHL" | "shl" => Ok(TextInstr::SHL),
            "SHR" | "shr" => Ok(TextInstr::SHR),
            "AND" | "and" => Ok(TextInstr::AND),
            "NOT" | "not" => Ok(TextInstr::NOT),
            "XOR" | "xor" => Ok(TextInstr::XOR),
            "OR" | "or" => Ok(TextInstr::OR),
            "ADD" | "add" => Ok(TextInstr::ADD),
            "SUB" | "sub" => Ok(TextInstr::SUB),
            "INC" | "inc" => Ok(TextInstr::INC),
            "DEC" | "dec" => Ok(TextInstr::DEC),
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
            "MOVE" | "move" | "MOV" | "mov" => Ok(TextInstr::MOV(
                Reg::from_str(arg1.unwrap())?,
                Reg::from_str(arg2.unwrap())?,
            )),
            // Jump instructions
            "JMP" | "jmp" => Ok(TextInstr::JMP(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
            )),
            "JG" | "jg" => Ok(TextInstr::JG(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
            )),
            "JL" | "jl" => Ok(TextInstr::JL(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
            )),
            "JE" | "je" => Ok(TextInstr::JE(
                BoolVal::from_str(arg1.unwrap())?,
                Val::from_str(arg2.unwrap())?,
            )),
            // Other
            "CMP" | "cmp" => Ok(TextInstr::CMP),
            "HALT" | "halt" | "BREAK" | "break" => Ok(TextInstr::HALT),
            _ => panic!("Instruction provided was invalid, {}", command),
        }
    }
    pub fn to_code(&self) -> Vec<u16> {
        match self {
            TextInstr::SHL => {
                vec![ALU | 0]
            }
            TextInstr::SHR => {
                vec![ALU | 1]
            }
            TextInstr::AND => {
                vec![ALU | 2]
            }
            TextInstr::NOT => {
                vec![ALU | 3]
            }
            TextInstr::XOR => {
                vec![ALU | 4]
            }
            TextInstr::OR => {
                vec![ALU | 5]
            }
            TextInstr::ADD => {
                vec![ALU | 6]
            }
            TextInstr::SUB => {
                vec![ALU | 7]
            }
            TextInstr::INC => {
                vec![ALU | 9]
            }
            TextInstr::DEC => {
                vec![ALU | 10]
            }

            TextInstr::STR(dest_addr, src_reg) => {
                vec![STORE | src_reg.to_code() << 8 | dest_addr.0 as u16]
            }
            TextInstr::LDR(dest_reg, src_addr) => {
                vec![LOAD | dest_reg.to_code() << 8 | src_addr.0 as u16]
            }
            TextInstr::LDI(dest_reg, val) => {
                vec![LOADIMD | dest_reg.to_code() << 8 | val.0]
            }
            TextInstr::MOV(dest_reg, src_reg) => {
                vec![MOVE | src_reg.to_code() << 4 | dest_reg.to_code()]
            }

            TextInstr::JMP(dir, size) => {
                vec![JUMP | 0 << 10 | dir.to_code() << 9 | size.0]
            }
            TextInstr::JG(dir, size) => {
                vec![JUMP | 1 << 10 | dir.to_code() << 9 | size.0]
            }
            TextInstr::JL(dir, size) => {
                vec![JUMP | 2 << 10 | dir.to_code() << 9 | size.0]
            }
            TextInstr::JE(dir, size) => {
                vec![JUMP | 3 << 10 | dir.to_code() << 9 | size.0]
            }
            TextInstr::CMP => {
                vec![ALU | 8]
            }
            TextInstr::HALT => vec![HALT],
        }
    }
}
