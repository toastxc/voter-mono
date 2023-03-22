pub mod data {
    pub mod admin;
    pub mod register;
    pub mod vote;
}
pub mod http {
    pub mod admin;
    pub mod common;
    pub mod user;
}
#[tokio::main]
async fn main() {}

/*
async fn profile() {
    let curl = reqwest::Client::new()
        .get("http://localhost:8000/profile/3602725565/no")
        //.body(serde_json::to_string(&data).unwrap())
        .send()
        .await
        .unwrap();

    println!("result: {:#?}", curl.text().await);
}

async fn register() {
    let dob = DateOfBirth {
        day: 26,
        month: Months::May,
        year: 2005,
    };

    let data = DataUserRegister {
        firstname: String::from("Kaia"),
        lastname: String::from("Collett"),
        electorate: Electorate::Canning,
        password: String::from("no"),
        token: String::from("1444542005256457878"),
        dob,
        ..Default::default()
    };

    let curl = reqwest::Client::new()
        .post("http://localhost:8000/register")
        .body(serde_json::to_string(&data).unwrap())
        .send()
        .await
        .unwrap();

    println!("{:#?}", curl.text().await);
}

pub async fn vote() {
    let mut vote = VoteRecord::new();

    println!("give preference for candidates in decending order (1 to 5)");

    // for every candidate
    for x in WORDLIST {
        loop {
            println!("{x}");

            // read input
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();

            let input = input.trim();
            // if input is invalid, add it and break loop
            if vote.validate(Some(input)) {
                vote = vote.input(input, x);
                break;
            }
        }
    }

    if !vote.validate(None) {
        println!("Vote invalid, process failed");
        return;
    };
    let votedata = DataUserVote {
        vote,
        password: String::from("no"),
    };

    let curl = reqwest::Client::new()
        .post("http://localhost:8000/vote/3602725565")
        .body(serde_json::to_string(&votedata).unwrap())
        .send()
        .await
        .unwrap();

    println!("{:#?}", curl.text().await);
}

*/
