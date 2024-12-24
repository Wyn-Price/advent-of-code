#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(duration_millis_float)]
#![feature(int_roundings)]
#![feature(iter_array_chunks)]
extern crate chrono;
extern crate scan_fmt;

mod aoc;
mod years;

use chrono::Datelike;
use dialoguer::Confirm;
use std::fs;

#[tokio::main]
async fn main() {
    let year_or_day = std::env::args()
        .nth(1)
        .expect("No year/day provided")
        .parse::<i32>()
        .expect("Year/day provided was not a number");

    let is_first_arg_year = year_or_day > 2000;

    let year = if is_first_arg_year {
        year_or_day
    } else {
        chrono::Utc::now().year()
    };

    let day = if is_first_arg_year {
        std::env::args()
            .nth(2)
            .expect("No day provided")
            .parse::<i32>()
            .expect("Day provided was not a number")
    } else {
        year_or_day
    };

    let part = std::env::args()
        .nth(if is_first_arg_year { 3 } else { 2 })
        .map_or_else(
            || Part::BOTH,
            |x| match x.as_str() {
                "a" => Part::A,
                "b" => Part::B,
                _ => panic!("Expected 'a', 'b', got '{x}'"),
            },
        );

    let dir = &format!("./inputs/{year}");
    fs::create_dir_all(dir).unwrap();
    let path = &format!("{dir}/{day}.txt");
    let input = match fs::read_to_string(path) {
        Ok(f) => f.to_owned(),
        Err(err) => {
            println!("Unable to find file {path}: {err}. Attempting download.");
            let str = aoc::download_input(year, day)
                .await
                .expect("Error while downloading input");
            fs::create_dir_all(dir).expect(&format!("Unable to create dir {dir}"));
            fs::write(path, &str).expect(&format!("Unable to write to {path}"));
            str
        }
    };

    for (part, to_submit) in years::run(year, day, part, &input) {
        let confirmation = Confirm::new()
            .with_prompt(format!("Do you want to submit {to_submit}?"))
            .interact()
            .unwrap();
        if confirmation {
            let text = aoc::submit_part(year, day, part, to_submit).await.unwrap();
            let response = aoc::Response::best_guess(&text)
                .unwrap_or(aoc::Response::Other(format!("Unable to guess: {}", text)));
            println!("{}", response.pretty_text());
        }
    }
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
