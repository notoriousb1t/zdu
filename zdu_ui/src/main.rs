mod app;
mod config;
mod server;
mod views;

use app::State;
use iced::{application, window, Result, Size, Theme};

pub fn main() -> Result {
    application(State::new, State::update, State::view)
        .title("Zelda: Dungeons Unseen (ZDU)")
        .window(window::Settings {
            min_size: Some(Size::new(800.0, 600.0)),
            max_size: Some(Size::new(1200.0, 4000.0)),
            ..Default::default()
        })
        .theme(theme)
        .subscription(State::subscription)
        .run()
}

fn theme(_state: &State) -> Theme {
    Theme::Dark
}
