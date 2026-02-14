use magic_wormhole::transit::Abilities;
use serde::{Deserialize, Serialize};

/// Default relay URL used by the official wormhole clients.
pub const DEFAULT_RELAY_URL: &str = "wss://relay.magic-wormhole.io";

/// Number of words in the generated wormhole code.
pub const DEFAULT_CODE_LENGTH: usize = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstudelyConfig {
    /// Custom relay URL (None = use default from magic-wormhole crate).
    pub relay_url: Option<String>,
    /// Transit abilities (direct, relay, or both).
    #[serde(skip)]
    pub transit_abilities: Abilities,
    /// Number of words in generated codes.
    pub code_length: usize,
}

impl Default for EstudelyConfig {
    fn default() -> Self {
        Self {
            relay_url: None,
            transit_abilities: Abilities::ALL,
            code_length: DEFAULT_CODE_LENGTH,
        }
    }
}
