use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use keyring::Entry;
use rand::Rng;

const KEYRING_SERVICE: &str = "ssh-terminal-app";
const KEYRING_USERNAME: &str = "encryption-key";
const NONCE_SIZE: usize = 12;

pub struct SecureStorage {
    cipher: Aes256Gcm,
}

impl SecureStorage {
    pub fn new() -> Result<Self> {
        let key = Self::get_or_create_key()?;
        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|_| anyhow::anyhow!("Failed to create cipher: invalid key length"))?;
        
        Ok(Self { cipher })
    }

    fn get_or_create_key() -> Result<Vec<u8>> {
        let entry = Entry::new(KEYRING_SERVICE, KEYRING_USERNAME)?;
        
        match entry.get_password() {
            Ok(key_b64) => {
                let key = base64::decode(&key_b64)
                    .map_err(|e| anyhow::anyhow!("Failed to decode key: {}", e))?;
                Ok(key)
            }
            Err(_) => {
                let key: Vec<u8> = (0..32).map(|_| rand::thread_rng().gen()).collect();
                let key_b64 = base64::encode(&key);
                entry.set_password(&key_b64)?;
                Ok(key)
            }
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let nonce_bytes: Vec<u8> = (0..NONCE_SIZE).map(|_| rand::thread_rng().gen()).collect();
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        let ciphertext = self.cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {:?}", e))?;
        
        let mut result = nonce_bytes;
        result.extend_from_slice(&ciphertext);
        
        Ok(base64::encode(result))
    }

    pub fn decrypt(&self, ciphertext_b64: &str) -> Result<String> {
        let data = base64::decode(ciphertext_b64)
            .map_err(|e| anyhow::anyhow!("Base64 decode failed: {}", e))?;
        
        if data.len() < NONCE_SIZE {
            return Err(anyhow::anyhow!("Invalid encrypted data"));
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| anyhow::anyhow!("Decryption failed: {:?}", e))?;
        
        String::from_utf8(plaintext)
            .context("Invalid UTF-8 in decrypted data")
    }

    pub fn store(&self, key: &str, value: &str) -> Result<()> {
        let entry = Entry::new(KEYRING_SERVICE, key)?;
        let encrypted = self.encrypt(value)?;
        entry.set_password(&encrypted)?;
        Ok(())
    }

    pub fn retrieve(&self, key: &str) -> Result<String> {
        let entry = Entry::new(KEYRING_SERVICE, key)?;
        let encrypted = entry.get_password()?;
        self.decrypt(&encrypted)
    }

    pub fn delete(&self, key: &str) -> Result<()> {
        let entry = Entry::new(KEYRING_SERVICE, key)?;
        entry.delete_password()?;
        Ok(())
    }

    pub fn list_keys(&self) -> Result<Vec<String>> {
        // Note: keyring doesn't provide a direct way to list keys
        // This is a limitation - we'd need to track keys separately
        // For now, return an empty list
        Ok(vec![])
    }
}

pub mod base64 {
    pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
        let bytes = input.as_ref();
        let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);
        
        for chunk in bytes.chunks(3) {
            let mut buf = [0u8; 3];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = byte;
            }
            
            let b = [
                buf[0] >> 2,
                ((buf[0] & 0x03) << 4) | (buf[1] >> 4),
                ((buf[1] & 0x0f) << 2) | (buf[2] >> 6),
                buf[2] & 0x3f,
            ];
            
            for i in 0..4 {
                if i <= chunk.len() {
                    result.push(ENCODING_TABLE[b[i] as usize]);
                } else {
                    result.push('=');
                }
            }
        }
        
        result
    }
    
    pub fn decode<T: AsRef<str>>(input: T) -> Result<Vec<u8>, String> {
        let input = input.as_ref().trim();
        let mut result = Vec::with_capacity(input.len() * 3 / 4);
        
        for chunk in input.as_bytes().chunks(4) {
            let mut buf = [0u8; 4];
            for (i, &byte) in chunk.iter().enumerate() {
                buf[i] = decode_byte(byte)?;
            }
            
            let len = chunk.iter().take_while(|&&b| b != b'=').count().saturating_sub(1);
            
            result.push((buf[0] << 2) | (buf[1] >> 4));
            if len > 0 {
                result.push((buf[1] << 4) | (buf[2] >> 2));
            }
            if len > 1 {
                result.push((buf[2] << 6) | buf[3]);
            }
        }
        
        Ok(result)
    }
    
    fn decode_byte(byte: u8) -> Result<u8, String> {
        match byte {
            b'A'..=b'Z' => Ok(byte - b'A'),
            b'a'..=b'z' => Ok(byte - b'a' + 26),
            b'0'..=b'9' => Ok(byte - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => Ok(0),
            _ => Err(format!("Invalid base64 character: {}", byte)),
        }
    }
    
    const ENCODING_TABLE: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
    ];
}
