use crate::data::{register::DataUserRegister, user::UserDetails};

use super::common::Error;

#[derive(Debug, Clone)]
pub struct User {
    pub userid: String,
}

impl User {
    pub async fn register(data: DataUserRegister) -> Result<Self, Error> {
        let curl = reqwest::Client::new()
            .post("http://localhost:8000/register")
            .body(serde_json::to_string(&data).unwrap())
            .send()
            .await
            .unwrap();

        let status = curl.status();
        match (status.is_success(), curl.text().await) {
            (false, _) => Err(Error::Http(status)),
            (true, Err(a)) => Err(Error::Json(a)),
            (true, Ok(a)) => Ok(Self { userid: a }),
        }
    }

    pub async fn profile(&self, password: &str) -> Result<UserDetails, Error> {
        let curl = reqwest::Client::new()
            .get(format!(
                "http://localhost:8000/profile/{}/{password}",
                self.userid
            ))
            .send()
            .await
            .unwrap();

        let status = curl.status();
        match (status.is_success(), curl.json::<UserDetails>().await) {
            (false, _) => Err(Error::Http(status)),
            (true, Err(a)) => Err(Error::Json(a)),
            (true, Ok(a)) => Ok(a),
        }
    }

    pub fn from_id(userid: &str) -> Self {
        Self {
            userid: String::from(userid),
        }
    }
}
