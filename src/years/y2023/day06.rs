pub fn part_a(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|(time, record)| {
            (0..=time)
                .map(|down| {
                    let speed = down;
                    let time_left = time - down;
                    return speed * time_left;
                })
                .filter(|&t| t > record)
                .count() as i64
        })
        .product()
}

pub fn part_b(input: &str) -> i64 {
    part_a(&input.replace(" ", ""))
}

fn parse(input: &str) -> Vec<(i64, i64)> {
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|li| li.split_once(":").unwrap().1)
        .map(|p| {
            p.trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    let time = lines[0].clone();
    let distance = lines[1].clone();

    time.into_iter()
        .enumerate()
        .map(|(idx, time)| (time, distance[idx]))
        .collect()
}
