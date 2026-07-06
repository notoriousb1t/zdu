use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, column, container, responsive, row, rule, scrollable, text, Column, Row, Space,
};
use iced::{Alignment, Element, Length};
use std::collections::HashMap;

pub enum SessionState {
    Closed,
    Open { port: u16, clients: usize },
}

pub fn view<'a, Message: Clone + 'a>(
    session: &SessionState,
    items: &HashMap<u16, u8>,
    toggle_msg: Message,
) -> Column<'a, Message> {
    let header_button = match session {
        SessionState::Closed => button("Open Session").on_press(toggle_msg),
        SessionState::Open { .. } => button("Close Session").on_press(toggle_msg),
    };

    let status_text = match session {
        SessionState::Closed => text(" - ❌ Server Status: Session Closed").size(18),
        SessionState::Open { port, clients } => text(format!(
            " - ✔️ Server Status: Running on Port {} ({} clients)",
            port, clients
        ))
        .size(18),
    };

    let header_row = row![
        text("Host Multiplayer").size(32),
        status_text,
        Space::new().width(Length::Fill),
        header_button,
    ]
    .align_y(Alignment::Center);

    let scrollable_content =
        scrollable(column![inventory_view(items), progression_view(items),].spacing(40))
            .width(Length::Fill)
            .height(Length::Fill);

    column![header_row, rule::horizontal(2), scrollable_content,]
        .spacing(20)
        .align_x(Alignment::Start)
}

fn build_item_row<'a, Message: 'a>(label_text: &'a str, value: String) -> Row<'a, Message> {
    row![
        container(text(label_text))
            .width(140)
            .align_x(Horizontal::Left),
        container(text(value)).width(150).align_x(Horizontal::Right),
    ]
    .align_y(Alignment::Center)
}

fn build_all_items<'a, Message: 'a>(items: &HashMap<u16, u8>) -> Vec<Element<'a, Message>> {
    let get_val = |offset: u16| -> u8 { items.get(&offset).copied().unwrap_or(0) };

    let get_bool = |offset: u16| -> String {
        if get_val(offset) > 0 {
            "Yes".to_string()
        } else {
            "No".to_string()
        }
    };

    vec![
        build_item_row(
            "Arrow:",
            match get_val(0x02) {
                1 => "Wood".to_string(),
                2 => "Silver".to_string(),
                _ => "None".to_string(),
            },
        )
        .into(),
        build_item_row("Bombs (Max):", format!("{}", get_val(0x25))).into(),
        build_item_row("Bombs (Start):", format!("{}", get_val(0x01))).into(),
        build_item_row("Book of Magic:", get_bool(0x0A)).into(),
        build_item_row(
            "Boomerang:",
            if get_val(0x1E) > 0 {
                "Magical".to_string()
            } else if get_val(0x1D) > 0 {
                "Wood".to_string()
            } else {
                "None".to_string()
            },
        )
        .into(),
        build_item_row("Bow:", get_bool(0x03)).into(),
        build_item_row(
            "Candle:",
            match get_val(0x04) {
                1 => "Blue".to_string(),
                2 => "Red".to_string(),
                _ => "None".to_string(),
            },
        )
        .into(),
        build_item_row("Food:", get_bool(0x06)).into(),
        build_item_row(
            "Heart Containers:",
            format!("{}", (get_val(0x18) & 0x0F) + 1),
        )
        .into(),
        build_item_row("Keys:", format!("{}", get_val(0x17))).into(),
        build_item_row("Ladder:", get_bool(0x0C)).into(),
        build_item_row("Letter:", get_bool(0x0F)).into(),
        build_item_row("Magic Key:", get_bool(0x0D)).into(),
        build_item_row("Magic Rod:", get_bool(0x08)).into(),
        build_item_row(
            "Potion:",
            match get_val(0x07) {
                1 => "Blue".to_string(),
                2 => "Red".to_string(),
                _ => "None".to_string(),
            },
        )
        .into(),
        build_item_row("Power Bracelet:", get_bool(0x0E)).into(),
        build_item_row("Raft:", get_bool(0x09)).into(),
        build_item_row("Recorder:", get_bool(0x05)).into(),
        build_item_row(
            "Ring:",
            match get_val(0x0B) {
                1 => "Blue".to_string(),
                2 => "Red".to_string(),
                _ => "None".to_string(),
            },
        )
        .into(),
        build_item_row("Rupees:", format!("{}", get_val(0x16))).into(),
        build_item_row(
            "Shield:",
            match get_val(0x1F) {
                1 => "Magical Shield".to_string(),
                _ => "Small Shield".to_string(),
            },
        )
        .into(),
        build_item_row(
            "Sword:",
            match get_val(0x00) {
                1 => "Wood".to_string(),
                2 => "White".to_string(),
                3 => "Magical".to_string(),
                7 => "Fairy".to_string(),
                _ => "None".to_string(),
            },
        )
        .into(),
    ]
}

