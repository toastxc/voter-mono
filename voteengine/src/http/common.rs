use argon2::Config;

pub struct RNG {}
impl RNG {
    pub fn small() -> u32 {
        rand::random()
    }
    pub fn large() -> u64 {
        rand::random()
    }
}

#[derive(Debug)]
pub enum EnErr {
    Fs,
    Verify,
    Hash,
}
pub fn encrypt(password: &str) -> Result<String, EnErr> {
    let password = password.as_bytes();

    let salt = match std::fs::read("salt.txt") {
        Ok(a) => a,
        Err(_) => return Err(EnErr::Fs),
    };

    let config = Config::default();
    let hash = match argon2::hash_encoded(password, &salt, &config) {
        Ok(a) => a,
        Err(_) => return Err(EnErr::Hash),
    };

    match argon2::verify_encoded(&hash, password) {
        Ok(true) => Ok(hash),
        _ => Err(EnErr::Verify),
    }
}
