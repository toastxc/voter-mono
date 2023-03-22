use reqwest::StatusCode;

#[derive(Debug)]
pub enum Error {
    Http(StatusCode),
    Json(reqwest::Error),
}
