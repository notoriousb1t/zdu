use std::io::{self, ErrorKind};
use tokio::io::AsyncReadExt;
use tokio::net::tcp::ReadHalf;

#[derive(Debug, Clone)]
pub enum ServerMessage {
    ClientConnected(u32),
    ClientDisconnected(u32),
    UpdateReceived {
        client_id: u32,
        change_number: u64,
        updates: Vec<(u16, u8)>,
    },
}

pub enum ClientMessage {
    Assign {
        #[allow(dead_code)]
        client_id: u32,
        #[allow(dead_code)]
        assigned_id: u32,
    },
    Check {
        client_id: u32,
        change_number: u64,
    },
    Update {
        client_id: u32,
        #[allow(dead_code)]
        change_number: u64,
        updates: Vec<(u16, u8)>,
    },
}

pub async fn read_message(rx: &mut ReadHalf<'_>) -> io::Result<Option<ClientMessage>> {
    let mut header = [0u8; 7]; // 1 byte type, 4 bytes Client ID, 2 bytes Length
    let _ = match rx.read_exact(&mut header).await {
        Ok(n) => n,
        Err(e) if e.kind() == ErrorKind::UnexpectedEof => return Ok(None),
        Err(e) => return Err(e),
    };

    let msg_type = header[0];

    let mut cid_bytes = [0u8; 4];
    cid_bytes.copy_from_slice(&header[1..5]);
    let client_id = u32::from_be_bytes(cid_bytes);

    let mut len_bytes = [0u8; 2];
    len_bytes.copy_from_slice(&header[5..7]);
    let length = u16::from_be_bytes(len_bytes) as usize;

    let mut payload = vec![0u8; length];
    if length > 0 {
        rx.read_exact(&mut payload).await?;
    }

    match msg_type {
        0x00 => {
            if length >= 4 {
                let mut bytes = [0u8; 4];
                bytes.copy_from_slice(&payload[0..4]);
                let assigned_id = u32::from_be_bytes(bytes);
                Ok(Some(ClientMessage::Assign {
                    client_id,
                    assigned_id,
                }))
            } else {
                Ok(None)
            }
        }
        0x01 => {
            if length >= 8 {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&payload[0..8]);
                let change_number = u64::from_be_bytes(bytes);
                Ok(Some(ClientMessage::Check {
                    client_id,
                    change_number,
                }))
            } else {
                Ok(None)
            }
        }
        0x02 => {
            if length >= 8 {
                let mut bytes = [0u8; 8];
                bytes.copy_from_slice(&payload[0..8]);
                let change_number = u64::from_be_bytes(bytes);

                let mut updates = Vec::new();
                let mut i = 8;
                while i + 2 < length {
                    let offset = u16::from_be_bytes([payload[i], payload[i + 1]]);
                    updates.push((offset, payload[i + 2]));
                    i += 3;
                }

                Ok(Some(ClientMessage::Update {
                    client_id,
                    change_number,
                    updates,
                }))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}

pub fn encode_assign(assigned_id: u32) -> Vec<u8> {
    let mut msg = Vec::new();
    msg.push(0x00); // Message Type
    msg.extend_from_slice(&0u32.to_be_bytes()); // Server ID is 0
    let length: u16 = 4;
    msg.extend_from_slice(&length.to_be_bytes()); // Length
    msg.extend_from_slice(&assigned_id.to_be_bytes());
    msg
}

pub fn encode_update(sender_id: u32, change_number: u64, updates: &[(u16, u8)]) -> Vec<u8> {
    let mut msg = Vec::new();
    msg.push(0x02); // Message Type
    msg.extend_from_slice(&sender_id.to_be_bytes());

    let length = (8 + updates.len() * 3) as u16;
    msg.extend_from_slice(&length.to_be_bytes()); // Length

    msg.extend_from_slice(&change_number.to_be_bytes());
    for &(offset, val) in updates {
        msg.extend_from_slice(&offset.to_be_bytes());
        msg.push(val);
    }

    msg
}
