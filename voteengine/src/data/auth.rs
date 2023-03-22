use serde::{Deserialize, Serialize};

use crate::http::common::RNG;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Key {
    pub key: String,
}

impl Key {
    pub fn new() -> Self {
        Self {
            key: RNG::large().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Authority {
    sudoers: Vec<String>,
}

impl Authority {
    pub fn is_sudo(&self, user: &str) -> bool {
        self.sudoers.contains(&String::from(user))
    }

    pub fn read(file: &str) -> Option<Self> {
        if let Ok(file) = std::fs::read(file) {
            let file = String::from_utf8(file).unwrap();

            if let Ok(content) = serde_json::from_str(&file) {
                return Some(content);
            };
        };
        None
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataAuthRequest {
    pub id: String,
    pub key: Option<String>,
}
