// ---------------------------------------------------------------------------
// Room trigger — bits 2-0 of AttrsF.
// ---------------------------------------------------------------------------

use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoomTrigger {
    None,
    Boss,
    KillAll,
    Other(u8),
}

impl RoomTrigger {
    pub fn from_bits(bits: u8) -> Self {
        match bits & 0x07 {
            0 => Self::None,
            3 => Self::Boss,
            7 => Self::KillAll,
            v => Self::Other(v),
        }
    }

    pub fn to_bits(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Boss => 3,
            Self::KillAll => 7,
            Self::Other(v) => v & 0x07,
        }
    }
}

impl Display for RoomTrigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                RoomTrigger::None => "-",
                RoomTrigger::Boss => "Boss",
                RoomTrigger::KillAll => "Kill-all",
                RoomTrigger::Other(_) => "Other",
            }
        )
    }
}
