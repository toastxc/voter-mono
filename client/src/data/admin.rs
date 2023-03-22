use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataAuthRequest {
    pub id: String,
    pub key: Option<String>,
}
