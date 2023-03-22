use bson::doc;
use rocket::http::Status;

use crate::data::{
    db::DB,
    vote::{whole_validate, DataUserVote},
};

use super::common::encrypt;
pub async fn makevote(user: String, data: DataUserVote) -> Status {
    let db = DB::users().await;

    let user: u32 = match user.parse() {
        Ok(a) => a,
        Err(_) => return Status::BadRequest,
    };

    // define user
    let user = match db.find_one(doc!("userid": user), None).await {
        Ok(Some(a)) => a,
        Ok(None) => return Status::NotFound,
        Err(_) => return Status::InternalServerError,
    };

    // user vailidation
    if user.hasvoted {
        println!("already voted");
        return Status::Forbidden;
    }

    // password validation

    let newen = encrypt(&data.password).unwrap();

    if user.password_hash != newen {
        println!("password failure");
        return Status::Unauthorized;
    };

    // vote validation check
    if !whole_validate(&data.vote) {
        return Status::BadRequest;
    }

    // passed all checks

    let mut user_mod = user;

    user_mod.hasvoted = true;

    let updater = doc! {"$set": {"hasvoted": true}};

    match db
        .update_one(doc! {"userid": user_mod.userid}, updater, None)
        .await
    {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError,
    }
}
