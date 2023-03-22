use core::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub userid: u32,
    pub dob: DateOfBirth,
    pub firstname: String,
    pub lastname: String,
    pub electorate: Electorate,
    pub password_hash: String,
    pub hasvoted: bool,
    pub drivers: DriversLicense,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DataUserRegister {
    pub firstname: String,
    pub lastname: String,
    pub electorate: Electorate,
    pub password: String,
    pub dob: DateOfBirth,
    pub drivers: DriversLicense,
    pub token: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DateOfBirth {
    pub day: u8,
    pub month: Months,
    pub year: u16,
}
impl DateOfBirth {
    pub fn is_minor(&self) -> bool {
        // i hate math

        // fuck you chrono

        true
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct DriversLicense {
    number: u32,
    backnumber: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Months {
    January,
    Febuary,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
    #[default]
    Void,
}

impl fmt::Display for Months {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Auth {
    pub firstxlast: String,
    pub password: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub enum Electorate {
    #[default]
    Void,
    Brand,
    Burt,
    Canning,
    Cowan,
    Curtin,
    Durack,
    Forest,
    Fremantle,
    Hasluck,
    Moore,
    OConnor,
    Pearce,
    Perth,
    Swan,
    Tangney,
}
impl fmt::Display for Electorate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
