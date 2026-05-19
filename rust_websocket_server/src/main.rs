use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio::sync::Mutex;
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

type SharedUsers = Arc<Mutex<HashMap<SocketAddr, String>>>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum MessageType {
    Users,
    Register,
    Message,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ClientMessage {
    message_type: MessageType,
    data: Option<String>,
    data_array: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerMessage {
    message_type: MessageType,
    data: Option<String>,
    data_array: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    from: String,
    message: String,
    time: i64,
}

fn users_message(users: Vec<String>) -> Result<String, serde_json::Error> {
    serde_json::to_string(&ServerMessage {
        message_type: MessageType::Users,
        data: None,
        data_array: Some(users),
    })
}

fn chat_message(from: String, message: String) -> Result<String, serde_json::Error> {
    let payload = ChatMessage {
        from,
        message,
        time: chrono::Utc::now().timestamp_millis(),
    };

    serde_json::to_string(&ServerMessage {
        message_type: MessageType::Message,
        data: Some(serde_json::to_string(&payload)?),
        data_array: None,
    })
}

async fn current_users(users: &SharedUsers) -> Vec<String> {
    users.lock().await.values().cloned().collect()
}

async fn broadcast_users(
    users: &SharedUsers,
    tx: &Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let message = users_message(current_users(users).await)?;
    let _ = tx.send(message);
    Ok(())
}

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    users: SharedUsers,
    tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut rx = tx.subscribe();
    println!("web client connected from {addr}");

    loop {
        tokio::select! {
            incoming = ws_stream.next() => {
                let Some(message) = incoming else {
                    break;
                };

                let message = message?;
                if let Some(text) = message.as_text() {
                    let parsed: ClientMessage = serde_json::from_str(text)?;

                    match parsed.message_type {
                        MessageType::Register => {
                            if let Some(username) = parsed.data {
                                println!("{addr} registered as {username}");
                                users.lock().await.insert(addr, username);
                                broadcast_users(&users, &tx).await?;
                            }
                        }
                        MessageType::Message => {
                            let username = users
                                .lock()
                                .await
                                .get(&addr)
                                .cloned()
                                .unwrap_or_else(|| addr.to_string());

                            if let Some(message) = parsed.data {
                                println!("from {username}: {message}");
                                let _ = tx.send(chat_message(username, message)?);
                            }
                        }
                        MessageType::Users => {}
                    }
                }
            }
            broadcast = rx.recv() => {
                let message = broadcast?;
                ws_stream.send(Message::text(message)).await?;
            }
        }
    }

    users.lock().await.remove(&addr);
    broadcast_users(&users, &tx).await?;
    println!("{addr} disconnected");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let users = Arc::new(Mutex::new(HashMap::new()));
    let (tx, _) = channel(128);
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    println!("Rust YewChat WebSocket server listening on port 8080");
    loop {
        let (socket, addr) = listener.accept().await?;
        let users = users.clone();
        let tx = tx.clone();

        tokio::spawn(async move {
            let result = async {
                let (_request, ws_stream) = ServerBuilder::new().accept(socket).await?;
                handle_connection(addr, ws_stream, users, tx).await
            }
            .await;

            if let Err(error) = result {
                eprintln!("connection error for {addr}: {error}");
            }
        });
    }
}
