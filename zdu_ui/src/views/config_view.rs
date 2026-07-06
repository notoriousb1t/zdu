use iced::widget::{button, checkbox, column, row, rule, text, text_input, Column, Space};
use iced::{Alignment, Length};

pub fn view<'a, Message: Clone + 'a>(
    rom_path: &str,
    remember_settings: bool,
    on_change: impl Fn(String) -> Message + 'a,
    on_browse: Message,
    on_save: Message,
    on_toggle_remember: impl Fn(bool) -> Message + 'a,
) -> Column<'a, Message> {
    let header_row = row![
        text("Local Configuration").size(32),
        Space::new().width(Length::Fill),
        button("Save Configuration").on_press(on_save),
    ]
    .align_y(Alignment::Center);

    column![
        header_row,
        rule::horizontal(2),
        text("Please select a clean Legend of Zelda (USA) NES ROM file.").size(18),
        row![
            text_input("ROM Path", rom_path)
                .on_input(on_change)
                .width(400),
            button("Browse").on_press(on_browse),
        ]
        .spacing(8)
        .align_y(Alignment::Center),
        checkbox(remember_settings)
            .label("Remember Game Settings")
            .on_toggle(on_toggle_remember),
    ]
    .spacing(20)
    .align_x(Alignment::Start)
}
