use reqwest::StatusCode;

use crate::data::admin::DataAuthRequest;

use super::common::Error;

// Derrives from DataAuthRequest
pub struct AdminClient {
    pub id: String,
    pub key: Option<String>,
    pub http_client: reqwest::Client,
    pub endpoint: String,
}
impl AdminClient {
    pub fn new(id: &str, key: Option<&str>, endpoint: Option<&str>) -> Self {
        Self {
            id: String::from(id),
            key: key.map(String::from),
            http_client: reqwest::Client::new(),
            endpoint: String::from(endpoint.unwrap_or("http://localhost:8000")),
        }
    }

    fn to_data(&self) -> DataAuthRequest {
        DataAuthRequest {
            id: self.id.clone(),
            key: self.key.clone(),
        }
    }

    pub async fn generate(&self) -> Result<String, Error> {
        let data = self.to_data();
        let endpoint = self.endpoint.clone();
        let response = self
            .http_client
            .post(format!("{endpoint}/dev/key"))
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match (status.is_success(), response.text().await) {
            (true, Ok(data)) => Ok(data),
            (true, Err(json_e)) => Err(Error::Json(json_e)),
            (false, _) => Err(Error::Http(status)),
        }
    }

    pub async fn list(&self) -> Result<Vec<String>, Error> {
        let data = self.to_data();
        let endpoint = self.endpoint.clone();
        let response = self
            .http_client
            .post(format!("{endpoint}/dev/key"))
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match (status.is_success(), response.json::<Vec<String>>().await) {
            (true, Ok(data)) => Ok(data),
            (true, Err(json_e)) => Err(Error::Json(json_e)),
            (false, _) => Err(Error::Http(status)),
        }
    }

    pub async fn delete(&self, key: Option<&str>) -> Result<(), StatusCode> {
        let original_data = self.to_data();
        let data = match key {
            Some(a) => DataAuthRequest {
                id: original_data.id.clone(),
                key: Some(String::from(a)),
            },
            None => original_data.clone(),
        };

        let endpoint = self.endpoint.clone();
        let response = self
            .http_client
            .post(format!("{endpoint}/dev/key"))
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap()
            .status();

        match response.as_u16() {
            200 => Ok(()),
            _ => Err(response),
        }
    }
}
