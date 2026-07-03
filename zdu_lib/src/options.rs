#[derive(Debug, Clone, Default)]
pub struct StartInventoryOptions {
    pub start_sword: u8,
    pub start_arrow: u8,
    pub start_bow: bool,
    pub start_candle: u8,
    pub start_ring: u8,
    pub start_magic_shield: u8,
    pub start_boomerang: u8,
    pub start_bombs: u8,
    pub max_bombs: u8,
    pub start_rupees: u8,
    pub start_keys: u8,
    pub heart_containers: u8,
    pub start_food: bool,
    pub start_potion: u8,
    pub start_recorder: bool,
    pub start_magic_rod: bool,
    pub start_raft: bool,
    pub start_book: bool,
    pub start_ladder: bool,
    pub start_magic_key: bool,
    pub start_bracelet: bool,
    pub start_letter: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ProgressionOptions {
    pub compasses: [bool; 9],
    pub maps: [bool; 9],
    pub triforce_pieces: [bool; 9],
    pub bosses_defeated: [bool; 9],
}
