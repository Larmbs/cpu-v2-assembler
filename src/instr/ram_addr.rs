//! Module with RAM address definition

use super::ParseAble;
use anyhow::Result;

/// Represents a RAM address
pub struct RAMAddr(pub usize);
impl ParseAble for RAMAddr {
    fn from_str(text: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let num: usize = text.parse()?;
        Ok(RAMAddr { 0: num })
    }
    fn to_code(&self) -> u16 {
        self.0 as u16
    }
}
