#[macro_export]
macro_rules! macro_create_day {
    ($day:ident) => {
        [
            Box::new(|i| Box::new($day::part_a(i))),
            Box::new(|i| Box::new($day::part_b(i))),
        ]
    };
}

#[macro_export]
macro_rules! macro_create_year_mod {
    ($year:tt) => {
        paste! {
            mod [<y $year>] {
                use crate::macro_import_year;
                macro_import_year!();
            }
        }
    };
}

#[macro_export]
macro_rules! macro_import_year {
    () => {
        use std::fmt::Display;
        use std::time::Instant;

        use crate::macro_create_day;
        use crate::Part;

        pub mod day01;
        pub mod day02;
        pub mod day03;
        pub mod day04;
        pub mod day05;
        pub mod day06;
        pub mod day07;
        pub mod day08;
        pub mod day09;
        pub mod day10;
        pub mod day11;
        pub mod day12;
        pub mod day13;
        pub mod day14;
        pub mod day15;
        pub mod day16;
        pub mod day17;
        pub mod day18;
        pub mod day19;
        pub mod day20;
        pub mod day21;
        pub mod day22;
        pub mod day23;
        pub mod day24;
        pub mod day25;

        type SolveFn = Box<dyn Fn(&str) -> Box<dyn Display>>;

        fn questions() -> [[SolveFn; 2]; 25] {
            [
                macro_create_day!(day01),
                macro_create_day!(day02),
                macro_create_day!(day03),
                macro_create_day!(day04),
                macro_create_day!(day05),
                macro_create_day!(day06),
                macro_create_day!(day07),
                macro_create_day!(day08),
                macro_create_day!(day09),
                macro_create_day!(day10),
                macro_create_day!(day11),
                macro_create_day!(day12),
                macro_create_day!(day13),
                macro_create_day!(day14),
                macro_create_day!(day15),
                macro_create_day!(day16),
                macro_create_day!(day17),
                macro_create_day!(day18),
                macro_create_day!(day19),
                macro_create_day!(day20),
                macro_create_day!(day21),
                macro_create_day!(day22),
                macro_create_day!(day23),
                macro_create_day!(day24),
                macro_create_day!(day25),
            ]
        }

        pub fn run(day: i32, part: Part, input: &str) -> Vec<(Part, String)> {
            let current_day = &questions()[day as usize - 1];
            let mut to_submit = vec![];

            if part.is_a() {
                let start = Instant::now();
                let res = current_day[0](input);
                let elapsed = start.elapsed();
                println!(
                    "Day {day} part A returned: {res} in {}ms",
                    elapsed.as_millis_f32()
                );
                to_submit.push((Part::A, format!("{res}")));
            }

            if part.is_b() {
                let start = Instant::now();
                let res = current_day[1](input);
                let elapsed = start.elapsed();
                println!(
                    "Day {day} part B returned: {res} in {}ms",
                    elapsed.as_millis_f32()
                );
                to_submit.push((Part::B, format!("{res}")));
            }

            return to_submit;
        }
    };
}
