#![feature(iter_array_chunks)]
#![feature(array_windows)]
mod days;

use std::fs;

static YEAR: &str = "2022";

#[tokio::main]
async fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("No day provided")
        .parse::<usize>()
        .expect("Day provided was not a number");
    let part = std::env::args().nth(2).map_or_else(
        || Part::BOTH,
        |x| match x.as_str() {
            "a" => Part::A,
            "b" => Part::B,
            _ => panic!("Expected 'a', 'b', got '{x}'"),
        },
    );

    let dir = "./inputs";
    let path = &format!("{dir}/{day}.txt");
    let input = match fs::read_to_string(path) {
        Ok(f) => f.to_owned(),
        Err(err) => {
            println!("Unable to find file {path}: {err}. Attempting download.");
            let str = download_input(day)
                .await
                .expect("Error while downloading input");
            fs::create_dir_all(dir).expect(&format!("Unable to create dir {dir}"));
            fs::write(path, &str).expect(&format!("Unable to write to {path}"));
            str
        }
    };

    days::run_for_day(day, &input, &part)
}

async fn download_input(day: usize) -> Result<String, reqwest::Error> {
    let session = fs::read_to_string(".session").expect("Unable to read session token from file");
    reqwest::Client::new()
        .get(format!("https://adventofcode.com/{YEAR}/day/{day}/input"))
        .header("Cookie", format!("session={session}"))
        .send()
        .await?
        .text()
        .await
}

pub enum Part {
    A,
    B,
    BOTH,
}

impl Part {
    fn is_a(&self) -> bool {
        match self {
            Part::A | Part::BOTH => true,
            Part::B => false,
        }
    }

    fn is_b(&self) -> bool {
        match self {
            Part::B | Part::BOTH => true,
            Part::A => false,
        }
    }
}
