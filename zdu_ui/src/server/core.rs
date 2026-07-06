use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::spawn;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::{broadcast, Mutex};

use super::protocol::{encode_assign, encode_update, read_message, ClientMessage, ServerMessage};
use super::state::{BroadcastUpdate, GameState};

pub struct Server {
    pub state: Arc<Mutex<GameState>>,
    broadcast_tx: broadcast::Sender<BroadcastUpdate>,
    next_client_id: Arc<AtomicU32>,
    pub port: u16,
}

impl Server {
    pub fn new(port: u16) -> (Self, broadcast::Receiver<BroadcastUpdate>) {
        let (tx, rx) = broadcast::channel(16);
        (
            Self {
                state: Arc::new(Mutex::new(GameState::default())),
                broadcast_tx: tx,
                next_client_id: Arc::new(AtomicU32::new(1)),
                port,
            },
            rx,
        )
    }

    #[allow(dead_code)]
    pub fn broadcast(&self, update: BroadcastUpdate) {
        let _ = self.broadcast_tx.send(update);
    }

    pub async fn run(self, ui_tx: UnboundedSender<ServerMessage>) {
        let listener = match TcpListener::bind(("0.0.0.0", self.port)).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Failed to bind TCP listener on port {}: {}", self.port, e);
                return;
            }
        };
        println!("Server listening on port {}", self.port);

        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    let client_id = self.next_client_id.fetch_add(1, Ordering::SeqCst);
                    println!("Client {} connected: {}", client_id, addr);
                    let _ = ui_tx.send(ServerMessage::ClientConnected(client_id));
                    let state_clone = self.state.clone();
                    let bcast_tx = self.broadcast_tx.clone();
                    let bcast_rx = self.broadcast_tx.subscribe();
                    let ui_tx_clone = ui_tx.clone();

                    spawn(async move {
                        handle_client(
                            client_id,
                            stream,
                            state_clone,
                            bcast_tx,
                            bcast_rx,
                            ui_tx_clone,
                        )
                        .await;
                        println!("Client {} disconnected", client_id);
                    });
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {}", e);
                }
            }
        }
    }
}

async fn handle_client(
    client_id: u32,
    mut stream: TcpStream,
    state: Arc<Mutex<GameState>>,
    bcast_tx: broadcast::Sender<BroadcastUpdate>,
    mut bcast_rx: broadcast::Receiver<BroadcastUpdate>,
    ui_tx: UnboundedSender<ServerMessage>,
) {
    let (mut rx, mut tx) = stream.split();

    // Immediately send Assign message
    let assign_msg = encode_assign(client_id);
    if tx.write_all(&assign_msg).await.is_err() {
        let _ = ui_tx.send(ServerMessage::ClientDisconnected(client_id));
        return;
    }

    loop {
        tokio::select! {
            result = read_message(&mut rx) => {
                match result {
                    Ok(Some(msg)) => {
                        match msg {
                            ClientMessage::Check { client_id: msg_client_id, change_number } => {
                                if msg_client_id == client_id {
                                    let s = state.lock().await;
                                    if s.change_number > change_number {
                                        let mut updates = Vec::new();
                                        for (&k, &v) in &s.items {
                                            updates.push((k, v));
                                        }
                                        let update_msg = encode_update(0, s.change_number, &updates);
                                        if tx.write_all(&update_msg).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                            }
                            ClientMessage::Update { client_id: msg_client_id, change_number: _, updates } => {
                                if msg_client_id == client_id {
                                    let mut s = state.lock().await;
                                    let mut actual_changes = Vec::new();
                                    for (offset, val) in updates {
                                        let current = s.items.get(&offset).copied().unwrap_or(0);
                                        if val != current {
                                            s.items.insert(offset, val);
                                            actual_changes.push((offset, val));
                                        }
                                    }

                                    if !actual_changes.is_empty() {
                                        s.change_number += 1;
                                        let new_change = s.change_number;
                                        let _ = ui_tx.send(ServerMessage::UpdateReceived {
                                            client_id,
                                            change_number: new_change,
                                            updates: actual_changes.clone(),
                                        });
                                        let _ = bcast_tx.send(BroadcastUpdate {
                                            sender_id: client_id,
                                            new_change_number: new_change,
                                            updates: actual_changes,
                                        });
                                    }
                                }
                            }
                            _ => {} // Ignore Assign messages from client
                        }
                    }
                    Ok(None) => break, // EOF
                    Err(_) => break, // Read error
                }
            }

            result = bcast_rx.recv() => {
                match result {
                    Ok(update) => {
                        // Don't echo back to the sender if it was this client
                        if update.sender_id != client_id {
                            let update_msg = encode_update(0, update.new_change_number, &update.updates);
                            if tx.write_all(&update_msg).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        }
    }

    let _ = ui_tx.send(ServerMessage::ClientDisconnected(client_id));
}
