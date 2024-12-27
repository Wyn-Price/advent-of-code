use std::collections::HashMap;

use itertools::Itertools;

pub fn part_a(input: &str) -> u16 {
    let mut wires = parse(input);
    return solve(&mut wires, "a");
}

pub fn part_b(input: &str) -> u16 {
    let solved_part_a = part_a(input);
    let mut wires = parse(input);
    wires.insert("b", WireConnection::Computed(solved_part_a));
    return solve(&mut wires, "a");
}

fn solve<'a>(map: &mut WireMap<'a>, wire: &'a str) -> u16 {
    if let Ok(v) = wire.parse::<u16>() {
        return v;
    }
    let v = match map[wire] {
        WireConnection::And(a, b) => solve(map, a) & solve(map, b),
        WireConnection::Or(a, b) => solve(map, a) | solve(map, b),
        WireConnection::LShift(a, b) => solve(map, a) << b,
        WireConnection::RShift(a, b) => solve(map, a) >> b,
        WireConnection::Not(a) => !solve(map, a),
        WireConnection::Direct(a) => solve(map, a),
        WireConnection::Computed(v) => v,
    };
    map.insert(wire, WireConnection::Computed(v));
    return v;
}

#[derive(Debug)]
enum WireConnection<'a> {
    Computed(u16),
    Direct(&'a str),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    LShift(&'a str, i32),
    RShift(&'a str, i32),
    Not(&'a str),
}

type WireMap<'a> = HashMap<&'a str, WireConnection<'a>>;

fn parse<'a>(input: &'a str) -> WireMap<'a> {
    input
        .lines()
        .map(|l| {
            let (data, key) = l.split_once(" -> ").unwrap();
            let d = data.split_whitespace().collect_vec();
            let v = if d.len() == 1 {
                WireConnection::Direct(d[0])
            } else if d[0] == "NOT" {
                WireConnection::Not(d[1])
            } else if d[1] == "AND" {
                WireConnection::And(d[0], d[2])
            } else if d[1] == "OR" {
                WireConnection::Or(d[0], d[2])
            } else if d[1] == "LSHIFT" {
                WireConnection::LShift(d[0], d[2].parse().unwrap())
            } else if d[1] == "RSHIFT" {
                WireConnection::RShift(d[0], d[2].parse().unwrap())
            } else {
                panic!("Unknown {data}");
            };

            return (key, v);
        })
        .collect()
}
