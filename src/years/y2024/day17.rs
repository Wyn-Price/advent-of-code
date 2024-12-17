use itertools::Itertools;
use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let comp = parse(input);
    println!("{comp:?}");
    let output = run(comp);

    println!("{}", output.into_iter().join(","));
    0
}

pub fn part_b(input: &str) -> i64 {
    let master_comp = parse(input);
    println!("{master_comp:?}");

    // Not really a general solution sorry
    // It tries to work out patterns in values of a that match the first N digits,
    // then extrapolate on that pattern

    // I just worked these out, but I've tried to make a more general solution
    // let mut i = 427492251;
    // let diff_patterns: Vec<i64> = vec![
    //     4194304, 1056768, 2097152, 1040384, 1056768, 2097152, 1040384, 4194304, 4194304, 4194304,
    //     4194304, 4194304, 4194304, 4194304, 4194304, 4194304, 4194304, 4194304, 4194304, 4194304,
    // ];

    let r1 = compute_with_match_pattern(&master_comp, 0, 200_000_000, vec![1], 7);
    if let Ok(value) = r1 {
        return value;
    }
    let m1 = r1.unwrap_err();
    let (p1s, p1p) = get_diff_pattern_for_matches(m1, false);
    let r2 = compute_with_match_pattern(&master_comp, p1s, 1000_000_000_000_000, p1p, 12);
    if let Ok(value) = r2 {
        return value;
    }

    panic!("No solution? Could keep repeating")
}

fn run(mut comp: Computer) -> Vec<i64> {
    let mut ins_point = 0;

    let mut output = vec![];

    while ins_point < comp.ins.len() {
        let opcode = comp.ins[ins_point];
        let operand = comp.ins[ins_point + 1];
        ins_point += 2;

        match opcode {
            // Division (bitshift)
            0 => {
                comp.a = comp.a >> combo(&comp, operand);
            }
            // Bitsift
            1 => {
                comp.b = comp.b ^ (operand as i64);
            }
            // Modulo (Mask 0b111)
            2 => {
                comp.b = combo(&comp, operand) & 0b111;
            }
            3 => {
                if comp.a != 0 {
                    ins_point = operand as usize;
                }
            }
            4 => {
                comp.b = comp.b ^ comp.c;
            }
            5 => {
                output.push(combo(&comp, operand) & 7);
            }
            6 => {
                comp.b = comp.a >> combo(&comp, operand);
            }
            7 => {
                comp.c = comp.a >> combo(&comp, operand);
            }
            _ => panic!("Unknown opcode"),
        }
    }
    output
}

fn get_diff_pattern_for_matches(mut matches: Vec<i64>, known_start: bool) -> (i64, Vec<i64>) {
    let mut diffs = matches
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    if !known_start {
        diffs.remove(0);
        matches.remove(0);
    }
    // println!("{:?}", diffs);

    for len in 2..diffs.len() / 2 {
        let pattern = &diffs[0..len];
        println!("Testing len={}: {:?}", len, pattern);
        let mut i = 0;
        let mut does_match = true;
        while i < diffs.len() {
            for pi in 0..len {
                if i + pi < diffs.len() && pattern[pi] != diffs[i + pi] {
                    does_match = false;
                }
            }
            i += len;
        }
        if does_match {
            return (matches[0], pattern.to_vec());
        }
    }

    panic!("No pattern? Adjust end maybe?");
}

fn compute_with_match_pattern(
    master_comp: &Computer,
    start: i64,
    end: i64,
    diff_pattern: Vec<i64>,
    first_num_digits: usize,
) -> Result<i64, Vec<i64>> {
    let mut prev_match = 0;
    let mut i = start;
    let mut ai = 0;
    let mut matched_on = vec![];
    while i <= end {
        i += diff_pattern[ai % diff_pattern.len()];
        ai += 1;

        let mut comp = master_comp.clone();
        comp.a = i;
        let output = run(comp);

        let diff = i - prev_match;
        let mut match_on_first: bool = diff > 10_000 && output.len() >= first_num_digits;
        if match_on_first {
            for i in 0..first_num_digits {
                if output[i] != master_comp.ins[i] as i64 {
                    match_on_first = false;
                }
            }
        }

        if match_on_first {
            // let differs_from_prev = output.len() != prev_result.len()
            //     || output.iter().zip(&prev_result).any(|(a, b)| *a != *b);
            // if differs_from_prev {
            println!("{i}, {} {} {output:?}", output.len(), diff);
            prev_match = i;
            // prev_result = output.clone();
            matched_on.push(i);
            // }
        }

        if master_comp.ins.len() != output.len() {
            continue;
        }

        if (0..master_comp.ins.len())
            .into_iter()
            .any(|i| master_comp.ins[i] as i64 != output[i])
        {
            continue;
        }

        return Ok(i);
    }

    return Err(matched_on);
}

fn combo(comp: &Computer, operand: u8) -> i64 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => comp.a,
        5 => comp.b,
        6 => comp.c,
        _ => panic!("bad"),
    }
}

#[derive(Debug, Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ins: Vec<u8>,
}

fn parse(input: &str) -> Computer {
    let (reg_str, ins_str) = input.split_once("\n\n").unwrap();
    let re = Regex::new(r"\d+").unwrap();

    let regs = reg_str
        .lines()
        .map(|s| {
            re.captures(s)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .parse::<i64>()
                .unwrap()
        })
        .collect_vec();

    let ins = re
        .captures_iter(ins_str)
        .map(|c| c.get(0).unwrap().as_str().parse().unwrap())
        .collect_vec();

    return Computer {
        ins,
        a: regs[0],
        b: regs[1],
        c: regs[2],
    };
}
