use iced::widget::{button, column, row, rule, scrollable, text, Column, Space};
use iced::{Alignment, Length};

pub fn view<'a, Message: Clone + 'a>(log: &[String], on_clear: Message) -> Column<'a, Message> {
    let mut log_col = Column::new().spacing(4);
    for entry in log {
        log_col = log_col.push(text(entry.clone()).size(14));
    }

    let scrollable_log = scrollable(log_col)
        .width(Length::Fill)
        .height(Length::Fill);

    let header_row = row![
        text("Activity Log").size(32),
        Space::new().width(Length::Fill),
        button("Clear Log").on_press(on_clear),
    ]
    .align_y(Alignment::Center);

    column![
        header_row,
        rule::horizontal(2),
        scrollable_log,
    ]
    .spacing(20)
    .align_x(Alignment::Start)
    .width(Length::Fill)
    .height(Length::Fill)
}
