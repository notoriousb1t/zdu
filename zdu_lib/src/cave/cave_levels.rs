use crate::cave::shop::Shop;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct CaveLevels {
    pub shop_keys: Vec<Shop>,
}

impl Display for CaveLevels {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if !self.shop_keys.is_empty() {
            writeln!(f, "## Shop Keys\n")?;
            for sk in &self.shop_keys {
                let slot_name = match sk.slot {
                    0 => "Left",
                    1 => "Middle",
                    _ => "Right",
                };
                writeln!(
                    f,
                    "| {} | {} | {} rupees |",
                    sk.shop_index, slot_name, sk.price
                )?;
            }
            writeln!(f, "")
        } else {
            writeln!(f, "No shops")
        }
    }
}
