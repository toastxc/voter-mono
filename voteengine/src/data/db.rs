use mongodb::{Collection, Database};

use super::{auth::Key, users::User};

pub struct DB {}

impl DB {
    pub async fn db() -> Database {
        easydb().await
    }
    pub async fn users() -> Collection<User> {
        easydb().await.collection::<User>("users")
    }
    pub async fn keys() -> Collection<Key> {
        easydb().await.collection::<Key>("keys")
    }
}

async fn easydb() -> Database {
    easymongo::mongo::Mongo::new()
        .username("username")
        .password("password")
        .database("test")
        .db_generate()
        .await
}
