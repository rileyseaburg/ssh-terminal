use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub theme: String,
    pub font_size: u32,
    pub font_family: String,
    pub cursor_style: String,
    pub bell_enabled: bool,
    pub copy_on_select: bool,
    pub scrollback_lines: u32,
    pub window_opacity: f64,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub auto_lock_timeout: u32,
    pub require_password_on_startup: bool,
    pub ssh_key_passphrase_cache: bool,
    pub verify_host_keys: bool,
    pub strict_host_key_checking: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            font_size: 14,
            font_family: "JetBrains Mono, monospace".to_string(),
            cursor_style: "block".to_string(),
            bell_enabled: false,
            copy_on_select: true,
            scrollback_lines: 10000,
            window_opacity: 1.0,
            security: SecurityConfig::default(),
        }
    }
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            auto_lock_timeout: 300,
            require_password_on_startup: false,
            ssh_key_passphrase_cache: false,
            verify_host_keys: true,
            strict_host_key_checking: true,
        }
    }
}

pub struct ConfigManager {
    config: AppConfig,
    config_path: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self> {
        let config_path = Self::get_config_path();
        let config = Self::load_config(&config_path)?;
        
        Ok(Self {
            config,
            config_path,
        })
    }

    fn get_config_path() -> PathBuf {
        if let Some(config_dir) = directories::ProjectDirs::from("com", "sshterminal", "app") {
            config_dir.config_dir().join("config.json")
        } else {
            PathBuf::from(".config/ssh-terminal/config.json")
        }
    }

    fn load_config(path: &PathBuf) -> Result<AppConfig> {
        if !path.exists() {
            let config = AppConfig::default();
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let content = serde_json::to_string_pretty(&config)?;
            std::fs::write(path, content)?;
            return Ok(config);
        }
        
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn get_config(&self) -> &AppConfig {
        &self.config
    }

    pub fn update_config<F>(&mut self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut AppConfig),
    {
        updater(&mut self.config);
        self.save_config()
    }

    fn save_config(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(&self.config)?;
        std::fs::write(&self.config_path, content)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&self.config_path)?.permissions();
            permissions.set_mode(0o600);
            std::fs::set_permissions(&self.config_path, permissions)?;
        }
        
        Ok(())
    }
}

pub mod themes {
    use std::collections::HashMap;
    
    pub type Theme = HashMap<String, String>;
    
    pub fn get_dark_theme() -> Theme {
        let mut theme = HashMap::new();
        theme.insert("background".to_string(), "#1e1e1e".to_string());
        theme.insert("foreground".to_string(), "#d4d4d4".to_string());
        theme.insert("cursor".to_string(), "#d4d4d4".to_string());
        theme.insert("selection".to_string(), "#264f78".to_string());
        theme.insert("black".to_string(), "#1e1e1e".to_string());
        theme.insert("red".to_string(), "#f44747".to_string());
        theme.insert("green".to_string(), "#608b4e".to_string());
        theme.insert("yellow".to_string(), "#dcdcaa".to_string());
        theme.insert("blue".to_string(), "#569cd6".to_string());
        theme.insert("magenta".to_string(), "#c586c0".to_string());
        theme.insert("cyan".to_string(), "#4ec9b0".to_string());
        theme.insert("white".to_string(), "#d4d4d4".to_string());
        theme.insert("brightBlack".to_string(), "#808080".to_string());
        theme.insert("brightRed".to_string(), "#f44747".to_string());
        theme.insert("brightGreen".to_string(), "#b5cea8".to_string());
        theme.insert("brightYellow".to_string(), "#dcdcaa".to_string());
        theme.insert("brightBlue".to_string(), "#9cdcfe".to_string());
        theme.insert("brightMagenta".to_string(), "#c586c0".to_string());
        theme.insert("brightCyan".to_string(), "#4ec9b0".to_string());
        theme.insert("brightWhite".to_string(), "#ffffff".to_string());
        theme
    }
    
    pub fn get_light_theme() -> Theme {
        let mut theme = HashMap::new();
        theme.insert("background".to_string(), "#ffffff".to_string());
        theme.insert("foreground".to_string(), "#323232".to_string());
        theme.insert("cursor".to_string(), "#323232".to_string());
        theme.insert("selection".to_string(), "#add6ff".to_string());
        theme.insert("black".to_string(), "#000000".to_string());
        theme.insert("red".to_string(), "#cd3131".to_string());
        theme.insert("green".to_string(), "#00bc00".to_string());
        theme.insert("yellow".to_string(), "#949800".to_string());
        theme.insert("blue".to_string(), "#0451a5".to_string());
        theme.insert("magenta".to_string(), "#bc05bc".to_string());
        theme.insert("cyan".to_string(), "#0598bc".to_string());
        theme.insert("white".to_string(), "#555555".to_string());
        theme.insert("brightBlack".to_string(), "#666666".to_string());
        theme.insert("brightRed".to_string(), "#cd3131".to_string());
        theme.insert("brightGreen".to_string(), "#14ce14".to_string());
        theme.insert("brightYellow".to_string(), "#b5ba00".to_string());
        theme.insert("brightBlue".to_string(), "#0451a5".to_string());
        theme.insert("brightMagenta".to_string(), "#bc05bc".to_string());
        theme.insert("brightCyan".to_string(), "#0598bc".to_string());
        theme.insert("brightWhite".to_string(), "#a5a5a5".to_string());
        theme
    }
    
    pub fn get_dracula_theme() -> Theme {
        let mut theme = HashMap::new();
        theme.insert("background".to_string(), "#282a36".to_string());
        theme.insert("foreground".to_string(), "#f8f8f2".to_string());
        theme.insert("cursor".to_string(), "#f8f8f2".to_string());
        theme.insert("selection".to_string(), "#44475a".to_string());
        theme.insert("black".to_string(), "#21222c".to_string());
        theme.insert("red".to_string(), "#ff5555".to_string());
        theme.insert("green".to_string(), "#50fa7b".to_string());
        theme.insert("yellow".to_string(), "#f1fa8c".to_string());
        theme.insert("blue".to_string(), "#bd93f9".to_string());
        theme.insert("magenta".to_string(), "#ff79c6".to_string());
        theme.insert("cyan".to_string(), "#8be9fd".to_string());
        theme.insert("white".to_string(), "#f8f8f2".to_string());
        theme.insert("brightBlack".to_string(), "#6272a4".to_string());
        theme.insert("brightRed".to_string(), "#ff6e6e".to_string());
        theme.insert("brightGreen".to_string(), "#69ff94".to_string());
        theme.insert("brightYellow".to_string(), "#ffffa5".to_string());
        theme.insert("brightBlue".to_string(), "#d6acff".to_string());
        theme.insert("brightMagenta".to_string(), "#ff92df".to_string());
        theme.insert("brightCyan".to_string(), "#a4ffff".to_string());
        theme.insert("brightWhite".to_string(), "#ffffff".to_string());
        theme
    }
}
