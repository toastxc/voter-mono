use bson::doc;
use futures::StreamExt;
use rocket::{http::Status, serde::json::Json};

use crate::data::{
    auth::{Authority, DataAuthRequest, Key},
    db::DB,
    vote::DataUserVote,
};

pub async fn make_regkey(data: DataAuthRequest, http: bool) -> Result<String, Status> {
    if http && !auth(&data.id) {
        return Err(Status::Unauthorized);
    };

    let keystore = DB::keys().await;

    let data = Key::new();

    match keystore.insert_one(&data, None).await {
        Ok(_) => Ok(data.key.to_string()),
        Err(_) => Err(Status::InternalServerError),
    }
}

pub async fn delete_regkey(data: DataAuthRequest, http: bool) -> Status {
    if http && !auth(&data.id) {
        return Status::Unauthorized;
    };

    if data.key.is_none() {
        return Status::NoContent;
    }
    let keystore = DB::keys().await;

    let query = doc!("key": &data.key);

    match keystore.delete_one(query, None).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}

pub async fn list_keys(data: DataAuthRequest, http: bool) -> Result<Json<Vec<String>>, Status> {
    if http && !auth(&data.id) {
        return Err(Status::Unauthorized);
    };
    let keystore = DB::keys().await;

    let mut keys = match keystore.find(doc! {}, None).await {
        Ok(a) => a,
        Err(_) => return Err(Status::InternalServerError),
    };

    let mut tokenvec = Vec::new();
    while let Some(Ok(document)) = keys.next().await {
        tokenvec.push(document.key);
    }
    Ok(Json(tokenvec))
}

pub async fn key_isvalid(key: &str) -> bool {
    let keystore = DB::keys().await;

    let mut master = Vec::new();
    let mut keys = keystore.find(doc! {"key": key}, None).await.unwrap();

    while let Some(Ok(document)) = keys.next().await {
        master.push(document)
    }

    match master.len() {
        0 => false,
        1 => true,
        _ => panic!("invalid option"),
    }
}

fn auth(user: &str) -> bool {
    Authority::read("auth.json").unwrap().is_sudo(user)
}
pub fn test() -> String {
    let user = DataUserVote::default();
    serde_json::to_string_pretty(&user).unwrap()
}
