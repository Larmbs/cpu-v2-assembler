use super::ParseAble;
use anyhow::{Context, Result};
use lazy_static::lazy_static;
use std::collections::HashMap;

// Defining mappings for quick parsing
lazy_static! {
    /// A mapping between names and Reg enum
    static ref RegNameHash: HashMap<&'static str, Reg> = HashMap::from([
        ("rz", Reg::ZERO),
        ("r0", Reg::ZERO),
        ("ra", Reg::AluA),
        ("r1", Reg::AluA),
        ("rb", Reg::AluB),
        ("r2", Reg::AluB),
        ("ro", Reg::AluOut),
        ("r3", Reg::AluOut),
        ("rf", Reg::FLAGS),
        ("r4", Reg::FLAGS),
        ("rbx", Reg::BASE),
        ("r5", Reg::BASE),
        ("rg1", Reg::GPR1),
        ("r6", Reg::GPR1),
        ("rg2", Reg::GPR2),
        ("r7", Reg::GPR2),
    ]);
    /// A mapping from Regs to address
    static ref RegAddrHash: HashMap<Reg, u8> = HashMap::from([
        (Reg::ZERO, 0),
        (Reg::AluA, 1),
        (Reg::AluB, 2),
        (Reg::AluOut, 3),
        (Reg::FLAGS, 4),
        (Reg::BASE, 5),
        (Reg::GPR1, 6),
        (Reg::GPR2, 7),
    ]);
}

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
    /// Parses out a Reg from a string representation
    fn from_str(name: &str) -> Result<Self> {
        RegNameHash.get(name).context("Invalid register").cloned()
    }
    /// Gets the address of the Reg
    fn to_code(&self) -> u16 {
        *RegAddrHash.get(self).unwrap() as u16
    }
}
