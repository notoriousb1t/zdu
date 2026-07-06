use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, checkbox, column, container, pick_list, responsive, row, rule, scrollable, text,
    text_input, Column, Row, Space,
};
use iced::{Alignment, Element, Length};

use crate::app::{Message, State};

pub fn view<'a>(state: &'a State) -> Column<'a, Message> {
    let seed_section = row![
        text_input("Seed", &state.seed_input)
            .on_input(Message::SeedInputChanged)
            .width(250),
        button("Randomize Seed").on_press(Message::RandomizeSeed),
        Space::new().width(20),
        text("Port:"),
        text_input("42069", &state.port_input)
            .on_input(Message::PortInputChanged)
            .width(80),
    ]
    .spacing(8)
    .align_y(Alignment::Center);

    let items_view = responsive(move |size| {
        let cols = if size.width > 900.0 {
            3
        } else if size.width > 600.0 {
            2
        } else {
            1
        };

        let all_items_vec = build_all_items(state);
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

        column![
            text("Starting Inventory").size(20),
            rule::horizontal(2),
            row_layout
        ]
        .spacing(8)
        .into()
    });

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
    table_headers = table_headers.push(
        container(text("All"))
            .width(50)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    );

    let boss_row = build_progression_row(
        "Boss",
        &state.bosses_defeated,
        |idx, val| Message::ToggleBossDefeated(idx, val),
        Message::ToggleAllBossesDefeated,
        false,
    );
    let compass_row = build_progression_row(
        "Compass",
        &state.compasses,
        |idx, val| Message::ToggleCompass(idx, val),
        Message::ToggleAllCompass,
        false,
    );
    let map_row = build_progression_row(
        "Map",
        &state.maps,
        |idx, val| Message::ToggleMap(idx, val),
        Message::ToggleAllMap,
        false,
    );
    let triforce_row = build_progression_row(
        "Triforce",
        &state.triforce_pieces,
        |idx, val| Message::ToggleTriforce(idx, val),
        Message::ToggleAllTriforce,
        true,
    );

    let progression_section = column![
        text("Starting Progression").size(20),
        rule::horizontal(2),
        table_headers,
        boss_row,
        compass_row,
        map_row,
        triforce_row,
    ]
    .spacing(8);

    let scrollable_content = scrollable(column![items_view, progression_section,].spacing(40))
        .width(Length::Fill)
        .height(Length::Fill);

    let header_row = row![
        text("Create Game").size(32),
        Space::new().width(Length::Fill),
        button("Create").on_press(Message::GenerateRom)
    ]
    .align_y(Alignment::Center);

    column![
        header_row,
        rule::horizontal(2),
        seed_section,
        scrollable_content,
    ]
    .spacing(20)
    .padding(4)
}

fn build_progression_row<'a>(
    label: &'a str,
    bits: &[bool; 9],
    on_toggle_bit: fn(usize, bool) -> Message,
    on_toggle_all: fn(bool) -> Message,
    is_triforce: bool,
) -> Row<'a, Message> {
    let mut row = row![container(text(label))
        .width(100)
        .align_x(Horizontal::Left)
        .align_y(Vertical::Center)]
    .spacing(4)
    .align_y(Alignment::Center);
    let mut all_checked = true;
    for i in 0..9 {
        if is_triforce && i == 8 {
            row = row.push(
                container(text("-"))
                    .width(40)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            );
            continue;
        }
        let val = bits[i];
        if !val {
            all_checked = false;
        }
        row = row.push(
            container(
                checkbox(val)
                    .label("")
                    .on_toggle(move |v| on_toggle_bit(i, v)),
            )
            .width(40)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
        );
    }
    row.push(
        container(checkbox(all_checked).label("").on_toggle(on_toggle_all))
            .width(50)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
    )
}

fn build_item_row<'a>(
    label_text: &'a str,
    control: impl Into<Element<'a, Message>>,
) -> Row<'a, Message> {
    row![
        container(text(label_text))
            .width(140)
            .align_x(Horizontal::Left),
        container(control).width(150).align_x(Horizontal::Right),
    ]
    .align_y(Alignment::Center)
}

