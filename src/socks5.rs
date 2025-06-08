/*
 * Copyright (c) 2025 YukiChan
 * Licensed under the MIT License.
 * https://github.com/achyuki/cfproxy-rs/blob/main/LICENSE
 */

use futures::StreamExt;
use log::{debug, error, info};
use std::io::{Error, ErrorKind};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::TlsConnector;
use tokio_rustls::rustls::{ClientConfig, RootCertStore, pki_types::ServerName};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use webpki_roots::TLS_SERVER_ROOTS;

use crate::configutil::Config;
use crate::error::Errors;
use crate::proxy::{proxy_tcp_to_ws, proxy_ws_to_tcp};

pub async fn start_server(config: Arc<Config>) -> Result<(), Errors> {
    let addr = format!("{}:{}", config.host, config.port);
    let listener = TcpListener::bind(&addr).await.map_err(|e| {
        error!("Failed to start SOCKS5 server on {}: {}", addr, e);
        e
    })?;
    info!("SOCKS5 server started on: {}", addr);

    while let Ok((stream, peer_addr)) = listener.accept().await {
        let config = Arc::clone(&config);
        tokio::spawn(async move {
            if let Err(e) = handle_client(stream, config).await {
                error!("Client {} error: {:?}", peer_addr, e);
            }
        });
    }

    Ok(())
}

async fn handle_client(stream: TcpStream, config: Arc<Config>) -> Result<(), Errors> {
    let peer_addr = stream.peer_addr()?;
    let (mut reader, mut writer) = tokio::io::split(stream);
    info!("Handle client connection: {}", peer_addr);

    debug!("SOCKS5 handshake with client: {}", peer_addr);
    handshake(&mut reader, &mut writer, &config).await?;
    let (host, port) = parse_socks5(&mut reader).await?;

    debug!("Connect to WebSocket {} via {}", config.cfhost, config.cfip);
    let ws_port = 443;
    let addr = format!("{}:{}", config.cfip, ws_port);
    let tcp = TcpStream::connect(addr).await?;

    let mut root_cert_store = RootCertStore::empty();
    root_cert_store.extend(TLS_SERVER_ROOTS.iter().cloned());
    let client_config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_no_client_auth();
    let tls_connector = TlsConnector::from(Arc::new(client_config));
    let server_name = ServerName::try_from(config.cfhost.as_str())
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "invalid dnsname"))?
        .to_owned();
    let tls_stream = tls_connector.connect(server_name, tcp).await?;

    let ws_url = format!("wss://{}/", config.cfhost);
    let mut req = ws_url.clone().into_client_request()?;
    req.headers_mut()
        .insert("Host", config.cfhost.parse().unwrap());
    req.headers_mut()
        .insert("Token", config.token.parse().unwrap());
    req.headers_mut().insert("Hostname", host.parse().unwrap());
    req.headers_mut()
        .insert("Port", port.to_string().parse().unwrap());
    let (ws_stream, _) = tokio_tungstenite::client_async(req, tls_stream).await?;

    debug!("WebSocket connection established");
    let (ws_write, ws_read) = ws_stream.split();
    writer
        .write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0])
        .await?;

    info!("Connected: {}:{} via {}", host, port, peer_addr);
    tokio::try_join!(
        proxy_tcp_to_ws(&mut reader, ws_write),
        proxy_ws_to_tcp(ws_read, &mut writer)
    )?;

    info!("Connection closed: {}:{} via {}", host, port, peer_addr);
    Ok(())
}

async fn handshake(
    reader: &mut ReadHalf<TcpStream>,
    writer: &mut WriteHalf<TcpStream>,
    config: &Config,
) -> Result<(), Errors> {
    let mut init_buf = [0u8; 2];
    reader.read_exact(&mut init_buf).await?;

    if init_buf[0] != 0x05 {
        writer.write_all(&[0x05, 0xff]).await?;
        return Err(Errors::IoError(Error::new(
            ErrorKind::InvalidData,
            "Unsupported SOCKS version",
        )));
    }

    let mut methods = vec![0u8; init_buf[1] as usize];
    reader.read_exact(&mut methods).await?;

    if methods.contains(&0x02) {
        writer.write_all(&[0x05, 0x02]).await?;
        auth_user(reader, writer, config).await?;
    } else if methods.contains(&0x00) {
        writer.write_all(&[0x05, 0x00]).await?;
    } else {
        writer.write_all(&[0x05, 0xff]).await?;
        return Err(Errors::IoError(Error::new(
            ErrorKind::PermissionDenied,
            "No acceptable authentication methods",
        )));
    }
    Ok(())
}

async fn auth_user(
    reader: &mut ReadHalf<TcpStream>,
    writer: &mut WriteHalf<TcpStream>,
    config: &Config,
) -> Result<(), Errors> {
    let mut version = [0u8; 1];
    reader.read_exact(&mut version).await?;
    if version[0] != 0x01 {
        return Err(Errors::IoError(Error::new(
            ErrorKind::InvalidData,
            "Unsupported authentication version",
        )));
    }

    let mut username_len = [0u8; 1];
    reader.read_exact(&mut username_len).await?;
    let mut username = vec![0u8; username_len[0] as usize];
    reader.read_exact(&mut username).await?;
    let username = String::from_utf8_lossy(&username);

    let mut password_len = [0u8; 1];
    reader.read_exact(&mut password_len).await?;
    let mut password = vec![0u8; password_len[0] as usize];
    reader.read_exact(&mut password).await?;
    let password = String::from_utf8_lossy(&password);

    if username == config.user && password == config.passwd {
        writer.write_all(&[0x01, 0x00]).await?;
        Ok(())
    } else {
        writer.write_all(&[0x01, 0x01]).await?;
        Err(Errors::IoError(Error::new(
            ErrorKind::PermissionDenied,
            "Invalid username or password",
        )))
    }
}

async fn parse_socks5(reader: &mut ReadHalf<TcpStream>) -> Result<(String, u16), Errors> {
    let mut request = [0u8; 4];
    reader.read_exact(&mut request).await?;

    if request[0] != 0x05 {
        return Err(Errors::IoError(Error::new(
            ErrorKind::InvalidData,
            "Invalid SOCKS5 request",
        )));
    }
    // TODO: UDP support
    if request[1] != 0x01 {
        // writer.write_all(&[0x05, 0x07, 0x00, 0x01, 0,0,0,0, 0,0]).await?;
        return Err(Errors::IoError(Error::new(
            ErrorKind::InvalidData,
            "No support BIND/UDP",
        )));
    }

    let host = match request[3] {
        0x01 => {
            let mut addr = [0u8; 4];
            reader.read_exact(&mut addr).await?;
            addr.iter()
                .map(|b| b.to_string())
                .collect::<Vec<_>>()
                .join(".")
        }
        0x03 => {
            let mut len = [0u8; 1];
            reader.read_exact(&mut len).await?;
            let mut domain = vec![0u8; len[0] as usize];
            reader.read_exact(&mut domain).await?;
            String::from_utf8_lossy(&domain).to_string()
        }
        0x04 => {
            let mut addr = [0u8; 16];
            reader.read_exact(&mut addr).await?;
            let ipv6 = std::net::Ipv6Addr::from(addr);
            ipv6.to_string()
        }
        _ => {
            return Err(Errors::IoError(Error::new(
                ErrorKind::InvalidData,
                "Unsupported address type",
            )));
        }
    };

    let mut port_bytes = [0u8; 2];
    reader.read_exact(&mut port_bytes).await?;
    let port = u16::from_be_bytes(port_bytes);

    Ok((host, port))
}
