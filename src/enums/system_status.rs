
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key // Or `Aes128Gcm`
};
use bcrypt;
use serde::{Deserialize, Serialize};

use crate::enums::Error;
// use crate::types::Http;

#[derive(Debug,Serialize)]
pub struct CredentialsRequest {

}

#[derive(Debug,Deserialize)]
pub struct CredentialsResponse {

}

type Result<T> = std::result::Result<T,Error>;
pub enum SystemStatus {
    Locked,
    Unlocked
}

impl SystemStatus {
    async fn get_public_key() -> Result<String> {
        Ok("".to_string())
    }

    pub async fn unlock(password: &str) -> Result<SystemStatus> {
        let mut key = [0_u8;32];
        let slice = password.as_bytes();
        key[..slice.len()].copy_from_slice(slice);

        let master_key: &Key<Aes256Gcm> = Key::<Aes256Gcm>::from_slice(&key);
        let _public_hash = SystemStatus::get_public_key().await?;
        let _cipher = Aes256Gcm::new(master_key);
        
        Ok(SystemStatus::Unlocked)
    }
}