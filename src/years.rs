use crate::Part;

mod base;
mod y2021 {
    use crate::macro_import_year;
    macro_import_year!();
}

mod y2022 {
    use crate::macro_import_year;
    macro_import_year!();
}

mod y2023 {
    use crate::macro_import_year;
    macro_import_year!();
}

mod y2024 {
    use crate::macro_import_year;
    macro_import_year!();
}

pub fn run(year: i32, day: i32, part: Part, input: &str) {
    match year {
        2021 => y2021::run(day, part, input),
        2022 => y2022::run(day, part, input),
        2023 => y2023::run(day, part, input),
        2024 => y2024::run(day, part, input),
        _ => panic!("Unknown year {year}"),
    }
}
