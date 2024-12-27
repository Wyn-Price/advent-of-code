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
use colored::Colorize;
use dialoguer::Confirm;
use std::{fs, sync::Arc, time::Duration};
use tokio::sync::Notify;

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

    let (part, to_submit) = years::run(year, day, part, &input.trim());
    let confirmation = Confirm::new()
        .with_prompt(format!("Do you want to submit {to_submit}?"))
        .interact()
        .unwrap();
    if confirmation {
        if day == 25 && part == Part::A {
            minimum_delta_day_25_submit(year, to_submit).await;
        } else {
            let response = 'main: loop {
                let response = aoc::submit_part(year, day, part, to_submit.clone())
                    .await
                    .unwrap();
                println!("{}", response.pretty_text());

                if let aoc::Response::RateLimited(_, time_o) = response {
                    if let Some(time) = time_o {
                        ratelimit_wait(time).await;
                        continue 'main;
                    }
                }

                break response;
            };

            if response == aoc::Response::Corret {
                let url = match part {
                    Part::A => format!("https://adventofcode.com/{year}/day/{day}#part2"),
                    Part::B => {
                        format!("https://adventofcode.com/{year}/leaderboard/private/view/413619")
                    }
                    _ => panic!("Invalid part {part:?}"),
                };
                open::that(url).unwrap();
            }
        }
    }
}

async fn ratelimit_wait(time: u64) {
    for i in 0..time {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!(
            "{}",
            format!(" - {}", aoc::seconds_to_str(time - i - 1))
                .red()
                .to_string()
        )
    }
}

async fn minimum_delta_day_25_submit(year: i32, to_submit: String) {
    let done_a = Arc::new(Notify::new());
    let done_b = Arc::new(Notify::new());

    let done_a_cloned = done_a.clone();
    tokio::spawn(async move {
        println!("Sending 25 Part A");
        let response = aoc::submit_part(year, 25, Part::A, to_submit)
            .await
            .unwrap();
        println!("25 Part A: {}", response.pretty_text());
        done_a_cloned.notify_one();
    });

    let done_a_cloned = done_a.clone();
    let mut interval = tokio::time::interval(Duration::from_millis(100));
    let mut handles = vec![];
    let mut i = 0;
    loop {
        i += 1;

        let done_b_cloned = done_b.clone();
        handles.push(tokio::spawn(async move {
            println!("Sending 25 Part B {i}");
            let response = match aoc::submit_part(year, 25, Part::B, "0".to_owned()).await {
                Ok(r) => r,
                Err(e) => aoc::Response::Other(e.to_string()),
            };
            if response == aoc::Response::Finished {
                done_b_cloned.notify_one();
            }
            println!("25 Part B {i}: {}", response.pretty_text());
        }));

        if tokio::select! {
            _ = done_a_cloned.notified() => { true },
            _ = interval.tick() => { false },
        } {
            break;
        };
    }

    let done_b_cloned = done_b.clone();
    done_b_cloned.notified().await;

    handles.iter().for_each(|h| h.abort());
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
