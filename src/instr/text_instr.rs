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
    MOV(Reg, Reg),     // Moves between Regs

    // *** Control Flow ops ***
    JMP(BoolVal, Val),          // Jump
    JG(BoolVal, Val, Reg, Reg), // Jump if greater
    JL(BoolVal, Val, Reg, Reg), // Jump if less
    JE(BoolVal, Val, Reg, Reg), // Jump if equal

    CMP(Reg, Reg), // Compares Regs
    HALT,          // Stop CPU
}
impl TextInstr {
    pub fn from_str(line: &str) -> Result<Self> {
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
            "HALT" | "halt" | "BREAK" | "break" => Ok(TextInstr::HALT),
            _ => panic!("Instruction provided was invalid, {}", command),
        }
    }
    pub fn to_code(&self) -> Vec<u16> {
        match self {
            TextInstr::SHL(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 0,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            }
            TextInstr::SHR(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 1,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::AND(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 2,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::NOT(out_reg, reg_in) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in.to_code(),
                    ALU | 3,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::XOR(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 4,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::OR(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 5,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::ADD(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 6,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::SUB(out_reg, reg_in_a, reg_in_b) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg_in_a.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg_in_b.to_code(),
                    ALU | 7,
                    MOVE | out_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::INC(dest_reg, src_reg) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | src_reg.to_code(),
                    ALU | 9,
                    MOVE | dest_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            }
            TextInstr::DEC(dest_reg, src_reg) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | src_reg.to_code(),
                    ALU | 10,
                    MOVE | dest_reg.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            }

            TextInstr::STR(dest_addr, src_reg) => {
                vec![STORE | src_reg.to_code() << 8 | dest_addr.0 as u16]
            }
            TextInstr::LDR(dest_reg, src_addr) => {
                vec![LOAD | dest_reg.to_code() << 8 | src_addr.0 as u16]
            }
            TextInstr::LDI(dest_reg, val) => {
                vec![LOADIMD | dest_reg.to_code() << 8 | val.0]
            },
            TextInstr::MOV(dest_reg, src_reg) => {
                vec![MOVE | src_reg.to_code() << 4 | dest_reg.to_code()]
            },

            TextInstr::JMP(dir, size) => {
                vec![JUMP | 0 << 10 | dir.to_code() << 9 | size.0]
            },
            TextInstr::JG(dir, size, reg1, reg2) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg1.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg2.to_code(),
                    ALU | 8,
                    MOVE | Reg::FLAGS.to_code() << 4 | Reg::AluOut.to_code(),
                    JUMP | 1 << 10 | dir.to_code() << 9 | size.0
                ]
            },
            TextInstr::JL(dir, size, reg1, reg2) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg1.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg2.to_code(),
                    ALU | 8,
                    MOVE | Reg::FLAGS.to_code() << 4 | Reg::AluOut.to_code(),
                    JUMP | 2 << 10 | dir.to_code() << 9 | size.0
                ]
            },
            TextInstr::JE(dir, size, reg1, reg2) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg1.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg2.to_code(),
                    ALU | 8,
                    MOVE | Reg::FLAGS.to_code() << 4 | Reg::AluOut.to_code(),
                    JUMP | 3 << 10 | dir.to_code() << 9 | size.0
                ]
            },
            TextInstr::CMP(reg1, reg2) => {
                vec![
                    MOVE | Reg::AluA.to_code() << 4 | reg1.to_code(),
                    MOVE | Reg::AluB.to_code() << 4 | reg2.to_code(),
                    ALU | 8,
                    MOVE | Reg::FLAGS.to_code() << 4 | Reg::AluOut.to_code(),
                ]
            },
            TextInstr::HALT => vec![HALT],
        }
    }
}
