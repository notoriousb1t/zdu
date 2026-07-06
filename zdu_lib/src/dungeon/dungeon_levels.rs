use crate::dungeon::{door_kind::DoorKind, room_data::RoomData, room_item::RoomItem};
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct DungeonLevels {
    pub levels: [DungeonLevel; 9],
}

#[derive(Debug, Clone)]
pub struct DungeonLevel {
    pub dungeon: u8,
    pub start_room: u8,
    pub rooms: Vec<RoomData>,
}

fn door_char(kind: DoorKind, vertical: bool) -> char {
    match kind {
        DoorKind::Open => ' ',
        DoorKind::Locked => 'Ω',
        DoorKind::PushBlock => 'p',
        DoorKind::Bombable => '~',
        DoorKind::Shutter | DoorKind::BossShutter => 'x',
        DoorKind::Wall | DoorKind::Unknown(_) => {
            if vertical {
                '│'
            } else {
                '─'
            }
        }
    }
}

fn center_char(room: &RoomData, start_room: u8) -> char {
    if room.room_id == start_room {
        return '⌂';
    }
    match room.item {
        RoomItem::HeartContainer => '♥',
        RoomItem::Triforce | RoomItem::TriforceOfPower => '▲',
        RoomItem::Compass => 'c',
        RoomItem::Map => 'm',
        RoomItem::Key | RoomItem::MagicKey => '⚷',
        RoomItem::Rupees | RoomItem::FiveRupees => '$',
        RoomItem::None => ' ',
        _ => 'i',
    }
}

impl Display for DungeonLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        if self.rooms.is_empty() {
            return writeln!(f, "No rooms found.");
        }

        let mut min_row = 7;
        let mut max_row = 0;
        let mut min_col = 15;
        let mut max_col = 0;

        for room in &self.rooms {
            let row = (room.room_id >> 4) as usize;
            let col = (room.room_id & 0x0F) as usize;
            if row < min_row {
                min_row = row;
            }
            if row > max_row {
                max_row = row;
            }
            if col < min_col {
                min_col = col;
            }
            if col > max_col {
                max_col = col;
            }
        }

        writeln!(f, "```text")?;
        for row in min_row..=max_row {
            // Row 0 (Top)
            for col in min_col..=max_col {
                if let Some(r) = self
                    .rooms
                    .iter()
                    .find(|x| x.room_id == ((row as u8) << 4 | (col as u8)))
                {
                    write!(f, "┌─{}─┐", door_char(r.doors.north, false))?;
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
            // Row 1 (Middle)
            for col in min_col..=max_col {
                if let Some(r) = self
                    .rooms
                    .iter()
                    .find(|x| x.room_id == ((row as u8) << 4 | (col as u8)))
                {
                    let c = center_char(r, self.start_room);
                    write!(
                        f,
                        "{} {} {}",
                        door_char(r.doors.west, true),
                        c,
                        door_char(r.doors.east, true)
                    )?;
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
            // Row 2 (Bottom)
            for col in min_col..=max_col {
                if let Some(r) = self
                    .rooms
                    .iter()
                    .find(|x| x.room_id == ((row as u8) << 4 | (col as u8)))
                {
                    write!(f, "└─{}─┘", door_char(r.doors.south, false))?;
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "```\n")?;

        writeln!(f, "| Room | Item | Trigger | N | S | E | W |")?;
        writeln!(f, "|------|------|---------|---|---|---|---|")?;
        for room in self.rooms.iter().filter(|r| r.is_notable()) {
            writeln!(f, "{}", room)?;
        }

        Ok(())
    }
}

impl Display for DungeonLevels {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for d in &self.levels {
            writeln!(f, "## Dungeon {}\n", d.dungeon)?;
            writeln!(f, "{}", d)?;
        }
        writeln!(f, "")
    }
}
