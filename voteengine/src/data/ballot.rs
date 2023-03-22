use crate::data::vote::VoteRecord;
use rand::Rng;
use serde::{Deserialize, Serialize};

impl Ballot {
    pub fn add(&self, vote: &VoteRecord) {
        let number: u32 = rand::thread_rng().gen();

        let filename = format!("vote-{}-{}", self.year, number);

        if std::fs::read_dir(self.directory.clone()).is_err() {
            std::fs::create_dir(self.directory.clone()).unwrap();
        }

        std::fs::write(
            format!("{}/{filename}.json", self.directory),
            vote.to_json(),
        )
        .unwrap();
    }

    pub fn import(directory: &str) -> Self {
        if let Err(error) = std::fs::read(directory) {
            println!("{error}");
            panic!();
        }

        todo!()
    }
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Ballot {
    directory: String,
    year: u32,
}
