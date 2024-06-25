//! Defines simples values to be written explicitly

use anyhow::Result;
use super::ParseAble;

/// Represents just some 16 bit number
pub struct Val(pub u16);
impl ParseAble for Val {
    fn from_str(text: &str) -> Result<Self> where Self: Sized {
        let num: u16 = text.parse()?;
        Ok(Val{0:num})
    }

    fn to_code(&self) -> u16 {
        self.0
    }
}

/// Boolean value can be either zero or one
pub struct BoolVal(pub bool);
impl ParseAble for BoolVal {
    fn from_str(text: &str) -> Result<Self> where Self: Sized {
        let dir: u8 = text.parse()?;
        Ok(BoolVal{0: dir > 0})
    }

    fn to_code(&self) -> u16 {
        if self.0 {1} else {0}
    }
}
