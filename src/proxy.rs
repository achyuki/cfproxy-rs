/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use log::{debug, error};
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_rustls::client::TlsStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

use crate::error::Errors;

const BUFFER_SIZE: usize = 32 * 1024;

// SOCKS5 -> CF
pub async fn proxy_tcp_to_ws(
    reader: &mut ReadHalf<TcpStream>,
    mut ws_write: SplitSink<WebSocketStream<TlsStream<TcpStream>>, Message>,
) -> Result<(), Errors> {
    debug!("Starting SOCKS5 to WebSocket proxy");
    let mut buffer = [0u8; BUFFER_SIZE];
    loop {
        match reader.read(&mut buffer).await {
            Ok(n) if n > 0 => {
                ws_write
                    .send(Message::Binary(buffer[..n].to_vec().into()))
                    .await?;
                debug!("SOCKS5 TO WS: sent {} bytes", n);
            }
            Ok(_) => {
                debug!("SOCKS5 closed by client");
                break;
            }
            Err(e) => {
                error!("SOCKS5 error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

// CF -> SOCKS5
pub async fn proxy_ws_to_tcp(
    mut ws_read: SplitStream<WebSocketStream<TlsStream<TcpStream>>>,
    writer: &mut WriteHalf<TcpStream>,
) -> Result<(), Errors> {
    debug!("Starting WebSocket to SOCKS5 proxy");
    while let Some(msg) = ws_read.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                writer.write_all(&data).await?;
                debug!("WS TO SOCKS5: sent {} bytes", data.len());
            }
            Ok(Message::Close(_)) => {
                debug!("WebSocket closed by server");
                break;
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                return Err(Errors::WsError(e));
            }
            _ => continue,
        }
    }
    Ok(())
}
