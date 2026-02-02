use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub auth_type: String,
    pub auth_value: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SavedSession {
    config: ConnectionConfig,
    encrypted_auth: String,
}

pub struct SessionManager {
    sessions: HashMap<String, SavedSession>,
    config_dir: PathBuf,
}

impl SessionManager {
    pub fn new() -> Self {
        let config_dir = Self::get_config_dir();
        std::fs::create_dir_all(&config_dir).ok();
        
        Self {
            sessions: HashMap::new(),
            config_dir,
        }
    }

    fn get_config_dir() -> PathBuf {
        if let Some(config_dir) = directories::ProjectDirs::from("com", "sshterminal", "app") {
            config_dir.config_dir().to_path_buf()
        } else {
            PathBuf::from(".config/ssh-terminal")
        }
    }

    fn get_sessions_file(&self) -> PathBuf {
        self.config_dir.join("sessions.json")
    }

    pub async fn save_session(
        &mut self,
        name: &str,
        config: ConnectionConfig,
        encrypted_auth: String,
    ) -> Result<()> {
        self.load_sessions_from_disk().await?;
        
        let session = SavedSession {
            config,
            encrypted_auth,
        };
        
        self.sessions.insert(name.to_string(), session);
        self.save_sessions_to_disk().await?;
        
        Ok(())
    }

    pub async fn load_sessions(&self) -> Result<Vec<(String, ConnectionConfig)>> {
        self.load_sessions_from_disk().await?;
        
        let mut result = Vec::new();
        for (name, session) in &self.sessions {
            result.push((name.clone(), session.config.clone()));
        }
        
        Ok(result)
    }

    pub async fn get_session(&self, name: &str) -> Result<(ConnectionConfig, String)> {
        let sessions = self.load_sessions_from_disk().await?;
        
        if let Some(session) = sessions.get(name) {
            Ok((session.config.clone(), session.encrypted_auth.clone()))
        } else {
            Err(anyhow::anyhow!("Session not found: {}", name))
        }
    }

    pub async fn delete_session(&mut self, name: &str) -> Result<()> {
        self.load_sessions_from_disk().await?;
        
        self.sessions.remove(name);
        self.save_sessions_to_disk().await?;
        
        Ok(())
    }

    async fn load_sessions_from_disk(&self) -> Result<HashMap<String, SavedSession>> {
        let file_path = self.get_sessions_file();
        
        if !file_path.exists() {
            return Ok(HashMap::new());
        }
        
        let content = tokio::fs::read_to_string(&file_path).await?;
        let sessions: HashMap<String, SavedSession> = serde_json::from_str(&content)?;
        
        Ok(sessions)
    }

    async fn save_sessions_to_disk(&self) -> Result<()> {
        let file_path = self.get_sessions_file();
        let content = serde_json::to_string_pretty(&self.sessions)?;
        tokio::fs::write(&file_path, content).await?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&file_path)?.permissions();
            permissions.set_mode(0o600);
            std::fs::set_permissions(&file_path, permissions)?;
        }
        
        Ok(())
    }
}
