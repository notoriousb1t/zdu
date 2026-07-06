/// Internal ROM offset helpers.
///
/// `locations.rs` is generated and must not be edited directly.
/// All per-dungeon ROM offset arithmetic lives here instead.
use crate::locations::Location;

// Layout within each 768-byte LevelBlock
pub const ATTRS_A_OFFSET: usize = 0x000; // S/N door types  (128 bytes)
pub const ATTRS_B_OFFSET: usize = 0x080; // E/W door types  (128 bytes)
pub const ATTRS_C_OFFSET: usize = 0x100; // monster list    (128 bytes)
pub const ATTRS_D_OFFSET: usize = 0x180; // layout ID       (128 bytes)
pub const ATTRS_E_OFFSET: usize = 0x200; // room item ID    (128 bytes)
pub const ATTRS_F_OFFSET: usize = 0x280; // secret trigger  (128 bytes)

/// 128-byte arrays, one entry per room ID (0x00-0x7F), shared by ALL dungeons
/// in the same LevelBlock group. Dungeons 1-6 share LevelBlockUW1Q1; 7-9 share
/// LevelBlockUW2Q1.
pub const BLOCK_ARRAY_LEN: usize = 0x80;

/// ROM file offset for the Q1 UW1 LevelBlock (dungeons 1-6).
pub fn uw1_block_base() -> usize {
    Location::FirstQuestDungeonDataEarly as usize
}

/// ROM file offset for the Q1 UW2 LevelBlock (dungeons 7-9).
pub fn uw2_block_base() -> usize {
    Location::FirstQuestDungeonDataLate as usize
}

/// ROM offset for AttrsA[room_id] within the correct UW block.
pub fn attrs_a_offset(block_base: usize, room_id: u8) -> usize {
    block_base + ATTRS_A_OFFSET + room_id as usize
}

/// ROM offset for AttrsB[room_id].
pub fn attrs_b_offset(block_base: usize, room_id: u8) -> usize {
    block_base + ATTRS_B_OFFSET + room_id as usize
}

/// ROM offset for AttrsE[room_id] (item ID).
pub fn attrs_e_offset(block_base: usize, room_id: u8) -> usize {
    block_base + ATTRS_E_OFFSET + room_id as usize
}

/// ROM offset for AttrsF[room_id] (secret trigger).
pub fn attrs_f_offset(block_base: usize, room_id: u8) -> usize {
    block_base + ATTRS_F_OFFSET + room_id as usize
}

/// Determine which room IDs belong to a dungeon by BFS from its start room.
///
/// Room connectivity is derived from the door types stored in AttrsA/B:
/// - AttrsA bits 7-5 = South door type
/// - AttrsA bits 4-2 = North door type
/// - AttrsB bits 7-5 = East door type
/// - AttrsB bits 4-2 = West door type
/// Door kind 1 = Wall (impassable). All others allow traversal.
///
/// Room IDs encode grid position: row = id >> 4, col = id & 0x0F.
pub fn bfs_dungeon_rooms(buf: &[u8], block_base: usize, start_room: u8) -> Vec<u8> {
    let mut visited = [false; 0x80];
    let mut queue = std::collections::VecDeque::new();

    if (start_room as usize) < 0x80 {
        visited[start_room as usize] = true;
        queue.push_back(start_room);
    }

    while let Some(room) = queue.pop_front() {
        let row = room >> 4;
        let col = room & 0x0F;

        let attrs_a = buf.get(block_base + ATTRS_A_OFFSET + room as usize).copied().unwrap_or(0);
        let attrs_b = buf.get(block_base + ATTRS_B_OFFSET + room as usize).copied().unwrap_or(0);

        let north_kind = (attrs_a >> 5) & 0x07;  // bits 7-5
        let south_kind = (attrs_a >> 2) & 0x07;  // bits 4-2
        let west_kind  = (attrs_b >> 5) & 0x07;  // bits 7-5
        let east_kind  = (attrs_b >> 2) & 0x07;  // bits 4-2

        // Only traverse known passable door types.
        // 0=Open, 2=Locked, 3=PushBlock, 4=Bombable, 5=Shutter.
        // 1=Wall, 6 and 7 are not valid door types in Zelda 1 (palette-bit noise)
        // and the engine treats them as solid — exclude them from BFS traversal.
        let passable = |k: u8| matches!(k, 0 | 2 | 3 | 4 | 5);

        // South
        if passable(south_kind) && row < 0x07 {
            let n = room + 0x10;
            if (n as usize) < 0x80 && !visited[n as usize] {
                visited[n as usize] = true;
                queue.push_back(n);
            }
        }
        // North
        if passable(north_kind) && row > 0 {
            let n = room - 0x10;
            if !visited[n as usize] {
                visited[n as usize] = true;
                queue.push_back(n);
            }
        }
        // East
        if passable(east_kind) && col < 0x0F {
            let n = room + 1;
            if (n as usize) < 0x80 && !visited[n as usize] {
                visited[n as usize] = true;
                queue.push_back(n);
            }
        }
        // West
        if passable(west_kind) && col > 0 {
            let n = room - 1;
            if !visited[n as usize] {
                visited[n as usize] = true;
                queue.push_back(n);
            }
        }
    }

    (0u8..0x80)
        .filter(|&id| visited[id as usize])
        .collect()
}

