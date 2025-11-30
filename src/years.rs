use std::{fmt::Display, time::Instant};

use crate::{macro_create_year_mod_12, macro_create_year_mod_25, Part};

mod macros;

use paste::paste;
macro_create_year_mod_25!(2015);

macro_create_year_mod_25!(2021);
macro_create_year_mod_25!(2022);
macro_create_year_mod_25!(2023);
macro_create_year_mod_25!(2024);
macro_create_year_mod_12!(2025);
// insert: new year create

pub type SolveFn = Box<dyn Fn(&str) -> Box<dyn Display>>;

pub fn run(year: i32, day: i32, part: Part, input: &str) -> (Part, String) {
    match year {
        2015 => run_with_questions(y2015::questions(), day, part, input),

        2021 => run_with_questions(y2021::questions(), day, part, input),
        2022 => run_with_questions(y2022::questions(), day, part, input),
        2023 => run_with_questions(y2023::questions(), day, part, input),
        2024 => run_with_questions(y2024::questions(), day, part, input),
        2025 => run_with_questions(y2025::questions(), day, part, input),
        // insert: new year run
        _ => panic!("Unknown year {year}"),
    }
}

pub fn run_with_questions<const N: usize>(
    questions: [[SolveFn; 2]; N],
    day: i32,
    part: Part,
    input: &str,
) -> (Part, String) {
    let current_day = &questions[day as usize - 1];

    let mut to_submit = None;
    if part.is_a() {
        let start = Instant::now();
        let res = current_day[0](input);
        let elapsed = start.elapsed();
        println!(
            "Day {day} part A returned: {res} in {}ms",
            elapsed.as_millis_f32()
        );
        to_submit = Some((Part::A, format!("{res}")));
    }

    if part.is_b() {
        let start = Instant::now();
        let res = current_day[1](input);
        let elapsed = start.elapsed();
        println!(
            "Day {day} part B returned: {res} in {}ms",
            elapsed.as_millis_f32()
        );
        to_submit = Some((Part::B, format!("{res}")));
    }

    return to_submit.unwrap();
}
