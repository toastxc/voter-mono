use rocket::{http::Status, serde::json::Json};
use voteengine::{
    data::{
        auth::DataAuthRequest,
        users::{DataUserRegister, User},
        vote::DataUserVote,
    },
    http::{self, common::EnErr},
};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    String::from("Welcome to the voting machine\nplease register")
}

// account
#[post("/register", data = "<user>")]
async fn register(user: Json<DataUserRegister>) -> Result<String, Status> {
    http::user::register(user.0).await
}

#[get("/profile/<user>/<password>")]
async fn profile(user: String, password: String) -> Result<Json<User>, Status> {
    http::user::profile(user, password).await
}

// vote
#[post("/vote/<user>", data = "<data>")]
async fn vote(user: String, data: Json<DataUserVote>) -> Status {
    crate::http::vote::makevote(user, data.0).await
}

// sudo space
#[post("/dev/key", data = "<data>")]
async fn keymake(data: Json<DataAuthRequest>) -> Result<String, Status> {
    http::dev::make_regkey(data.0, true).await
}

#[delete("/dev/key", data = "<data>")]
async fn keydel(data: Json<DataAuthRequest>) -> Status {
    http::dev::delete_regkey(data.0, true).await
}

#[put("/dev/key", data = "<data>")]
async fn keyls(data: Json<DataAuthRequest>) -> Result<Json<Vec<String>>, Status> {
    http::dev::list_keys(data.0, true).await
}

#[get("/test")]
async fn test() -> String {
    http::dev::test()
}

#[launch]
fn rocket() -> _ {
    // prerun config
    if let Err(EnErr::Fs) = crate::http::common::encrypt("example") {
        crate::http::common::mkecrypt()
    };

    rocket::build().mount(
        "/",
        routes![index, register, keydel, keymake, keyls, test, profile, vote],
    )
}
