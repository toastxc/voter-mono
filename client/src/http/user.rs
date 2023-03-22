use crate::data::{register::DataUserRegister, user::UserDetails};

use super::common::Error;

#[derive(Debug, Clone, Default)]
pub struct UserData {
    pub userid: String,
}
impl UserData {
    pub fn new() -> Self {
        Self {
            userid: String::new()
        }
    }
}
pub struct UserClient {
    pub http_client: reqwest::Client,
    pub data: UserData,
    pub endpoint: String,
}
impl UserClient {
    pub fn new(endpoint: Option<&str>) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            data: UserData::new(),
            endpoint: String::from(endpoint.unwrap_or("http://localhost:8000"))
        }
    }

    pub async fn register(&self, data: DataUserRegister) -> Result<UserData, Error> {
        let endpoint = self.endpoint.clone();
        let response = self.http_client
            .post(format!("{endpoint}/register"))
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap();

        let status = response.status();
        match (status.is_success(), response.text().await) {
            (false, _) => Err(Error::Http(status)),
            (true, Err(a)) => Err(Error::Json(a)),
            (true, Ok(a)) => Ok(UserData { userid: a }),
        }
    }

    pub async fn profile(&self, password: &str) -> Result<UserDetails, Error> {
        let user_id = self.data.userid.clone();
        let endpoint = self.endpoint.clone();
        let response = self.http_client
            .get(format!("{endpoint}/profile/{user_id}/{password}"))
            .send()
            .await
            .unwrap();

        let status = response.status();
        match (status.is_success(), response.json::<UserDetails>().await) {
            (false, _) => Err(Error::Http(status)),
            (true, Err(a)) => Err(Error::Json(a)),
            (true, Ok(a)) => Ok(a),
        }
    }

    pub async fn from_id(userid: &str) -> Self {
        let mut item = UserClient::new(None);
        item.data.userid = userid.to_string();

        item
    }
}
