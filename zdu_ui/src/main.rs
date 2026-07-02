mod app;
mod patcher;
mod server;
mod views;

use iced::Theme;
use app::State;

pub fn main() -> iced::Result {
    iced::application(State::new, State::update, State::view)
        .title("Zelda: Dungeons Unseen (ZDU)")
        .theme(theme)
        .subscription(State::subscription)
        .run()
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}
