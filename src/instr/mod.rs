use anyhow::Result;

mod regs;
pub use regs::Reg;

mod text_instr;
pub use text_instr::TextInstr;

mod val;
pub use val::{BoolVal, Val};
mod ram_addr;
pub use ram_addr::RAMAddr;

/// Trait that represents an object that can be parsed and converted to machine code
pub trait ParseAble {
    /// Converts from text into its intermediate representation
    fn from_str(text: &str) -> Result<Self>
    where
        Self: Sized;
    /// Converts from itself into machine code
    fn to_code(&self) -> u16;
}

pub struct Program {
    pub text: Vec<TextInstr>,
}
