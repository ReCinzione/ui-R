/// WebSocket — Aggiornamenti real-time ai client.

use axum::{
    extract::State,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
};
use tokio::sync::broadcast;

use super::state::AppState;

/// Handler per l'upgrade WebSocket.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Gestisce una singola connessione WebSocket.
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let mut rx = state.broadcast_tx.subscribe();

    // Invia stato iniziale
    let (tx, rx_state) = tokio::sync::oneshot::channel();
    let _ = state.cmd_tx.send(super::state::EngineCommand::GetState { reply: tx }).await;
    if let Ok(snapshot) = rx_state.await {
        let msg = serde_json::json!({
            "type": "state_update",
            "data": snapshot,
        });
        let _ = socket.send(Message::Text(msg.to_string().into())).await;
    }

    // Loop: inoltra broadcast ai client
    loop {
        tokio::select! {
            // Messaggi broadcast dall'engine
            msg = rx.recv() => {
                match msg {
                    Ok(text) => {
                        if socket.send(Message::Text(text.into())).await.is_err() {
                            break; // Client disconnesso
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => break,
                }
            }
            // Messaggi dal client (ping/pong, chiusura)
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        let _ = socket.send(Message::Pong(data)).await;
                    }
                    _ => {} // Ignora altri messaggi dal client
                }
            }
        }
    }
}
