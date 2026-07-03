use std::fs;
use std::path::{Path, PathBuf};
use std::io;
use bsdiff::diff;

use crate::options::{StartInventoryOptions, ProgressionOptions};
use crate::locations::Location;

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
        patcher.apply(&old, std::io::Cursor::new(&mut buffer))
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
        patcher.apply(&old, std::io::Cursor::new(&mut buffer))
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
        
        Ok(Self {
            base_rom_path: base_rom_path.to_path_buf(),
            buffer,
        })
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
}
