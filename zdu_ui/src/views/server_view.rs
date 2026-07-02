use iced::widget::{button, column, text, Column};
use iced::Alignment;

pub enum SessionState {
    Closed,
    Open {
        port: u16,
        clients: usize,
    },
}

pub fn view<'a, Message: Clone + 'a>(
    session: &SessionState,
    log: &[String],
    items: &std::collections::HashMap<u8, u8>,
    toggle_msg: Message,
) -> Column<'a, Message> {
    let (status_text, button_text) = match session {
        SessionState::Closed => ("Session Closed", "Open Session"),
        SessionState::Open { port, clients } => {
            let s = format!("Session Open on port {} ({} clients)", port, clients);
            return column![
                text("Zelda: Dungeons Unseen (ZDU)").size(32),
                text(s).size(20),
                button("Close Session").on_press(toggle_msg),
                inventory_view(items),
                log_view(log),
            ]
            .spacing(20)
            .align_x(Alignment::Center);
        }
    };

    column![
        text("Zelda: Dungeons Unseen (ZDU)").size(32),
        text(status_text).size(20),
        button(button_text).on_press(toggle_msg),
        inventory_view(items),
        log_view(log),
    ]
    .spacing(20)
    .align_x(Alignment::Center)
}

fn log_view<'a, Message: 'a>(log: &[String]) -> Column<'a, Message> {
    let mut col = Column::new().spacing(5);
    for entry in log {
        col = col.push(text(entry.clone()).size(14));
    }
    col
}

fn inventory_view<'a, Message: 'a>(items: &std::collections::HashMap<u8, u8>) -> Column<'a, Message> {
    let get_val = |offset: u8| -> u8 { items.get(&offset).copied().unwrap_or(0) };
    
    let bombs = get_val(0x01);
    let bow = get_val(0x03) > 0;
    let arrows = get_val(0x02);
    let arrow_str = match arrows { 0 => "None", 1 => "Wood", _ => "Silver" };
    let keys = get_val(0x17);
    let triforce = get_val(0x1A);
    let triforce_count = triforce.count_ones();
    let compass = get_val(0x10);
    let map = get_val(0x11);
    
    let boomerang = get_val(0x1D) > 0 || get_val(0x1E) > 0;
    let magic_shield = get_val(0x1F) > 0;
    
    column![
        text("Shared Inventory").size(24),
        text(format!("Triforce Pieces: {}", triforce_count)),
        text(format!("Bombs: {}", bombs)),
        text(format!("Keys: {}", keys)),
        text(format!("Bow: {} (Arrows: {})", if bow { "Yes" } else { "No" }, arrow_str)),
        text(format!("Boomerang: {}", if boomerang { "Yes" } else { "No" })),
        text(format!("Magic Shield: {}", if magic_shield { "Yes" } else { "No" })),
        text(format!("Compass bits: {:08b}", compass)),
        text(format!("Map bits: {:08b}", map)),
    ].spacing(5)
}
