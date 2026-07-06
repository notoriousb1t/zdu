use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct GameState {
    pub items: HashMap<u16, u8>,
    pub change_number: u64,
}

#[derive(Debug, Clone)]
pub struct BroadcastUpdate {
    pub sender_id: u32,
    pub new_change_number: u64,
    pub updates: Vec<(u16, u8)>,
}
