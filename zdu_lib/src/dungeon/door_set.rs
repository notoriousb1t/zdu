// ---------------------------------------------------------------------------
// Door set — one per room, covering all four walls.
// ---------------------------------------------------------------------------

use crate::dungeon::door_kind::DoorKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoorSet {
    pub north: DoorKind,
    pub south: DoorKind,
    pub east: DoorKind,
    pub west: DoorKind,
}

impl DoorSet {
    pub fn all_open() -> Self {
        Self {
            north: super::door_kind::DoorKind::Open,
            south: DoorKind::Open,
            east: DoorKind::Open,
            west: DoorKind::Open,
        }
    }

    /// Decode from AttrsA (S/N) and AttrsB (E/W) raw bytes.
    ///
    /// AttrsA layout: bits 7-5 = North door (3-bit), bits 4-2 = South door.
    /// AttrsB layout: bits 7-5 = West door, bits 4-2 = East door (3-bit each).
    pub fn from_attrs(attrs_a: u8, attrs_b: u8) -> Self {
        Self {
            north: DoorKind::from_bits(attrs_a >> 5),
            south: DoorKind::from_bits((attrs_a >> 2) & 0x07),
            west: DoorKind::from_bits(attrs_b >> 5),
            east: DoorKind::from_bits((attrs_b >> 2) & 0x07),
        }
    }

    /// Pack these doors back into the two attribute bytes.
    pub fn encode_into(&self, attrs_a: &mut u8, attrs_b: &mut u8) {
        // Clear old door bits
        *attrs_a &= 0x03; // Keep bits 1-0
        *attrs_b &= 0x03;

        // Apply new door bits
        *attrs_a |= (self.north.to_bits() << 5) | (self.south.to_bits() << 2);
        *attrs_b |= (self.west.to_bits() << 5) | (self.east.to_bits() << 2);
    }

    /// True if any door is notable (not Open or Wall).
    pub fn has_notable(&self) -> bool {
        let notable = |d: DoorKind| !matches!(d, DoorKind::Open | DoorKind::Wall);
        notable(self.north) || notable(self.south) || notable(self.east) || notable(self.west)
    }
}
