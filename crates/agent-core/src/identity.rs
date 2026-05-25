//! Agent identity — NPK/ISK keypair persisted to disk. The NPK doubles as the
//! agent's A2A AgentCard ID and its Logos Messaging address.

use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    /// 32-byte signing key, hex-encoded for storage.
    signing_key_hex: String,
}

impl Identity {
    pub fn load_or_create<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let path = path.as_ref();
        if path.exists() {
            let data = std::fs::read_to_string(path)?;
            let id: Identity = serde_json::from_str(&data)?;
            Ok(id)
        } else {
            let mut bytes = [0u8; 32];
            rand::RngCore::fill_bytes(&mut rand::thread_rng(), &mut bytes);
            let id = Identity {
                signing_key_hex: hex::encode(bytes),
            };
            if let Some(parent) = path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(path, serde_json::to_string_pretty(&id)?)?;
            Ok(id)
        }
    }

    pub fn signing_key(&self) -> anyhow::Result<SigningKey> {
        let bytes = hex::decode(&self.signing_key_hex)?;
        let arr: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| anyhow::anyhow!("identity key must be 32 bytes"))?;
        Ok(SigningKey::from_bytes(&arr))
    }

    pub fn verifying_key(&self) -> anyhow::Result<VerifyingKey> {
        Ok(self.signing_key()?.verifying_key())
    }

    /// The agent's NPK, hex-encoded — used as the AgentCard `id` and the
    /// Messaging address.
    pub fn npk_hex(&self) -> String {
        match self.verifying_key() {
            Ok(vk) => hex::encode(vk.to_bytes()),
            Err(_) => "<unset>".into(),
        }
    }
}
