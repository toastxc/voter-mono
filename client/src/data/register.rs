use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
pub struct DateOfBirth {
    pub day: u8,
    pub month: Months,
    pub year: u16,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct DriversLicense {
    pub number: u32,
    pub backnumber: String,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
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
