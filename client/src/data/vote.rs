use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct VoteRecord {
    #[serde(rename = "ALP")]
    alp: u8,
    #[serde(rename = "LNP")]
    lnp: u8,
    #[serde(rename = "ONP")]
    onp: u8,
    #[serde(rename = "UAP")]
    uap: u8,
    #[serde(rename = "AGP")]
    agp: u8,
}

pub const WORDLIST: [&str; 5] = ["ALP", "LNP", "ONP", "UAP", "AGP"];

impl VoteRecord {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        vec![self.alp, self.lnp, self.onp, self.uap, self.agp]
    }

    pub fn validate(&self, input: Option<&str>) -> bool {
        match input {
            Some(a) => a.parse::<u8>().is_ok(),
            None => whole_validate(self),
        }
    }

    pub fn input(&self, user_input: &str, party: &str) -> Self {
        let mut data: VoteRecord = self.clone();

        let user_input = user_input.parse::<u8>().unwrap();

        match party {
            "ALP" => data.alp = user_input,
            "LNP" => data.lnp = user_input,
            "ONP" => data.onp = user_input,
            "UAP" => data.uap = user_input,
            "AGP" => data.agp = user_input,
            _ => panic!("impossible"),
        };

        data
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

pub fn whole_validate(input: &VoteRecord) -> bool {
    let vec = input.to_vec();

    // beyond range
    for x in vec.clone() {
        if !(matches!(x, 1..=5)) {
            return false;
        }
    }

    // mutual equality
    let mut iter = 0;
    for x in vec {
        iter += x;
    }
    if iter != 15 {
        return false;
    }

    // else return valid
    true
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct DataUserVote {
    pub vote: VoteRecord,
    pub password: String,
}
