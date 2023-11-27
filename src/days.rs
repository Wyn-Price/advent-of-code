use crate::Part;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

static DAY_PARTS: [[for<'r> fn(&'r str) -> i64; 2]; 25] = [
    [day01::part_a, day01::part_b],
    [day02::part_a, day02::part_b],
    [day03::part_a, day03::part_b],
    [day04::part_a, day04::part_b],
    [day05::part_a, day05::part_b],
    [day06::part_a, day06::part_b],
    [day07::part_a, day07::part_b],
    [day08::part_a, day08::part_b],
    [day09::part_a, day09::part_b],
    [day10::part_a, day10::part_b],
    [day11::part_a, day11::part_b],
    [day12::part_a, day12::part_b],
    [day13::part_a, day13::part_b],
    [day14::part_a, day14::part_b],
    [day15::part_a, day15::part_b],
    [day16::part_a, day16::part_b],
    [day17::part_a, day17::part_b],
    [day18::part_a, day18::part_b],
    [day19::part_a, day19::part_b],
    [day20::part_a, day20::part_b],
    [day21::part_a, day21::part_b],
    [day22::part_a, day22::part_b],
    [day23::part_a, day23::part_b],
    [day24::part_a, day24::part_b],
    [day25::part_a, day25::part_b],
];

pub fn run_for_day(day: usize, input: &str, part: &Part) {
    let current_day = DAY_PARTS[day - 1];

    if part.is_a() {
        let res = current_day[0](input);
        println!("Day {day} part A returned: {res}");
    }

    if part.is_b() {
        let res = current_day[1](input);
        println!("Day {day} part B returned: {res}");
    }
}
