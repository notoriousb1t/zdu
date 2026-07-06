use iced::futures::stream;
use iced::widget::{button, column, container, row, text, Column, Space};
use iced::{Length, Subscription, Task};

use crate::server::Server;
use crate::server::ServerMessage;
use crate::views::patcher_view;
use crate::views::server_view::{self, SessionState};
use std::collections::HashMap;
use std::path::PathBuf;
use std::thread;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSession,
    ServerMessage(ServerMessage),
    SwitchView(ViewMode),
    RomPathInputChanged(String),
    BrowseRomPath,
    RomPathSelected(Option<PathBuf>),
    SaveRomPath,
    ToggleRememberSettings(bool),
    ClearLog,
    SeedInputChanged(String),
    RandomizeSeed,
    GenerateRom,
    LogMessage(String),
    PortInputChanged(String),

    SetStartSword(u8),
    SetStartArrow(u8),
    ToggleStartBow(bool),
    SetStartCandle(u8),
    SetStartRing(u8),
    SetStartMagicShield(u8),
    SetStartBoomerang(u8),
    SetStartBombsInput(String),
    SetMaxBombsInput(String),

    SetStartRupeesInput(String),
    SetStartKeysInput(String),
    SetHeartContainers(u8),
    ToggleStartFood(bool),
    SetStartPotion(u8),

    ToggleStartRecorder(bool),
    ToggleStartMagicRod(bool),
    ToggleStartRaft(bool),
    ToggleStartBook(bool),
    ToggleStartLadder(bool),
    ToggleStartMagicKey(bool),
    ToggleStartBracelet(bool),
    ToggleStartLetter(bool),

    ToggleCompass(usize, bool),
    ToggleMap(usize, bool),
    ToggleTriforce(usize, bool),
    ToggleAllCompass(bool),
    ToggleAllMap(bool),
    ToggleAllTriforce(bool),
    ToggleBossDefeated(usize, bool),
    ToggleAllBossesDefeated(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    Patch,
    Host,
    Configure,
    Log,
}

enum SubState {
    Init,
    Ready(UnboundedReceiver<ServerMessage>),
}

pub struct State {
    pub session: SessionState,
    pub log: Vec<String>,
    pub items: HashMap<u16, u8>,
    pub view_mode: ViewMode,
    pub base_rom_path: Option<PathBuf>,
    pub rom_path_input: String,
    pub remember_settings: bool,
    pub seed_input: String,
    pub port_input: String,

    pub start_sword: u8,
    pub start_arrow: u8,
    pub start_bow: bool,
    pub start_candle: u8,
    pub start_ring: u8,
    pub start_magic_shield: u8,
    pub start_boomerang: u8,
    pub start_bombs_input: String,
    pub max_bombs_input: String,

    pub start_rupees_input: String,
    pub start_keys_input: String,
    pub heart_containers: u8,
    pub start_food: bool,
    pub start_potion: u8,

    pub start_recorder: bool,
    pub start_magic_rod: bool,
    pub start_raft: bool,
    pub start_book: bool,
    pub start_ladder: bool,
    pub start_magic_key: bool,
    pub start_bracelet: bool,
    pub start_letter: bool,