/// ROM offset of the LevelInfoAddrsQ1 table.
/// LevelBlockAddrsQ1 is at Location::LevelAddresses (10 entries × 2 bytes).
/// LevelInfoAddrsQ1 immediately follows it.
pub fn level_info_addrs_q1_rom_offset() -> usize {
    Location::LevelAddresses as usize + 10 * 2
}

/// Resolve the ROM base address of the LevelInfo block for a dungeon (1-9).
fn level_info_rom_base(buf: &[u8], dungeon: u8) -> Option<usize> {
    assert!((1..=9).contains(&dungeon));
    let table_off = level_info_addrs_q1_rom_offset();
    let entry_off = table_off + dungeon as usize * 2;

    let lo = *buf.get(entry_off)? as usize;
    let hi = *buf.get(entry_off + 1)? as usize;
    let cpu_addr = (hi << 8) | lo;

    // Bank 6: CPU $8000-$BFFF → ROM offset 0x18010 + (cpu - $8000).
    const BANK6_ROM_BASE: usize = 0x18010;
    const BANK6_CPU_BASE: usize = 0x8000;
    if cpu_addr < BANK6_CPU_BASE { return None; }
    Some(BANK6_ROM_BASE + (cpu_addr - BANK6_CPU_BASE))
}

/// Key room IDs read directly from a dungeon's LevelInfo block.
pub struct LevelInfoRooms {
    pub start_room:   u8,
    pub triforce_room: u8,
    pub boss_room:    u8,
    /// Room IDs that contain staircases to hidden cellar rooms ($FF = unused slot).
    pub cellar_rooms: [u8; 10],
}

/// Extract all key room IDs from LevelInfo for a dungeon (1-9).
pub fn level_info_start_room(buf: &[u8], dungeon: u8) -> Option<u8> {
    let base = level_info_rom_base(buf, dungeon)?;
    buf.get(base + 0x2F).copied()
}

/// Extract the full set of key room IDs from LevelInfo.
pub fn level_info_rooms(buf: &[u8], dungeon: u8) -> Option<LevelInfoRooms> {
    let base = level_info_rom_base(buf, dungeon)?;

    let start_room    = *buf.get(base + 0x2F)?;
    let triforce_room = *buf.get(base + 0x30)?;
    let boss_room     = *buf.get(base + 0x3E)?;

    let mut cellar_rooms = [0xFFu8; 10];
    for (i, slot) in cellar_rooms.iter_mut().enumerate() {
        *slot = buf.get(base + 0x34 + i).copied().unwrap_or(0xFF);
    }

    Some(LevelInfoRooms { start_room, triforce_room, boss_room, cellar_rooms })
}

/// Determine which room IDs belong to a dungeon via BFS from all known seed rooms:
/// start room, triforce room, boss room, and any cellar room IDs listed in LevelInfo.
///
/// Door traversal rules:
/// - AttrsA bits 7-5 = South door, bits 2-0 = North door
/// - AttrsB bits 7-5 = East door,  bits 2-0 = West door
/// - Passable kinds: 0=Open, 2=Locked, 3=PushBlock, 4=Bombable, 5=Shutter
/// - 1=Wall and 6/7 (palette-bit noise) treated as impassable.
pub fn bfs_dungeon_rooms_full(buf: &[u8], block_base: usize, info: &LevelInfoRooms) -> Vec<u8> {
    let passable = |k: u8| matches!(k, 0 | 2 | 3 | 4 | 5);
    let mut visited = [false; 0x80];
    let mut queue = std::collections::VecDeque::new();

    let seed = |room: u8, visited: &mut [bool; 0x80], queue: &mut std::collections::VecDeque<u8>| {
        if (room as usize) < 0x80 && !visited[room as usize] {
            visited[room as usize] = true;
            queue.push_back(room);
        }
    };

    seed(info.start_room,    &mut visited, &mut queue);
    seed(info.triforce_room, &mut visited, &mut queue);
    seed(info.boss_room,     &mut visited, &mut queue);
    for &cr in &info.cellar_rooms {
        if cr != 0xFF { seed(cr, &mut visited, &mut queue); }
    }

    while let Some(room) = queue.pop_front() {
        let row = room >> 4;
        let col = room & 0x0F;
        let attrs_a = buf.get(block_base + ATTRS_A_OFFSET + room as usize).copied().unwrap_or(0);
        let attrs_b = buf.get(block_base + ATTRS_B_OFFSET + room as usize).copied().unwrap_or(0);

        let north_kind = (attrs_a >> 5) & 0x07;
        let south_kind = (attrs_a >> 2) & 0x07;
        let west_kind  = (attrs_b >> 5) & 0x07;
        let east_kind  = (attrs_b >> 2) & 0x07;

        if passable(south_kind) && row < 0x07 {
            let n = room + 0x10;
            if (n as usize) < 0x80 && !visited[n as usize] {
                visited[n as usize] = true; queue.push_back(n);
            }
        }
        if passable(north_kind) && row > 0 {
            let n = room - 0x10;
            if !visited[n as usize] { visited[n as usize] = true; queue.push_back(n); }
        }
        if passable(east_kind) && col < 0x0F {
            let n = room + 1;
            if (n as usize) < 0x80 && !visited[n as usize] {
                visited[n as usize] = true; queue.push_back(n);
            }
        }
        if passable(west_kind) && col > 0 {
            let n = room - 1;
            if !visited[n as usize] { visited[n as usize] = true; queue.push_back(n); }
        }
    }

    (0u8..0x80).filter(|&id| visited[id as usize]).collect()
}

