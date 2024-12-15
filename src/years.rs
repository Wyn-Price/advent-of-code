use crate::{macro_create_year_mod, Part};

mod macros;

use paste::paste;
macro_create_year_mod!(2021);
macro_create_year_mod!(2022);
macro_create_year_mod!(2023);
macro_create_year_mod!(2024);

pub fn run(year: i32, day: i32, part: Part, input: &str) {
    match year {
        2021 => y2021::run(day, part, input),
        2022 => y2022::run(day, part, input),
        2023 => y2023::run(day, part, input),
        2024 => y2024::run(day, part, input),
        _ => panic!("Unknown year {year}"),
    }
}
