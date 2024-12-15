// Add takes 2 cycles, noop takes 1

pub fn part_a(input: &str) -> i64 {
    let cmds = parse(input);

    let mut cycle = 0;
    let mut register = 1;

    let mut tth_values: Vec<i64> = vec![];

    for cmd in cmds {
        let diff = if cmd.is_some() { 2 } else { 1 };

        let tth_before = (cycle - 20_i64).div_floor(40);
        let tth_after = (cycle + diff - 20_i64).div_floor(40);

        if tth_before != tth_after {
            tth_values.push((tth_after * 40 + 20) * register);
        }

        if cmd.is_some() {
            register += cmd.unwrap();
        }
        cycle += diff;
    }

    tth_values.into_iter().sum()
}

pub fn part_b(input: &str) -> i64 {
    let cmds = parse(input);

    let mut register = 0_i64;

    let mut current_line = "".to_owned();

    for cmd in cmds {
        let diff = if cmd.is_some() { 2 } else { 1 };

        for _ in 0..diff {
            let at = current_line.len() as i64;
            let char = if at >= register && at < register + 3 {
                "#"
            } else {
                "."
            };
            current_line = current_line + char;

            if current_line.len() == 40 {
                println!("{current_line}");
                current_line = "".to_owned();
            }
        }

        if cmd.is_some() {
            register += cmd.unwrap();
        }
    }

    0
}

fn parse(input: &str) -> Vec<Option<i64>> {
    input
        .lines()
        .map(|l| {
            if l == "noop" {
                None
            } else {
                let (_, n) = l.split_once(" ").unwrap();
                Some(n.parse().unwrap())
            }
        })
        .collect()
}
