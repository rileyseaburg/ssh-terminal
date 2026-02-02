use ssh2::Session;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::time::Duration;
use crate::session::ConnectionConfig;
use log::info;

pub struct SshConnection {
    session: Session,
    channel: ssh2::Channel,
    stream: TcpStream,
}

pub struct SshManager {
    connections: HashMap<String, Arc<Mutex<SshConnection>>>,
}

impl SshManager {
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
        }
    }

    pub async fn connect(&mut self, config: ConnectionConfig) -> Result<String, SshError> {
        info!("Starting SSH connection to {}:{} as {}", config.host, config.port, config.username);
        
        let session_id = uuid::Uuid::new_v4().to_string();
        
        let addr = format!("{}:{}", config.host, config.port);
        info!("Connecting to address: {}", addr);
        
        // Set a timeout for the connection
        let tcp = tokio::time::timeout(
            Duration::from_secs(10),
            tokio::task::spawn_blocking(move || {
                TcpStream::connect(&addr)
            })
        ).await
        .map_err(|_| SshError::ConnectionFailed("Connection timeout".to_string()))?
        .map_err(|e| SshError::ConnectionFailed(format!("Task join error: {}", e)))?
        .map_err(|e| SshError::ConnectionFailed(format!("TCP connect failed: {}", e)))?;
        
        info!("TCP connection established");
        
        // Set non-blocking mode for the stream
        tcp.set_nonblocking(true)
            .map_err(|e| SshError::ConnectionFailed(format!("Failed to set non-blocking: {}", e)))?;
        
        info!("Creating SSH session...");
        let mut session = Session::new()
            .map_err(|e| SshError::SessionCreationFailed(e.to_string()))?;
        
        info!("Setting TCP stream...");
        session.set_tcp_stream(tcp.try_clone().map_err(|e| SshError::CloneFailed(e.to_string()))?);
        
        info!("Starting SSH handshake...");
        session.handshake()
            .map_err(|e| SshError::HandshakeFailed(e.to_string()))?;
        
        info!("Handshake complete, authenticating...");
        match config.auth_type.as_str() {
            "password" => {
                info!("Using password authentication");
                session.userauth_password(&config.username, &config.auth_value)
                    .map_err(|e| SshError::AuthFailed(e.to_string()))?;
            }
            "key" => {
                info!("Using key authentication");
                let key_path = Path::new(&config.auth_value);
                session.userauth_pubkey_file(&config.username, None, key_path, None)
                    .map_err(|e| SshError::AuthFailed(format!("Key auth failed: {}", e)))?;
            }
            "agent" => {
                info!("Using agent authentication");
                session.userauth_agent(&config.username)
                    .map_err(|e| SshError::AuthFailed(format!("Agent auth failed: {}", e)))?;
            }
            _ => return Err(SshError::InvalidAuthType),
        }
        
        if !session.authenticated() {
            return Err(SshError::AuthFailed("Authentication failed".to_string()));
        }
        
        info!("Authenticated successfully");
        
        info!("Creating channel session...");
        let mut channel = session.channel_session()
            .map_err(|e| SshError::ChannelFailed(e.to_string()))?;
        
        info!("Requesting PTY...");
        channel.request_pty("xterm-256color", None, Some((80, 24, 0, 0)))
            .map_err(|e| SshError::PtyRequestFailed(e.to_string()))?;
        
        info!("Starting shell...");
        channel.shell()
            .map_err(|e| SshError::ShellFailed(e.to_string()))?;
        
        info!("Connection established successfully");
        
        let connection = SshConnection {
            session,
            channel,
            stream: tcp,
        };
        
        self.connections.insert(
            session_id.clone(),
            Arc::new(Mutex::new(connection))
        );
        
        Ok(session_id)
    }

    pub async fn disconnect(&mut self, session_id: &str) -> Result<(), SshError> {
        if let Some(conn) = self.connections.remove(session_id) {
            let mut conn = conn.lock().await;
            conn.channel.send_eof().ok();
            conn.channel.wait_eof().ok();
            conn.channel.close().ok();
            conn.channel.wait_close().ok();
        }
        Ok(())
    }

    pub async fn send_command(&self, session_id: &str, command: &str) -> Result<(), SshError> {
        if let Some(conn) = self.connections.get(session_id) {
            let mut conn = conn.lock().await;
            conn.channel.write_all(command.as_bytes())
                .map_err(|e| SshError::WriteFailed(e.to_string()))?;
            conn.channel.flush()
                .map_err(|e| SshError::WriteFailed(e.to_string()))?;
            Ok(())
        } else {
            Err(SshError::SessionNotFound)
        }
    }

    pub async fn read_output(&self, session_id: &str) -> Result<String, SshError> {
        if let Some(conn) = self.connections.get(session_id) {
            let mut conn = conn.lock().await;
            let mut buffer = [0u8; 8192];
            let mut output = String::new();
            
            match conn.channel.read(&mut buffer) {
                Ok(0) => Ok(output),
                Ok(n) => {
                    output.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    Ok(output)
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => Ok(output),
                Err(e) => Err(SshError::ReadFailed(e.to_string())),
            }
        } else {
            Err(SshError::SessionNotFound)
        }
    }

    pub async fn resize_terminal(
        &self,
        session_id: &str,
        cols: u32,
        rows: u32,
    ) -> Result<(), SshError> {
        if let Some(conn) = self.connections.get(session_id) {
            let mut conn = conn.lock().await;
            conn.channel.request_pty_size(cols as u32, rows as u32, None, None)
                .map_err(|e| SshError::ResizeFailed(e.to_string()))?;
            Ok(())
        } else {
            Err(SshError::SessionNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SshError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    #[error("Session creation failed: {0}")]
    SessionCreationFailed(String),
    #[error("Stream clone failed: {0}")]
    CloneFailed(String),
    #[error("Handshake failed: {0}")]
    HandshakeFailed(String),
    #[error("Authentication failed: {0}")]
    AuthFailed(String),
    #[error("Invalid authentication type")]
    InvalidAuthType,
    #[error("Channel creation failed: {0}")]
    ChannelFailed(String),
    #[error("PTY request failed: {0}")]
    PtyRequestFailed(String),
    #[error("Shell request failed: {0}")]
    ShellFailed(String),
    #[error("Write failed: {0}")]
    WriteFailed(String),
    #[error("Read failed: {0}")]
    ReadFailed(String),
    #[error("Resize failed: {0}")]
    ResizeFailed(String),
    #[error("Session not found")]
    SessionNotFound,
}
