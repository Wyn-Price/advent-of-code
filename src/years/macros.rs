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
macro_rules! macro_create_year_mod_25 {
    ($year:tt) => {
        paste! {
            mod [<y $year>] {
                use crate::macro_import_year_25;
                macro_import_year_25!();
            }
        }
    };
}

#[macro_export]
macro_rules! macro_create_year_mod_12 {
    ($year:tt) => {
        paste! {
            mod [<y $year>] {
                use crate::macro_import_year_12;
                macro_import_year_12!();
            }
        }
    };
}

#[macro_export]
macro_rules! macro_import_year_25 {
    () => {
        use crate::macro_create_day;
        use crate::years::SolveFn;

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

        pub fn questions() -> [[SolveFn; 2]; 25] {
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
                [
                    Box::new(|i| Box::new(day25::part_a(i))),
                    Box::new(|_| Box::new("0".to_owned())),
                ],
            ]
        }
    };
}

#[macro_export]
macro_rules! macro_import_year_12 {
    () => {
        use crate::macro_create_day;
        use crate::years::SolveFn;

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

        pub fn questions() -> [[SolveFn; 2]; 12] {
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
            ]
        }
    };
}
