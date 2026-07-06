#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DoorKind {
    Open = 0,
    Wall = 1,
    Shutter = 2,
    PushBlock = 3,
    Bombable = 4,
    Locked = 5,
    BossShutter = 7,
    /// Values like 6 are not assigned in vanilla; stored as-is for round-tripping.
    Unknown(u8),
}

impl DoorKind {
    pub(crate) fn from_bits(bits: u8) -> Self {
        match bits & 0x07 {
            0 => Self::Open,
            1 => Self::Wall,
            2 => Self::Locked, // Was Shutter!
            3 => Self::PushBlock,
            4 => Self::Bombable,
            5 => Self::Shutter, // Was Locked!
            6 => Self::Unknown(6),
            7 => Self::BossShutter,
            _ => unreachable!(),
        }
    }

    pub(crate) fn to_bits(self) -> u8 {
        match self {
            Self::Open => 0,
            Self::Wall => 1,
            Self::Locked => 2,
            Self::PushBlock => 3,
            Self::Bombable => 4,
            Self::Shutter => 5,
            Self::BossShutter => 7,
            Self::Unknown(v) => v & 0x07,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Wall => "wall",
            Self::Locked => "locked",
            Self::PushBlock => "push-block",
            Self::Bombable => "bombable",
            Self::Shutter | Self::BossShutter => "shutter",
            Self::Unknown(_) => "wall",
        }
    }
}
