//! A2A AgentCard — JSON metadata document published to the discovery topic.
//!
//! Conforms to the A2A v1.0.0 schema. The `interfaces` array declares the
//! Logos Messaging transport binding URI, and the `extensions` array declares
//! the LEZ payment extension.

use agent_skill_sdk::SkillManifest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCard {
    pub id: String,
    pub name: String,
    pub description: String,
    pub provider: AgentProvider,
    pub skills: Vec<SkillManifest>,
    pub capabilities: AgentCapabilities,
    #[serde(rename = "securitySchemes", default)]
    pub security_schemes: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub security: Vec<serde_json::Value>,
    pub interfaces: Vec<AgentInterface>,
    #[serde(default)]
    pub extensions: Vec<AgentExtension>,
    pub signature: AgentCardSignature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentProvider {
    pub name: String,
    pub website: Option<String>,
    pub contact: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapabilities {
    pub streaming: bool,
    #[serde(rename = "pushNotifications")]
    pub push_notifications: bool,
    #[serde(rename = "extendedAgentCard")]
    pub extended_agent_card: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInterface {
    /// Binding identifier — for Logos Messaging,
    /// `https://logos.co/spec/a2a-bindings/logos-messaging/0.1`.
    pub uri: String,
    /// Endpoint for this binding. For Logos Messaging the endpoint is a Waku
    /// content topic: `waku:/logos-a2a/0.1/tasks/<npk>/inbox`.
    pub endpoint: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentExtension {
    pub uri: String,
    pub required: bool,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCardSignature {
    pub algorithm: String,
    #[serde(rename = "keyId")]
    pub key_id: String,
    /// Hex-encoded signature over the canonicalized card (excluding this field).
    pub value: String,
}

impl AgentCard {
    /// Build a card with the standard Logos bindings already filled in.
    pub fn for_agent(
        id: String,
        name: String,
        description: String,
        provider_name: String,
        skills: Vec<SkillManifest>,
        npk_hex: &str,
    ) -> Self {
        Self {
            id,
            name,
            description,
            provider: AgentProvider {
                name: provider_name,
                website: None,
                contact: None,
            },
            skills,
            capabilities: AgentCapabilities {
                streaming: true,
                push_notifications: false,
                extended_agent_card: false,
            },
            security_schemes: serde_json::Map::new(),
            security: vec![],
            interfaces: vec![AgentInterface {
                uri: super::BINDING_URI.into(),
                endpoint: format!("waku:{}", super::topics::task_inbox(npk_hex)),
                version: "0.1".into(),
            }],
            extensions: vec![AgentExtension {
                uri: super::PAYMENT_EXT_URI.into(),
                required: true,
                params: serde_json::json!({
                    "token": "LEZ",
                    "settlement": "shielded",
                }),
            }],
            signature: AgentCardSignature {
                algorithm: "ed25519".into(),
                key_id: npk_hex.to_string(),
                value: "<unsigned>".into(),
            },
        }
    }
}
