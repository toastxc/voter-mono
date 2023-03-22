use reqwest::StatusCode;

use crate::data::admin::DataAuthRequest;

use super::common::Error;

impl DataAuthRequest {
    pub fn new(id: &str, key: Option<&str>) -> Self {
        Self {
            id: String::from(id),
            key: key.map(String::from),
        }
    }

    pub async fn generate(&self) -> Result<String, Error> {
        let curl = reqwest::Client::new()
            .post("http://localhost:8000/dev/key")
            .body(serde_json::to_string(&self).unwrap())
            .send()
            .await
            .unwrap();

        let status = curl.status();
        match (status.is_success(), curl.text().await) {
            (true, Ok(data)) => Ok(data),
            (true, Err(json_e)) => Err(Error::Json(json_e)),
            (false, _) => Err(Error::Http(status)),
        }
    }

    pub async fn list(&self) -> Result<Vec<String>, Error> {
        let curl = reqwest::Client::new()
            .put("http://localhost:8000/dev/key")
            .body(serde_json::to_string(&self).unwrap())
            .send()
            .await
            .unwrap();

        let status = curl.status();
        match (status.is_success(), curl.json::<Vec<String>>().await) {
            (true, Ok(data)) => Ok(data),
            (true, Err(json_e)) => Err(Error::Json(json_e)),
            (false, _) => Err(Error::Http(status)),
        }
    }
    pub async fn delete(&self, key: Option<&str>) -> Result<(), StatusCode> {
        let data = match key {
            Some(a) => Self {
                id: self.id.clone(),
                key: Some(String::from(a)),
            },
            None => self.clone(),
        };

        let curl = reqwest::Client::new()
            .delete("http://localhost:8000/dev/key")
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap()
            .status();

        match curl.as_u16() {
            200 => Ok(()),
            _ => Err(curl),
        }
    }
}
