use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let (mut stacks, commands) = parse(input);

    for [amount, src, dest] in commands {
        for _ in 0..amount {
            let e = stacks[src - 1].pop().unwrap();
            stacks[dest - 1].push(e);
        }
    }

    output(&mut stacks);
    0
}

pub fn part_b(input: &str) -> i64 {
    let (mut stacks, commands) = parse(input);

    for [amount, src, dest] in commands {
        let idx = stacks[dest - 1].len();
        for _ in 0..amount {
            let e = stacks[src - 1].pop().unwrap();
            stacks[dest - 1].insert(idx, e);
        }
    }

    output(&mut stacks);
    0
}

fn parse(input: &str) -> ([Vec<char>; 9], Vec<[usize; 3]>) {
    let (stacks, commands) = input.split_once("\n\n").unwrap();
    return (parse_stacks(stacks), parse_commands(commands));
}

fn parse_stacks(input: &str) -> [Vec<char>; 9] {
    let re = Regex::new(r"\[(.)\]|\s{3}\s").unwrap();

    let mut results: [Vec<char>; 9] = Default::default();

    input
        .lines()
        .flat_map(|line| {
            re.captures_iter(line)
                .map(|c| c.get(1))
                .enumerate()
                .filter(|(_, r)| r.is_some())
        })
        .for_each(|(i, res)| results[i].insert(0, res.unwrap().as_str().chars().nth(0).unwrap()));

    results
}

fn parse_commands(input: &str) -> Vec<[usize; 3]> {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input
        .lines()
        .map(|l| {
            let cap = re.captures(l).unwrap();
            return [
                cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                cap.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            ];
        })
        .collect()
}

fn output(stacks: &mut [Vec<char>; 9]) {
    let str: String = stacks.into_iter().map(|s| s.pop().unwrap_or(' ')).collect();
    println!("{str}");
}