    pub compasses: [bool; 9],
    pub maps: [bool; 9],
    pub triforce_pieces: [bool; 9],
    pub bosses_defeated: [bool; 9],
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let loaded_path = crate::config::load_rom_path();
        let input_str = loaded_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let initial_seed: u64 = rand::random();
        let loaded_port = crate::config::load_port();
        (
            Self {
                session: SessionState::Closed,
                log: vec!["Ready.".to_string()],
                items: HashMap::new(),
                view_mode: if loaded_path.is_none() {
                    ViewMode::Configure
                } else {
                    ViewMode::Patch
                },
                base_rom_path: loaded_path,
                rom_path_input: input_str,
                remember_settings: true,
                seed_input: initial_seed.to_string(),
                port_input: loaded_port.to_string(),

                start_sword: 0,
                start_arrow: 0,
                start_bow: false,
                start_candle: 0,
                start_ring: 0,
                start_magic_shield: 0,
                start_boomerang: 0,
                start_bombs_input: "8".to_string(),
                max_bombs_input: "8".to_string(),

                start_rupees_input: "0".to_string(),
                start_keys_input: "0".to_string(),
                heart_containers: 3,
                start_food: false,
                start_potion: 0,

                start_recorder: false,
                start_magic_rod: false,
                start_raft: false,
                start_book: false,
                start_ladder: false,
                start_magic_key: false,
                start_bracelet: false,
                start_letter: false,

                compasses: [false; 9],
                maps: [false; 9],
                triforce_pieces: [false; 9],
                bosses_defeated: [false; 9],
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SwitchView(mode) => {
                self.view_mode = mode;
                Task::none()
            }
            Message::ToggleSession => {
                match &self.session {
                    SessionState::Closed => {
                        let port = self.port_input.parse::<u16>().unwrap_or(42069);
                        self.session = SessionState::Open { port, clients: 0 };
                        self.log.push(format!("Opened session on port {}", port));
                    }
                    SessionState::Open { .. } => {
                        self.session = SessionState::Closed;
                        self.log.push("Closed session (UI only).".to_string());
                    }
                }
                Task::none()
            }
            Message::ServerMessage(msg) => {
                match msg {
                    ServerMessage::ClientConnected(id) => {
                        if let SessionState::Open { clients, .. } = &mut self.session {
                            *clients += 1;
                        }
                        self.log.push(format!("Client {} connected.", id));
                    }
                    ServerMessage::ClientDisconnected(id) => {
                        if let SessionState::Open { clients, .. } = &mut self.session {
                            *clients = clients.saturating_sub(1);
                        }
                        self.log.push(format!("Client {} disconnected.", id));
                    }
                    ServerMessage::UpdateReceived {
                        client_id,
                        change_number,
                        updates,
                    } => {
                        for &(offset, val) in &updates {
                            self.items.insert(offset, val);
                        }

                        self.log.push(format!(
                            "Client {} updated state to change #{}. {} items updated.",
                            client_id,
                            change_number,
                            updates.len()
                        ));
                    }
                }

                if self.log.len() > 20 {
                    self.log.remove(0);
                }

                Task::none()
            }
            Message::RomPathInputChanged(path) => {
                self.rom_path_input = path;
                Task::none()
            }
            Message::BrowseRomPath => Task::perform(
                async {
                    rfd::FileDialog::new()
                        .add_filter("NES ROM", &["nes"])
                        .pick_file()
                },
                Message::RomPathSelected,
            ),
            Message::RomPathSelected(path_opt) => {
                if let Some(path) = path_opt {
                    self.rom_path_input = path.to_string_lossy().to_string();
                }
                Task::none()
            }
            Message::ToggleRememberSettings(val) => {
                self.remember_settings = val;
                Task::none()
            }
            Message::ClearLog => {
                self.log.clear();
                Task::none()
            }
            Message::SaveRomPath => {
                let path = PathBuf::from(self.rom_path_input.trim());
                if path.exists() && path.is_file() {
                    if let Ok(_) = crate::config::save_rom_path(&path) {
                        self.base_rom_path = Some(path);
                        self.log.push("ROM path saved successfully.".to_string());
                    } else {
                        self.log
                            .push("Failed to save ROM path to config.".to_string());
                    }
                } else {
                    self.log
                        .push("Invalid ROM path: file does not exist.".to_string());
                }
                Task::none()
            }
            Message::SeedInputChanged(seed) => {
                self.seed_input = seed;
                Task::none()
            }
            Message::RandomizeSeed => {
                let val: u64 = rand::random();
                self.seed_input = val.to_string();
                Task::none()
            }
            Message::PortInputChanged(port) => {
                self.port_input = port.clone();
                if let Ok(p) = port.parse::<u16>() {
                    let _ = crate::config::save_port(p);
                }
                Task::none()
            }
            Message::SetStartSword(val) => {
                self.start_sword = val;
                Task::none()
            }
            Message::SetStartArrow(val) => {
                self.start_arrow = val;
                Task::none()
            }
            Message::ToggleStartBow(val) => {
                self.start_bow = val;
                Task::none()
            }
            Message::SetStartCandle(val) => {
                self.start_candle = val;
                Task::none()
            }
            Message::SetStartRing(val) => {
                self.start_ring = val;
                Task::none()
            }
            Message::SetStartMagicShield(val) => {
                self.start_magic_shield = val;
                Task::none()
            }
            Message::SetStartBoomerang(val) => {
                self.start_boomerang = val;
                Task::none()
            }
            Message::SetStartBombsInput(val) => {
                self.start_bombs_input = val;
                Task::none()
            }
            Message::SetMaxBombsInput(val) => {
                self.max_bombs_input = val;
                Task::none()
            }

            Message::SetStartRupeesInput(val) => {
                self.start_rupees_input = val;
                Task::none()
            }
            Message::SetStartKeysInput(val) => {
                self.start_keys_input = val;
                Task::none()
            }
            Message::SetHeartContainers(val) => {
                self.heart_containers = val;
                Task::none()
            }
            Message::ToggleStartFood(val) => {
                self.start_food = val;
                Task::none()
            }
            Message::SetStartPotion(val) => {
                self.start_potion = val;
                Task::none()
            }

            Message::ToggleStartRecorder(val) => {
                self.start_recorder = val;
                Task::none()
            }
            Message::ToggleStartMagicRod(val) => {
                self.start_magic_rod = val;
                Task::none()
            }
            Message::ToggleStartRaft(val) => {
                self.start_raft = val;
                Task::none()
            }
            Message::ToggleStartBook(val) => {
                self.start_book = val;
                Task::none()
            }
            Message::ToggleStartLadder(val) => {
                self.start_ladder = val;
                Task::none()
            }
            Message::ToggleStartMagicKey(val) => {
                self.start_magic_key = val;
                Task::none()
            }
            Message::ToggleStartBracelet(val) => {
                self.start_bracelet = val;
                Task::none()
            }
            Message::ToggleStartLetter(val) => {
                self.start_letter = val;
                Task::none()
            }

            Message::ToggleCompass(idx, val) => {
                if idx < 9 {
                    self.compasses[idx] = val;
                }
                Task::none()
            }
            Message::ToggleMap(idx, val) => {
                if idx < 9 {
                    self.maps[idx] = val;
                }
                Task::none()
            }
            Message::ToggleTriforce(idx, val) => {
                if idx < 9 {
                    self.triforce_pieces[idx] = val;
                }
                Task::none()
            }
            Message::ToggleAllCompass(val) => {
                self.compasses = [val; 9];
                Task::none()
            }
            Message::ToggleAllMap(val) => {
                self.maps = [val; 9];
                Task::none()
            }
            Message::ToggleAllTriforce(val) => {
                self.triforce_pieces = [val; 9];
                Task::none()
            }
            Message::ToggleBossDefeated(idx, val) => {
                if idx < 9 {
                    self.bosses_defeated[idx] = val;
                }
                Task::none()
            }
            Message::ToggleAllBossesDefeated(val) => {
                self.bosses_defeated = [val; 9];
                Task::none()
            }

            Message::LogMessage(msg) => {
                self.log.push(msg);
                Task::none()
            }

            Message::GenerateRom => {
                self.log.push("ROM Generation triggered.".to_string());
                if let Some(base_path) = &self.base_rom_path {
                    let mut inventory_options = zdu_lib::StartInventoryOptions::default();
                    inventory_options.start_sword = self.start_sword;
                    inventory_options.start_arrow = self.start_arrow;
                    inventory_options.start_bow = self.start_bow;
                    inventory_options.start_candle = self.start_candle;
                    inventory_options.start_ring = self.start_ring;
                    inventory_options.start_magic_shield = self.start_magic_shield;
                    inventory_options.start_boomerang = self.start_boomerang;
                    inventory_options.start_bombs = self.start_bombs_input.parse().unwrap_or(0);
                    inventory_options.max_bombs = self.max_bombs_input.parse().unwrap_or(0);
                    inventory_options.start_rupees = self.start_rupees_input.parse().unwrap_or(0);
                    inventory_options.start_keys = self.start_keys_input.parse().unwrap_or(0);
                    inventory_options.heart_containers = self.heart_containers;
                    inventory_options.start_food = self.start_food;
                    inventory_options.start_potion = self.start_potion;
                    inventory_options.start_recorder = self.start_recorder;
                    inventory_options.start_magic_rod = self.start_magic_rod;
                    inventory_options.start_raft = self.start_raft;
                    inventory_options.start_book = self.start_book;
                    inventory_options.start_ladder = self.start_ladder;
                    inventory_options.start_magic_key = self.start_magic_key;
                    inventory_options.start_bracelet = self.start_bracelet;
                    inventory_options.start_letter = self.start_letter;

                    let mut progression_options = zdu_lib::ProgressionOptions::default();
                    progression_options.compasses = self.compasses;
                    progression_options.maps = self.maps;
                    progression_options.triforce_pieces = self.triforce_pieces;
                    progression_options.bosses_defeated = self.bosses_defeated;

                    let base_path_clone = base_path.clone();

                    return Task::perform(
                        async move {
                            // Offload blocking file I/O to a thread if possible, but async block is fine for now
                            let mut game = match zdu_lib::Game::new(&base_path_clone) {
                                Ok(g) => g,
                                Err(e) => return format!("Failed to load base ROM: {}", e),
                            };

                            game.set_starting_inventory(&inventory_options);
                            game.set_progression(&progression_options);

                            let out_nes = std::path::Path::new("zdu_generated.nes");
                            if let Err(e) = game.write(out_nes) {
                                return format!("Failed to write generated ROM: {}", e);
                            }

                            let out_bsdiff = std::path::Path::new("zdu_generated.bsdiff");
                            if let Err(e) = game.write_patch(out_bsdiff) {
                                return format!("Failed to write patch: {}", e);
                            }

                            let dungeon_data = game.read_dungeon_data();
                            let dungeon_spoiler = dungeon_data.to_string();
                            let cave_data = game.read_cave_data();
                            let cave_spoiler = cave_data.to_string();
                            let spoiler = cave_spoiler + &dungeon_spoiler;
                            let out_spoiler = std::path::Path::new("zdu_generated_spoiler.md");
                            if let Err(e) = std::fs::write(out_spoiler, spoiler) {
                                return format!("Failed to write spoiler log: {}", e);
                            }
                            // Show native success dialog
                            let _ = rfd::MessageDialog::new()
                                .set_title("Success")
                                .set_description("Successfully generated ROM and patch!")
                                .set_level(rfd::MessageLevel::Info)
                                .show();

                            "Successfully generated ROM and patch!".to_string()
                        },
                        |msg| Message::LogMessage(msg),
                    );
                } else {
                    self.log.push("Error: Base ROM path not set.".to_string());
                }
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        #[derive(Hash)]
        struct ServerSub(u16);
        match self.session {
            SessionState::Closed => Subscription::none(),
            SessionState::Open { port, .. } => Subscription::run_with(ServerSub(port), |id| {
                let port = id.0;
                stream::unfold(SubState::Init, move |state| async move {
                    match state {
                        SubState::Init => {
                            let (tx, mut rx) = unbounded_channel();
                            let (server_instance, _) = Server::new(port);
                            thread::spawn(move || {
                                let rt = Runtime::new().unwrap();
                                rt.block_on(async move {
                                    server_instance.run(tx).await;
                                });
                            });

                            if let Some(msg) = rx.recv().await {
                                Some((Message::ServerMessage(msg), SubState::Ready(rx)))
                            } else {
                                None
                            }
                        }
                        SubState::Ready(mut rx) => {
                            if let Some(msg) = rx.recv().await {
                                Some((Message::ServerMessage(msg), SubState::Ready(rx)))
                            } else {
                                None
                            }
                        }
                    }
                })
            }),
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let nav = column![
            button("Create")
                .on_press(Message::SwitchView(ViewMode::Patch))
                .width(100)
                .style(if self.view_mode == ViewMode::Patch {
                    button::primary
                } else {
                    button::text
                }),
            button("Configure")
                .on_press(Message::SwitchView(ViewMode::Configure))
                .width(100)
                .style(if self.view_mode == ViewMode::Configure {
                    button::primary
                } else {
                    button::text
                }),
            Space::new().height(8),
            button("Host")
                .on_press(Message::SwitchView(ViewMode::Host))
                .width(100)
                .style(if self.view_mode == ViewMode::Host {
                    button::primary
                } else {
                    button::text
                }),
            button("Log")
                .on_press(Message::SwitchView(ViewMode::Log))
                .width(100)
                .style(if self.view_mode == ViewMode::Log {
                    button::primary
                } else {
                    button::text
                }),
        ]
        .spacing(8);

        let content = match self.view_mode {
            ViewMode::Patch => {
                if self.base_rom_path.is_none() {
                    column![
                        text("Please configure your ROM path first in the Configure tab.").size(20)
                    ]
                } else {
                    patcher_view::view(self)
                }
            }
            ViewMode::Host => server_view::view(&self.session, &self.items, Message::ToggleSession),
            ViewMode::Log => crate::views::log_view::view(&self.log, Message::ClearLog),
            ViewMode::Configure => crate::views::config_view::view(
                &self.rom_path_input,
                self.remember_settings,
                Message::RomPathInputChanged,
                Message::BrowseRomPath,
                Message::SaveRomPath,
                Message::ToggleRememberSettings,
            ),
        };

        column![row![nav, container(content).width(Length::Fill)].spacing(40)].padding(20)
    }
}
