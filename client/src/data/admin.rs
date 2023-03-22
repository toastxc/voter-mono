use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataAuthRequest {
    pub id: String,
    pub key: Option<String>,
}
impl DataAuthRequest {
    pub fn new(id: &str, key: Option<&str>) -> Self {
        Self {
            id: String::from(id),
            key: key.map(String::from),
        }
    }
}
