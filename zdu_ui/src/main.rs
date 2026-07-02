use iced::widget::{button, column, text, Column};
use iced::{Alignment, Theme};

pub fn main() -> iced::Result {
    iced::application(State::default, State::update, State::view)
        .title("Zelda: Dungeons Unseen (ZDU)")
        .theme(theme)
        .run()
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}

#[derive(Default)]
struct State {
    click_count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonClicked,
}

impl State {
    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonClicked => {
                self.click_count += 1;
            }
        }
    }

    fn view(&self) -> Column<'_, Message> {
        column![
            text("Zelda: Dungeons Unseen (ZDU)").size(32),
            text(format!("Button clicked {} times", self.click_count)).size(20),
            button("Interact").on_press(Message::ButtonClicked),
        ]
        .spacing(20)
        .align_x(Alignment::Center)
    }
}
