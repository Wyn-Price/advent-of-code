use crate::Part;

mod base;
mod y2021 {
    use crate::macro_import_year;
    macro_import_year!();
}

pub fn run(year: i32, day: i32, part: Part, input: &str) {
    match year {
        2021 => y2021::run(day, part, input),
        _ => panic!("Unknown year {year}"),
    }
}
