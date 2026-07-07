#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum DoorKind {
    Open = 0,
    Wall = 1,
    InvisibleWall = 2,
    PushBlock = 3,
    Bombable = 4,
    Locked = 5,
    Shutter = 7,
}

impl DoorKind {
    pub(crate) fn from_bits(bits: u8) -> Self {
        match bits & 0x07 {
            0 => Self::Open,
            1 => Self::Wall,
            2 => Self::InvisibleWall,
            3 => Self::PushBlock,
            4 => Self::Bombable,
            5 => Self::Locked,
            6 => Self::Locked,
            7 => Self::Shutter,
            _ => unreachable!(),
        }
    }

    pub(crate) fn to_bits(self) -> u8 {
        match self {
            Self::Open => 0,
            Self::Wall => 1,
            Self::InvisibleWall => 2,
            Self::PushBlock => 3,
            Self::Bombable => 4,
            Self::Locked => 5,
            Self::Shutter => 7,
        }
    }

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Wall => "wall",
            Self::InvisibleWall => "invisible-wall",
            Self::Locked => "locked",
            Self::PushBlock => "push-block",
            Self::Bombable => "bombable",
            Self::Shutter => "shutter",
        }
    }
}
