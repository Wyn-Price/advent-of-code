use std::fs;

use colored::Colorize;
use regex::Regex;

use crate::Part;

pub async fn download_input(year: i32, day: i32) -> Result<String, reqwest::Error> {
    reqwest::Client::new()
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header("Cookie", format!("session={}", get_session()))
        .send()
        .await?
        .text()
        .await
}

pub async fn submit_part(
    year: i32,
    day: i32,
    part: Part,
    answer: String,
) -> Result<Response, reqwest::Error> {
    let level = match part {
        Part::A => "1",
        Part::B => "2",
        Part::BOTH => panic!("Invalid part for submission"),
    };
    let text = reqwest::Client::new()
        .post(format!("https://adventofcode.com/{year}/day/{day}/answer"))
        .header("Cookie", format!("session={}", get_session()))
        .form(&[("level", level), ("answer", &answer)])
        .send()
        .await?
        .text()
        .await?;

    let re = Regex::new(r"(?s)<article>(.+)</article>").unwrap();
    let body = re
        .captures(&text)
        .expect(&format!("Unknown HTML {text}"))
        .get(1)
        .unwrap()
        .as_str()
        .to_owned();

    return Ok(Response::best_guess(&body));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Response {
    WrongLevel,
    WrongAnswer(Option<String>, String),
    RateLimited(String),
    Corret,
    Finished,
    Other(String),
}

impl Response {
    pub fn best_guess(body: &str) -> Self {
        // You don't seem to be solving the right level.
        let wrong_level_re =
            Regex::new(r"(?i)You don't seem to be solving the right level").unwrap();
        if wrong_level_re.is_match(body) {
            return Self::WrongLevel;
        }

        let wrong_answer_re =
            Regex::new(r"(?i)That's not the right answer;?\.?(.+)If you're stuck").unwrap();
        if let Some(caps) = wrong_answer_re.captures(body) {
            let hint = caps
                .get(1)
                .map(|c| c.as_str().trim().to_owned())
                .filter(|s| !s.is_empty());
            let to_wait = Regex::new("(?i)Please wait(.+)before trying again")
                .unwrap()
                .captures(body)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .to_owned();
            return Self::WrongAnswer(hint, to_wait);
        }

        let ratelimit_re = Regex::new(r"(?i)You gave an answer too recently; you have to wait after submitting an answer before trying again.\s+You have(.+)left to wait.").unwrap();
        if let Some(caps) = ratelimit_re.captures(body) {
            return Self::RateLimited(caps.get(1).unwrap().as_str().trim().to_owned());
        }

        let correct_re = Regex::new("(?i)That's the right answer!").unwrap();
        if correct_re.is_match(body) {
            return Self::Corret;
        }

        let finished_re =
            Regex::new(r"(?i)You've finished every puzzle in Advent of Code").unwrap();
        if finished_re.is_match(body) {
            return Self::Finished;
        }

        return Self::Other(body.to_owned());
    }

    pub fn pretty_text(&self) -> String {
        match &self {
            Self::WrongLevel => "Wrong level".yellow().to_string(),
            Self::WrongAnswer(hint, ratelimit) => {
                let mut txt = "Wrong Answer.".red().to_string();
                if let Some(h) = hint {
                    txt += &format!(" ({})", h).bold().blue().to_string();
                }
                txt += &format!("\nWait: {}", ratelimit).yellow().to_string();
                return txt;
            }
            Self::RateLimited(time) => {
                "Ratelimited!!".bold().red().underline().to_string()
                    + " "
                    + &format!("Wait {}.", time).red().to_string()
            }
            Self::Corret => "Correct!".bold().green().to_string(),
            Self::Finished => "Correct - All Finished!"
                .bold()
                .green()
                .underline()
                .to_string(),
            Self::Other(text) => "Unknown ".bright_black().to_string() + text,
        }
    }
}

fn get_session() -> String {
    fs::read_to_string(".session")
        .expect("Unable to read session token from file")
        .trim()
        .to_string()
}
