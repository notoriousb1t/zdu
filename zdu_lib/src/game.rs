use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::cave::cave_levels::CaveLevels;
use crate::cave::shop::Shop;
use crate::dungeon::door_set::DoorSet;
use crate::dungeon::dungeon_levels::{DungeonLevel, DungeonLevels};
use crate::dungeon::room_data::RoomData;
use crate::dungeon::room_item::RoomItem;
use crate::dungeon::room_trigger::RoomTrigger;
use crate::locations::Location;
use crate::options::{ProgressionOptions, StartInventoryOptions};
use crate::rom_ext;

pub struct Game {
    base_rom_path: PathBuf,
    buffer: Vec<u8>,
}

impl Game {
    pub fn new(base_rom_path: &Path) -> io::Result<Self> {
        let old = fs::read(base_rom_path)?;
        let patch_data = include_bytes!("../base_patch.bsdiff4");

        let patcher = qbsdiff::Bspatch::new(patch_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Bspatch requires us to provide a writer, and allocating with exact capacity is best
        let mut buffer = Vec::with_capacity(old.len() + 1024);
        patcher
            .apply(&old, std::io::Cursor::new(&mut buffer))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(Self {
            base_rom_path: base_rom_path.to_path_buf(),
            buffer,
        })
    }

    pub fn create_from_patch(base_rom_path: &Path, patch_path: &Path) -> io::Result<Self> {
        let old = fs::read(base_rom_path)?;
        let patch_data = fs::read(patch_path)?;

        let patcher = qbsdiff::Bspatch::new(&patch_data)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        let mut buffer = Vec::with_capacity(old.len() + 1024);
        patcher
            .apply(&old, std::io::Cursor::new(&mut buffer))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(Self {
            base_rom_path: base_rom_path.to_path_buf(),
            buffer,
        })
    }

    /// Build a `Game` from an already-patched ROM buffer.
    ///
    /// Useful for reading an existing `.nes` file directly without needing
    /// the original base ROM and bsdiff patch.
    pub fn from_buffer(path: &Path, buffer: Vec<u8>) -> Self {
        Self {
            base_rom_path: path.to_path_buf(),
            buffer,
        }
    }

    pub fn set_starting_inventory(&mut self, options: &StartInventoryOptions) {
        let offset = Location::StartingInventoryData as usize;
        if self.buffer.len() < offset + 0x26 {
            return; // Safety bounds check for 38 bytes
        }

        self.buffer[offset + 0x00] = options.start_sword;
        self.buffer[offset + 0x01] = options.start_bombs;
        self.buffer[offset + 0x02] = options.start_arrow;
        self.buffer[offset + 0x03] = if options.start_bow { 1 } else { 0 };
        self.buffer[offset + 0x04] = options.start_candle;
        self.buffer[offset + 0x05] = if options.start_recorder { 1 } else { 0 };
        self.buffer[offset + 0x06] = if options.start_food { 1 } else { 0 };
        self.buffer[offset + 0x07] = options.start_potion;
        self.buffer[offset + 0x08] = if options.start_magic_rod { 1 } else { 0 };
        self.buffer[offset + 0x09] = if options.start_raft { 1 } else { 0 };
        self.buffer[offset + 0x0A] = if options.start_book { 1 } else { 0 };
        self.buffer[offset + 0x0B] = options.start_ring;
        self.buffer[offset + 0x0C] = if options.start_ladder { 1 } else { 0 };
        self.buffer[offset + 0x0D] = if options.start_magic_key { 1 } else { 0 };
        self.buffer[offset + 0x0E] = if options.start_bracelet { 1 } else { 0 };
        self.buffer[offset + 0x0F] = if options.start_letter { 1 } else { 0 };

        self.buffer[offset + 0x16] = options.start_rupees;
        self.buffer[offset + 0x17] = options.start_keys;

        // Heart containers logic (3 containers = $22)
        let hc = options.heart_containers.saturating_sub(1).min(15);
        self.buffer[offset + 0x18] = (hc << 4) | hc;
        self.buffer[offset + 0x19] = 0xFF; // HeartPartial

        // Boomerangs
        self.buffer[offset + 0x1D] = if options.start_boomerang == 1 { 1 } else { 0 };
        self.buffer[offset + 0x1E] = if options.start_boomerang == 2 { 1 } else { 0 };

        self.buffer[offset + 0x1F] = options.start_magic_shield;
        self.buffer[offset + 0x25] = options.max_bombs;
    }

    pub fn set_progression(&mut self, options: &ProgressionOptions) {
        let offset = Location::StartingInventoryData as usize;
        if self.buffer.len() < offset + 0x26 {
            return;
        }

        let pack_bits = |bools: &[bool; 9]| -> (u8, u8) {
            let mut byte0 = 0u8;
            for i in 0..8 {
                if bools[i] {
                    byte0 |= 1 << i;
                }
            }
            let byte1 = if bools[8] { 1 } else { 0 };
            (byte0, byte1)
        };

        let (compass_b0, compass_b1) = pack_bits(&options.compasses);
        let (map_b0, map_b1) = pack_bits(&options.maps);
        let (triforce_b0, _) = pack_bits(&options.triforce_pieces);
        let (bosses_b0, _) = pack_bits(&options.bosses_defeated);

        self.buffer[offset + 0x10] = compass_b0;
        self.buffer[offset + 0x11] = map_b0;
        self.buffer[offset + 0x12] = compass_b1;
        self.buffer[offset + 0x13] = map_b1;

        self.buffer[offset + 0x1A] = triforce_b0;
        self.buffer[offset + 0x1B] = bosses_b0;
    }

    pub fn write(&self, output_path: &Path) -> io::Result<()> {
        fs::write(output_path, &self.buffer)?;
        Ok(())
    }

    pub fn write_patch(&self, patch_output_path: &Path) -> io::Result<()> {
        let old = fs::read(&self.base_rom_path)?;
        let mut patch_file = fs::File::create(patch_output_path)?;

        bsdiff::diff(&old, &self.buffer, &mut patch_file)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(())
    }

    pub fn read_dungeon_data(&self) -> DungeonLevels {
        let dungeons = std::array::from_fn(|i| self.read_dungeon((i + 1) as u8));
        DungeonLevels { levels: dungeons }
    }

    pub fn read_cave_data(&self) -> CaveLevels {
        let shop_keys = self.read_shop_keys();
        CaveLevels { shop_keys }
    }

    fn read_dungeon(&self, dungeon: u8) -> DungeonLevel {
        let block_base = if dungeon <= 6 {
            rom_ext::uw1_block_base()
        } else {
            rom_ext::uw2_block_base()
        };

        let info =
            rom_ext::level_info_rooms(&self.buffer, dungeon).unwrap_or(rom_ext::LevelInfoRooms {
                start_room: 0x73,
                triforce_room: 0xFF,
                boss_room: 0xFF,
                cellar_rooms: [0xFF; 10],
                map_mask: [0; 16],
                map_rotation: 0,
            });

        let room_ids = rom_ext::bfs_dungeon_rooms_full(&self.buffer, block_base, &info);

        let rooms = room_ids
            .into_iter()
            .map(|room_id| {
                let raw_a = self
                    .buffer
                    .get(rom_ext::attrs_a_offset(block_base, room_id))
                    .copied()
                    .unwrap_or(0);
                let raw_b = self
                    .buffer
                    .get(rom_ext::attrs_b_offset(block_base, room_id))
                    .copied()
                    .unwrap_or(0);
                let raw_e = self
                    .buffer
                    .get(rom_ext::attrs_e_offset(block_base, room_id))
                    .copied()
                    .unwrap_or(0x03);
                let raw_f = self
                    .buffer
                    .get(rom_ext::attrs_f_offset(block_base, room_id))
                    .copied()
                    .unwrap_or(0);

                RoomData {
                    room_id,
                    item: RoomItem::from_id(raw_e & 0x1F),
                    trigger: RoomTrigger::from_bits(raw_f & 0x07),
                    doors: DoorSet::from_attrs(raw_a, raw_b),
                }
            })
            .collect();

        let mut cellar_entrances = Vec::new();
        let mut item_cellar_entrances = Vec::new();
        for &cr in &info.cellar_rooms {
            if cr != 0xFF {
                let raw_e = self
                    .buffer
                    .get(rom_ext::attrs_e_offset(block_base, cr))
                    .copied()
                    .unwrap_or(0);
                let item = RoomItem::from_id(raw_e & 0x1F);
                let is_item_cellar = !matches!(item, RoomItem::None);

                let attrs_a = self
                    .buffer
                    .get(rom_ext::attrs_a_offset(block_base, cr))
                    .copied()
                    .unwrap_or(0xFF);
                let attrs_b = self
                    .buffer
                    .get(rom_ext::attrs_b_offset(block_base, cr))
                    .copied()
                    .unwrap_or(0xFF);

                let target_vec = if is_item_cellar {
                    &mut item_cellar_entrances
                } else {
                    &mut cellar_entrances
                };

                if attrs_a != 0xFF && attrs_a < 0x80 {
                    target_vec.push(attrs_a);
                }
                if attrs_b != 0xFF && attrs_b < 0x80 {
                    target_vec.push(attrs_b);
                }
            }
        }
        let mut cellar_rooms = Vec::new();
        for &cr in &info.cellar_rooms {
            if cr != 0xFF {
                cellar_rooms.push(cr);
            }
        }

        cellar_entrances.sort();
        cellar_entrances.dedup();
        item_cellar_entrances.sort();
        item_cellar_entrances.dedup();

        DungeonLevel {
            dungeon,
            start_room: info.start_room,
            rooms,
            map_mask: info.map_mask,
            map_rotation: info.map_rotation,
            cellar_entrances,
            item_cellar_entrances,
            cellar_rooms,
            triforce_room: info.triforce_room,
        }
    }

    fn read_shop_keys(&self) -> Vec<Shop> {
        let shop_item_offsets: &[(u8, u8, usize)] = &[
            (0, 0, Location::ShopPotionLeft as usize),
            (0, 1, Location::ShopPotionMiddle as usize),
            (0, 2, Location::ShopPotionRight as usize),
            (1, 0, Location::ShopArrowLeft as usize),
            (1, 1, Location::ShopArrowMiddle as usize),
            (1, 2, Location::ShopArrowRight as usize),
            (2, 0, Location::ShopCandleLeft as usize),
            (2, 1, Location::ShopCandleMiddle as usize),
            (2, 2, Location::ShopCandleRight as usize),
            (3, 0, Location::ShopShieldLeft as usize),
            (3, 1, Location::ShopShieldMiddle as usize),
            (3, 2, Location::ShopShieldRight as usize),
            (4, 0, Location::ShopRingLeft as usize),
            (4, 1, Location::ShopRingMiddle as usize),
            (4, 2, Location::ShopRingRight as usize),
        ];
        let price_offsets: &[usize] = &[
            Location::PricePotionLeft as usize,
            Location::PricePotionMiddle as usize,
            Location::PricePotionRight as usize,
            Location::PriceArrowLeft as usize,
            Location::PriceArrowMiddle as usize,
            Location::PriceArrowRight as usize,
            Location::PriceCandleLeft as usize,
            Location::PriceCandleMiddle as usize,
            Location::PriceCandleRight as usize,
            Location::PriceShieldLeft as usize,
            Location::PriceShieldMiddle as usize,
            Location::PriceShieldRight as usize,
            Location::PriceRingLeft as usize,
            Location::PriceRingMiddle as usize,
            Location::PriceRingRight as usize,
        ];

        let mut keys = Vec::new();
        for (i, &(shop_index, slot, item_off)) in shop_item_offsets.iter().enumerate() {
            let item_id = self.buffer.get(item_off).copied().unwrap_or(0) & 0x1F;
            if item_id == 0x19 {
                let price = self.buffer.get(price_offsets[i]).copied().unwrap_or(0);
                keys.push(Shop {
                    shop_index,
                    slot,
                    price,
                });
            }
        }
        keys
    }
}
