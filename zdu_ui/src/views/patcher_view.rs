use iced::widget::{column, text, Column};
use iced::Alignment;

pub fn view<'a, Message: 'a>() -> Column<'a, Message> {
    column![
        text("Patcher View").size(32),
        text("ROM patching functionality will go here.").size(20),
    ]
    .spacing(20)
    .align_x(Alignment::Center)
}
