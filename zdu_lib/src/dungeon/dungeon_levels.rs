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
    pub map_mask: [u8; 16],
    pub map_rotation: u8,
    pub cellar_entrances: Vec<u8>,
    pub item_cellar_entrances: Vec<u8>,
    pub cellar_rooms: Vec<u8>,
    pub triforce_room: u8,
}

impl DungeonLevel {
    fn is_room_visible(&self, room_id: u8) -> bool {
        let col = (room_id & 0x0F) as usize;
        let row = (room_id >> 4) as usize;
        let mask_col = (col + self.map_rotation as usize) % 16;
        let mask_byte = self.map_mask.get(mask_col).copied().unwrap_or(0);
        (mask_byte & (1 << (7 - row))) != 0
    }
}

fn door_char(kind: DoorKind, vertical: bool, is_visible: bool) -> char {
    match kind {
        DoorKind::Open => ' ',
        DoorKind::Locked => 'Ω',
        DoorKind::PushBlock => 'p',
        DoorKind::Bombable => '~',
        DoorKind::Shutter => 'x',
        DoorKind::InvisibleWall => '░',
        DoorKind::Wall => {
            if is_visible {
                if vertical {
                    '║'
                } else {
                    '═'
                }
            } else {
                if vertical {
                    '│'
                } else {
                    '─'
                }
            }
        }
    }
}

fn center_char(room: &RoomData) -> char {
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

        let base_col = if (self.start_room & 0x0F) < 8 { 0 } else { 8 };

        writeln!(f, "```text")?;
        writeln!(f, "    A    B    C    D    E    F    G    H")?;
        for row in 0..8 {
            // Row 0 (Top)
            write!(f, "  ")?;
            for col_idx in 0..8 {
                let room_id = (row as u8) << 4 | (base_col + col_idx) as u8;
                if let Some(r) = self
                    .rooms
                    .iter()
                    .find(|x| x.room_id == room_id && !self.cellar_rooms.contains(&x.room_id))
                {
                    let is_visible = self.is_room_visible(r.room_id);
                    let (tl, tr, hw) = if is_visible {
                        ('╔', '╗', '═')
                    } else {
                        ('┌', '┐', '─')
                    };
                    write!(
                        f,
                        "{}{}{}{}{}",
                        tl,
                        hw,
                        door_char(r.doors.north, false, is_visible),
                        hw,
                        tr
                    )?;
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
            // Row 1 (Middle)
            write!(f, "{} ", row + 1)?;
            for col_idx in 0..8 {
                let room_id = (row as u8) << 4 | (base_col + col_idx) as u8;
                if let Some(r) = self
                    .rooms
                    .iter()
                    .find(|x| x.room_id == room_id && !self.cellar_rooms.contains(&x.room_id))
                {
                    let is_visible = self.is_room_visible(r.room_id);
                    let has_item_cellar = self.item_cellar_entrances.contains(&r.room_id);
                    let has_cellar = self.cellar_entrances.contains(&r.room_id);
                    let c = if self.dungeon == 9 && r.room_id == self.triforce_room {
                        'z'
                    } else if has_item_cellar {
                        'i'
                    } else if has_cellar {
                        's'
                    } else {
                        center_char(r)
                    };
                    write!(
                        f,
                        "{} {} {}",
                        door_char(r.doors.west, true, is_visible),
                        c,
                        door_char(r.doors.east, true, is_visible)
                    )?;
                } else {
                    write!(f, "     ")?;
                }
            }
            writeln!(f)?;
            // Row 2 (Bottom)
            write!(f, "  ")?;
            for col_idx in 0..8 {
                let room_id = (row as u8) << 4 | (base_col + col_idx) as u8;
                if let Some(r) = self
                    .rooms
                    .iter()
                    .find(|x| x.room_id == room_id && !self.cellar_rooms.contains(&x.room_id))
                {
                    let is_visible = self.is_room_visible(r.room_id);
                    let (bl, br, hw) = if is_visible {
                        ('╚', '╝', '═')
                    } else {
                        ('└', '┘', '─')
                    };
                    let mut s = door_char(r.doors.south, false, is_visible);
                    if r.room_id == self.start_room {
                        s = '⌂';
                    }
                    write!(f, "{}{}{}{}{}", bl, hw, s, hw, br)?;
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
            writeln!(f, "{}", room.format_row())?;
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
