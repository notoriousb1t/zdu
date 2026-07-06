#[derive(Debug, Clone)]
pub struct Shop {
    pub shop_index: u8, // 0-based index into the shop slot arrays
    pub slot: u8,       // 0=left, 1=middle, 2=right
    pub price: u8,
}
