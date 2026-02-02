#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod ssh;
mod crypto;
mod session;
mod config;

use std::sync::Arc;
use tauri::{Manager, State};
use tokio::sync::Mutex;
use ssh_key::{Algorithm, PrivateKey, LineEnding};
use rand::rngs::OsRng;

use crate::ssh::{SshManager, SshConnection};
use crate::session::{SessionManager, ConnectionConfig};
use crate::crypto::SecureStorage;

pub struct AppState {
    ssh_manager: Arc<Mutex<SshManager>>,
    session_manager: Arc<Mutex<SessionManager>>,
    secure_storage: Arc<Mutex<SecureStorage>>,
}

#[tauri::command]
async fn connect_ssh(
    state: State<'_, AppState>,
    host: String,
    port: u16,
    username: String,
    auth_type: String,
    auth_value: String,
) -> Result<String, String> {
    let mut manager = state.ssh_manager.lock().await;
    
    let connection = ConnectionConfig {
        host,
        port,
        username,
        auth_type,
        auth_value,
    };
    
    match manager.connect(connection).await {
        Ok(session_id) => Ok(session_id),
        Err(e) => Err(format!("Connection failed: {}", e)),
    }
}

#[tauri::command]
async fn disconnect_ssh(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<(), String> {
    let mut manager = state.ssh_manager.lock().await;
    
    match manager.disconnect(&session_id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Disconnect failed: {}", e)),
    }
}

#[tauri::command]
async fn send_command(
    state: State<'_, AppState>,
    session_id: String,
    command: String,
) -> Result<(), String> {
    let manager = state.ssh_manager.lock().await;
    
    match manager.send_command(&session_id, &command).await {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Command failed: {}", e)),
    }
}

#[tauri::command]
async fn read_output(
    state: State<'_, AppState>,
    session_id: String,
) -> Result<String, String> {
    let manager = state.ssh_manager.lock().await;
    
    match manager.read_output(&session_id).await {
        Ok(output) => Ok(output),
        Err(e) => Err(format!("Read failed: {}", e)),
    }
}

#[tauri::command]
async fn save_session(
    state: State<'_, AppState>,
    name: String,
    host: String,
    port: u16,
    username: String,
    auth_type: String,
    auth_value: String,
) -> Result<(), String> {
    let mut session_manager = state.session_manager.lock().await;
    let secure_storage = state.secure_storage.lock().await;
    
    let config = ConnectionConfig {
        host,
        port,
        username: username.clone(),
        auth_type: auth_type.clone(),
        auth_value: auth_value.clone(),
    };
    
    let encrypted_auth = secure_storage.encrypt(&auth_value)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    match session_manager.save_session(&name, config, encrypted_auth).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Save failed: {}", e)),
    }
}

#[tauri::command]
async fn load_sessions(
    state: State<'_, AppState>,
) -> Result<Vec<serde_json::Value>, String> {
    let session_manager = state.session_manager.lock().await;
    
    match session_manager.load_sessions().await {
        Ok(sessions) => {
            let sessions_json: Vec<serde_json::Value> = sessions
                .into_iter()
                .map(|(name, config)| {
                    serde_json::json!({
                        "name": name,
                        "host": config.host,
                        "port": config.port,
                        "username": config.username,
                        "auth_type": config.auth_type,
                    })
                })
                .collect();
            Ok(sessions_json)
        }
        Err(e) => Err(format!("Load failed: {}", e)),
    }
}

#[tauri::command]
async fn delete_session(
    state: State<'_, AppState>,
    name: String,
) -> Result<(), String> {
    let mut session_manager = state.session_manager.lock().await;
    
    match session_manager.delete_session(&name).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Delete failed: {}", e)),
    }
}

#[tauri::command]
async fn get_session_credentials(
    state: State<'_, AppState>,
    name: String,
) -> Result<String, String> {
    let session_manager = state.session_manager.lock().await;
    let secure_storage = state.secure_storage.lock().await;
    
    match session_manager.get_session(&name).await {
        Ok((config, encrypted_auth)) => {
            let decrypted = secure_storage.decrypt(&encrypted_auth)
                .map_err(|e| format!("Decryption failed: {}", e))?;
            Ok(decrypted)
        }
        Err(e) => Err(format!("Get credentials failed: {}", e)),
    }
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
async fn resize_terminal(
    state: State<'_, AppState>,
    session_id: String,
    cols: u32,
    rows: u32,
) -> Result<(), String> {
    let manager = state.ssh_manager.lock().await;
    
    match manager.resize_terminal(&session_id, cols, rows).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Resize failed: {}", e)),
    }
}

