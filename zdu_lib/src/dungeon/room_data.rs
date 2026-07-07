// ---------------------------------------------------------------------------
// Per-room data.
// ---------------------------------------------------------------------------

use crate::dungeon::{door_set::DoorSet, room_item::RoomItem, room_trigger::RoomTrigger};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone)]
pub struct RoomData {
    pub room_id: u8,
    pub item: RoomItem,
    pub trigger: RoomTrigger,
    pub doors: DoorSet,
}

impl RoomData {
    /// True if this room has anything worth showing in the spoiler log.
    pub fn is_notable(&self) -> bool {
        !matches!(self.item, RoomItem::None)
            || !matches!(self.trigger, RoomTrigger::None)
            || self.doors.has_notable()
    }

    pub fn format_row(&self) -> String {
        let row = (self.room_id >> 4) as usize;
        let col = (self.room_id & 0x0F) as usize;

        let col_char = (b'A' + (col % 8) as u8) as char;
        let row_char = (b'1' + row as u8) as char;

        format!(
            "| {}{} / ${:02X} | {} | {} | {} | {} | {} | {} |",
            col_char,
            row_char,
            self.room_id,
            self.item,
            &self.trigger,
            self.doors.north.label(),
            self.doors.south.label(),
            self.doors.east.label(),
            self.doors.west.label(),
        )
    }
}

impl Display for RoomData {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "| Room ${:02X} | {} | {} | {} | {} | {} | {} |",
            self.room_id,
            self.item,
            &self.trigger,
            self.doors.north.label(),
            self.doors.south.label(),
            self.doors.east.label(),
            self.doors.west.label(),
        )
    }
}
