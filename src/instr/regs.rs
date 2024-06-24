//! Module which defines registers
 
use super::ParseAble;
use anyhow::Result;

/// Represents all Regs on the CPU
#[derive(PartialEq, Hash, Eq, Clone)]
pub enum Reg {
    ZERO,
    AluA,
    AluB,
    AluOut,
    FLAGS,
    BASE,
    GPR1,
    GPR2,
}
impl ParseAble for Reg {
    fn from_str(name: &str) -> Result<Self> {
        match name {
            "rz" | "r0" => Ok(Reg::ZERO),
            "ra" | "r1" => Ok(Reg::AluA),
            "rb" | "r2" => Ok(Reg::AluB),
            "ro" | "r3" => Ok(Reg::AluOut),
            "rf" | "r4" => Ok(Reg::FLAGS),
            "rbx" | "r5" => Ok(Reg::BASE),
            "rg1" | "r6" => Ok(Reg::GPR1),
            "rg2" | "r7" => Ok(Reg::GPR2),
            _ => panic!("Register provided was invalid")
        }
    }
    fn to_code(&self) -> u16 {
        match self {
            Reg::ZERO => 0,
            Reg::AluA => 1,
            Reg::AluB => 2,
            Reg::AluOut => 3,
            Reg::FLAGS => 4,
            Reg::BASE => 5,
            Reg::GPR1 => 6,
            Reg::GPR2 => 7,
        }
    }
}