#[tauri::command]
async fn generate_ssh_key(
    key_type: String,
    passphrase: Option<String>,
    comment: Option<String>,
) -> Result<serde_json::Value, String> {
    let algorithm = match key_type.as_str() {
        "ed25519" => Algorithm::Ed25519,
        "rsa" => Algorithm::Rsa { hash: None },
        _ => return Err("Unsupported key type. Use 'ed25519' or 'rsa'".to_string()),
    };
    
    let mut rng = OsRng;
    let private_key = PrivateKey::random(&mut rng, algorithm)
        .map_err(|e| format!("Failed to generate key: {}", e))?;
    
    // Set comment if provided
    let private_key = if let Some(comment) = comment {
        private_key.with_comment(&comment)
    } else {
        private_key
    };
    
    // Encrypt with passphrase if provided
    let private_key = if let Some(passphrase) = passphrase {
        private_key.encrypt(&mut rng, passphrase)
            .map_err(|e| format!("Failed to encrypt key: {}", e))?
    } else {
        private_key
    };
    
    // Generate OpenSSH format private key
    let private_key_pem = private_key.to_openssh(LineEnding::LF)
        .map_err(|e| format!("Failed to encode private key: {}", e))?;
    
    // Generate public key
    let public_key = private_key.public_key();
    let public_key_openssh = public_key.to_openssh()
        .map_err(|e| format!("Failed to encode public key: {}", e))?;
    
    // Generate fingerprint
    let fingerprint = public_key.fingerprint(ssh_key::HashAlg::Sha256);
    
    Ok(serde_json::json!({
        "private_key": private_key_pem,
        "public_key": public_key_openssh,
        "fingerprint": fingerprint.to_string(),
        "algorithm": key_type,
    }))
}

#[tauri::command]
async fn save_ssh_key(
    state: State<'_, AppState>,
    name: String,
    private_key: String,
) -> Result<(), String> {
    let storage = state.secure_storage.lock().await;
    
    // Save private key to secure storage
    storage.store(&format!("ssh_key_{}", name), &private_key)
        .map_err(|e| format!("Failed to save key: {}", e))?;
    
    Ok(())
}

#[tauri::command]
async fn load_ssh_key(
    state: State<'_, AppState>,
    name: String,
) -> Result<String, String> {
    let storage = state.secure_storage.lock().await;
    
    storage.retrieve(&format!("ssh_key_{}", name))
        .map_err(|e| format!("Failed to load key: {}", e))
}

#[tauri::command]
async fn list_ssh_keys(
    state: State<'_, AppState>,
) -> Result<Vec<String>, String> {
    let storage = state.secure_storage.lock().await;
    
    // List all keys with prefix "ssh_key_"
    let all_keys = storage.list_keys()
        .map_err(|e| format!("Failed to list keys: {}", e))?;
    
    let ssh_keys: Vec<String> = all_keys
        .into_iter()
        .filter(|k| k.starts_with("ssh_key_"))
        .map(|k| k.trim_start_matches("ssh_key_").to_string())
        .collect();
    
    Ok(ssh_keys)
}

#[tauri::command]
async fn delete_ssh_key(
    state: State<'_, AppState>,
    name: String,
) -> Result<(), String> {
    let storage = state.secure_storage.lock().await;
    
    storage.delete(&format!("ssh_key_{}", name))
        .map_err(|e| format!("Failed to delete key: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    
    let app_state = AppState {
        ssh_manager: Arc::new(Mutex::new(SshManager::new())),
        session_manager: Arc::new(Mutex::new(SessionManager::new())),
        secure_storage: Arc::new(Mutex::new(SecureStorage::new().expect("Failed to initialize secure storage"))),
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            connect_ssh,
            disconnect_ssh,
            send_command,
            read_output,
            save_session,
            load_sessions,
            delete_session,
            get_session_credentials,
            get_app_version,
            resize_terminal,
            generate_ssh_key,
            save_ssh_key,
            load_ssh_key,
            list_ssh_keys,
            delete_ssh_key,
        ])
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
