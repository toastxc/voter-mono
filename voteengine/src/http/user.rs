use bson::doc;
use rocket::{http::Status, serde::json::Json};

use super::common::{encrypt, RNG};
use crate::data::{
    auth::DataAuthRequest,
    db::DB,
    users::{DataUserRegister, Electorate, Months, User},
};

pub async fn register(user: DataUserRegister) -> Result<String, Status> {
    // import DB
    let db = DB::users().await;

    // if key is valid, remove the key from db
    if !crate::http::dev::key_isvalid(&user.token).await {
        return Err(Status::Unauthorized);
    } else {
        let data = DataAuthRequest {
            key: Some(user.token),
            ..Default::default()
        };

        crate::http::dev::delete_regkey(data, false).await
    };

    // check for invalid default enums
    match (&user.electorate, &user.dob.month) {
        (Electorate::Void, _) => return Err(Status::BadRequest),
        (_, Months::Void) => return Err(Status::BadRequest),
        _ => {}
    };

    // generate new user
    let new_user = User {
        userid: RNG::small(),
        firstname: user.firstname,
        lastname: user.lastname,
        electorate: user.electorate,
        password_hash: encrypt(&user.password).expect("failure to generate key"),
        hasvoted: false,
        dob: user.dob,
        drivers: user.drivers,
    };

    // insert user
    match db.insert_one(&new_user, None).await {
        Ok(_) => Ok(new_user.userid.to_string()),
        Err(_) => Err(Status::UnprocessableEntity),
    }
}

pub async fn profile(user: String, _password: String) -> Result<Json<User>, Status> {
    // if user input is a real number
    if let Ok(user) = user.parse::<u32>() {
        // import database
        let db = DB::users().await;

        // if exists return user json
        return match db.find_one(doc!("userid": user), None).await {
            Ok(Some(a)) => Ok(Json(a)),
            Ok(None) => Err(Status::NotFound),
            Err(_) => Err(Status::BadRequest),
        };
    };

    Err(Status::BadRequest)
}
