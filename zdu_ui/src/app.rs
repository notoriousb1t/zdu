use iced::widget::{column, row, button, text, Column};
use iced::{Alignment, Subscription, Task};

use crate::server::ServerMessage;
use crate::server::{Server, PORT};
use crate::views::{server_view, patcher_view};

#[derive(Debug, Clone)]
pub enum Message {
    ToggleSession,
    ServerMessage(ServerMessage),
    SwitchView(ViewMode),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewMode {
    Patcher,
    Server,
}

enum SubState {
    Init,
    Ready(tokio::sync::mpsc::UnboundedReceiver<ServerMessage>),
}

pub struct State {
    pub session: server_view::SessionState,
    pub log: Vec<String>,
    pub items: std::collections::HashMap<u8, u8>,
    pub view_mode: ViewMode,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                session: server_view::SessionState::Closed,
                log: vec!["Ready.".to_string()],
                items: std::collections::HashMap::new(),
                view_mode: ViewMode::Server,
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
                    server_view::SessionState::Closed => {
                        self.session = server_view::SessionState::Open {
                            port: PORT,
                            clients: 0,
                        };
                        self.log.push(format!("Opened session on port {}", PORT));
                    }
                    server_view::SessionState::Open { .. } => {
                        self.session = server_view::SessionState::Closed;
                        self.log.push("Closed session (UI only).".to_string());
                    }
                }
                Task::none()
            }
            Message::ServerMessage(msg) => {
                match msg {
                    ServerMessage::ClientConnected(id) => {
                        if let server_view::SessionState::Open { clients, .. } = &mut self.session {
                            *clients += 1;
                        }
                        self.log.push(format!("Client {} connected.", id));
                    }
                    ServerMessage::ClientDisconnected(id) => {
                        if let server_view::SessionState::Open { clients, .. } = &mut self.session {
                            *clients = clients.saturating_sub(1);
                        }
                        self.log.push(format!("Client {} disconnected.", id));
                    }
                    ServerMessage::UpdateReceived { client_id, change_number, updates } => {
                        for &(offset, val) in &updates {
                            self.items.insert(offset, val);
                        }
                        
                        self.log.push(format!(
                            "Client {} updated state to change #{}. {} items updated.",
                            client_id, change_number, updates.len()
                        ));
                    }
                }
                
                // Keep log bounded
                if self.log.len() > 20 {
                    self.log.remove(0);
                }
                
                Task::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        struct ServerSub;
        match self.session {
            server_view::SessionState::Closed => Subscription::none(),
            server_view::SessionState::Open { .. } => {
                Subscription::run_with(
                    std::any::TypeId::of::<ServerSub>(),
                    |_id| {
                        iced::futures::stream::unfold(
                            SubState::Init,
                            |state| async move {
                                match state {
                                    SubState::Init => {
                                        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                                        let (server_instance, _) = Server::new();
                                        std::thread::spawn(move || {
                                            let rt = tokio::runtime::Runtime::new().unwrap();
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
                            }
                        )
                    }
                )
            }
        }
    }

    pub fn view(&self) -> Column<'_, Message> {
        let nav = row![
            button("Server").on_press(Message::SwitchView(ViewMode::Server)),
            button("Patcher").on_press(Message::SwitchView(ViewMode::Patcher)),
        ].spacing(10);
        
        let content = match self.view_mode {
            ViewMode::Server => server_view::view(&self.session, &self.log, &self.items, Message::ToggleSession),
            ViewMode::Patcher => patcher_view::view(),
        };

        column![
            nav,
            content
        ].spacing(20).padding(20).align_x(Alignment::Center)
    }
}