fn inventory_view<'a, Message: 'a>(items: &HashMap<u16, u8>) -> Element<'a, Message> {
    let items_map = items.clone();
    responsive(move |size| {
        let cols = if size.width > 900.0 {
            3
        } else if size.width > 600.0 {
            2
        } else {
            1
        };

        let all_items_vec = build_all_items(&items_map);
        let items_per_col = (all_items_vec.len() + cols - 1) / cols;
        let mut all_items = all_items_vec.into_iter();

        let mut row_layout = row![].spacing(40);
        for _ in 0..cols {
            let mut col_layout = column![].spacing(8);
            for _ in 0..items_per_col {
                if let Some(item) = all_items.next() {
                    col_layout = col_layout.push(item);
                }
            }
            row_layout = row_layout.push(col_layout);
        }

        column![text("Inventory").size(24), rule::horizontal(2), row_layout]
            .spacing(8)
            .into()
    })
    .into()
}

fn progression_view<'a, Message: 'a>(items: &HashMap<u16, u8>) -> Column<'a, Message> {
    let get_val = |offset: u16| -> u8 { items.get(&offset).copied().unwrap_or(0) };

    let compass_mask = (get_val(0x10) as u16) | ((get_val(0x12) as u16) << 8);
    let map_mask = (get_val(0x11) as u16) | ((get_val(0x13) as u16) << 8);
    let triforce_mask = get_val(0x1A) as u16;
    let boss_mask = get_val(0x1B) as u16;

    let mut table_headers = row![container(text("Level"))
        .width(100)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Center)]
    .spacing(4)
    .align_y(Alignment::Center);

    for i in 1..=9 {
        table_headers = table_headers.push(
            container(text(format!("{}", i)))
                .width(40)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
        );
    }

    let build_row = |label: &'static str, mask: u16, is_triforce: bool| -> Row<'a, Message> {
        let mut r = row![container(text(label))
            .width(100)
            .align_x(Horizontal::Left)
            .align_y(Vertical::Center)]
        .spacing(4)
        .align_y(Alignment::Center);

        for i in 0..9 {
            if is_triforce && i == 8 {
                r = r.push(
                    container(text("-"))
                        .width(40)
                        .align_x(Horizontal::Center)
                        .align_y(Vertical::Center),
                );
                continue;
            }
            let is_checked = (mask & (1 << i)) != 0;
            let check_text = if is_checked { "✔️" } else { " " };
            r = r.push(
                container(text(check_text))
                    .width(40)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            );
        }
        r
    };

    let boss_row = build_row("Boss", boss_mask, false);
    let compass_row = build_row("Compass", compass_mask, false);
    let map_row = build_row("Map", map_mask, false);
    let triforce_row = build_row("Triforce", triforce_mask, true);

    column![
        text("Progression").size(24),
        rule::horizontal(2),
        table_headers,
        boss_row,
        compass_row,
        map_row,
        triforce_row,
    ]
    .spacing(8)
}
