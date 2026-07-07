mod app;
mod config;
mod server;
mod views;

use app::State;
use iced::{application, window, Result, Size, Theme};
use std::env;

pub fn main() -> Result {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--create") {
        create_headless();
        return Ok(());
    }

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

fn create_headless() {
    let base_path = match config::load_rom_path() {
        Some(path) => path,
        None => {
            eprintln!(
                "Error: Base ROM path not configured. Please run the UI and configure it first."
            );
            std::process::exit(1);
        }
    };

    let mut game = match zdu_lib::Game::new(&base_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to load base ROM: {}", e);
            std::process::exit(1);
        }
    };

    let inventory_options = zdu_lib::StartInventoryOptions::default();
    game.set_starting_inventory(&inventory_options);

    let progression_options = zdu_lib::ProgressionOptions::default();
    game.set_progression(&progression_options);

    let out_nes = std::path::Path::new("zdu_generated.nes");
    if let Err(e) = game.write(out_nes) {
        eprintln!("Failed to write generated ROM: {}", e);
        std::process::exit(1);
    }

    let out_bsdiff = std::path::Path::new("zdu_generated.bsdiff");
    if let Err(e) = game.write_patch(out_bsdiff) {
        eprintln!("Failed to write patch: {}", e);
        std::process::exit(1);
    }

    let dungeon_data = game.read_dungeon_data();
    let dungeon_spoiler = dungeon_data.to_string();
    let cave_data = game.read_cave_data();
    let cave_spoiler = cave_data.to_string();
    let spoiler = cave_spoiler + &dungeon_spoiler;
    let out_spoiler = std::path::Path::new("zdu_generated_spoiler.md");
    if let Err(e) = std::fs::write(out_spoiler, spoiler) {
        eprintln!("Failed to write spoiler log: {}", e);
        std::process::exit(1);
    }

    println!("Successfully generated zdu_generated.nes, zdu_generated.bsdiff, and zdu_generated_spoiler.md.");
}