fn build_all_items<'a>(state: &'a State) -> Vec<Element<'a, Message>> {
    vec![
        build_item_row(
            "Arrow:",
            pick_list(
                vec!["None", "Wood", "Silver"],
                match state.start_arrow {
                    1 => Some("Wood"),
                    2 => Some("Silver"),
                    _ => Some("None"),
                },
                |s| {
                    Message::SetStartArrow(match s {
                        "Wood" => 1,
                        "Silver" => 2,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
        build_item_row(
            "Bombs (Max):",
            text_input("8", &state.max_bombs_input)
                .on_input(Message::SetMaxBombsInput)
                .align_x(Horizontal::Right)
                .width(50),
        )
        .into(),
        build_item_row(
            "Bombs (Start):",
            text_input("0", &state.start_bombs_input)
                .on_input(Message::SetStartBombsInput)
                .align_x(Horizontal::Right)
                .width(50),
        )
        .into(),
        build_item_row(
            "Book of Magic:",
            checkbox(state.start_book)
                .label("")
                .on_toggle(Message::ToggleStartBook),
        )
        .into(),
        build_item_row(
            "Boomerang:",
            pick_list(
                vec!["None", "Wood", "Magical"],
                match state.start_boomerang {
                    1 => Some("Wood"),
                    2 => Some("Magical"),
                    _ => Some("None"),
                },
                |s| {
                    Message::SetStartBoomerang(match s {
                        "Wood" => 1,
                        "Magical" => 2,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
        build_item_row(
            "Bow:",
            checkbox(state.start_bow)
                .label("")
                .on_toggle(Message::ToggleStartBow),
        )
        .into(),
        build_item_row(
            "Candle:",
            pick_list(
                vec!["None", "Blue", "Red"],
                match state.start_candle {
                    1 => Some("Blue"),
                    2 => Some("Red"),
                    _ => Some("None"),
                },
                |s| {
                    Message::SetStartCandle(match s {
                        "Blue" => 1,
                        "Red" => 2,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
        build_item_row(
            "Food:",
            checkbox(state.start_food)
                .label("")
                .on_toggle(Message::ToggleStartFood),
        )
        .into(),
        build_item_row(
            "Heart Containers:",
            pick_list(
                (3..=16).collect::<Vec<u8>>(),
                Some(state.heart_containers),
                Message::SetHeartContainers,
            ),
        )
        .into(),
        build_item_row(
            "Keys:",
            text_input("0", &state.start_keys_input)
                .on_input(Message::SetStartKeysInput)
                .align_x(Horizontal::Right)
                .width(50),
        )
        .into(),
        build_item_row(
            "Ladder:",
            checkbox(state.start_ladder)
                .label("")
                .on_toggle(Message::ToggleStartLadder),
        )
        .into(),
        build_item_row(
            "Letter:",
            checkbox(state.start_letter)
                .label("")
                .on_toggle(Message::ToggleStartLetter),
        )
        .into(),
        build_item_row(
            "Magic Key:",
            checkbox(state.start_magic_key)
                .label("")
                .on_toggle(Message::ToggleStartMagicKey),
        )
        .into(),
        build_item_row(
            "Magic Rod:",
            checkbox(state.start_magic_rod)
                .label("")
                .on_toggle(Message::ToggleStartMagicRod),
        )
        .into(),
        build_item_row(
            "Potion:",
            pick_list(
                vec!["None", "Blue", "Red"],
                match state.start_potion {
                    1 => Some("Blue"),
                    2 => Some("Red"),
                    _ => Some("None"),
                },
                |s| {
                    Message::SetStartPotion(match s {
                        "Blue" => 1,
                        "Red" => 2,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
        build_item_row(
            "Power Bracelet:",
            checkbox(state.start_bracelet)
                .label("")
                .on_toggle(Message::ToggleStartBracelet),
        )
        .into(),
        build_item_row(
            "Raft:",
            checkbox(state.start_raft)
                .label("")
                .on_toggle(Message::ToggleStartRaft),
        )
        .into(),
        build_item_row(
            "Recorder:",
            checkbox(state.start_recorder)
                .label("")
                .on_toggle(Message::ToggleStartRecorder),
        )
        .into(),
        build_item_row(
            "Ring:",
            pick_list(
                vec!["None", "Blue", "Red"],
                match state.start_ring {
                    1 => Some("Blue"),
                    2 => Some("Red"),
                    _ => Some("None"),
                },
                |s| {
                    Message::SetStartRing(match s {
                        "Blue" => 1,
                        "Red" => 2,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
        build_item_row(
            "Rupees:",
            text_input("0", &state.start_rupees_input)
                .on_input(Message::SetStartRupeesInput)
                .align_x(Horizontal::Right)
                .width(50),
        )
        .into(),
        build_item_row(
            "Shield:",
            pick_list(
                vec!["Small Shield", "Magical Shield"],
                match state.start_magic_shield {
                    1 => Some("Magical Shield"),
                    _ => Some("Small Shield"),
                },
                |s| {
                    Message::SetStartMagicShield(match s {
                        "Magical Shield" => 1,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
        build_item_row(
            "Sword:",
            pick_list(
                vec!["None", "Wood", "White", "Magical", "Fairy"],
                match state.start_sword {
                    1 => Some("Wood"),
                    2 => Some("White"),
                    3 => Some("Magical"),
                    7 => Some("Fairy"),
                    _ => Some("None"),
                },
                |s| {
                    Message::SetStartSword(match s {
                        "Wood" => 1,
                        "White" => 2,
                        "Magical" => 3,
                        "Fairy" => 7,
                        _ => 0,
                    })
                },
            ),
        )
        .into(),
    ]
}
