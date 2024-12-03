use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            (
                c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            )
        })
        .map(|(a, b)| (a * b) as i64)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    re.captures_iter(input)
        .filter_map(|c| {
            let str = c.get(0).unwrap().as_str();
            match str {
                "do()" => {
                    enabled = true;
                    None
                }
                "don't()" => {
                    enabled = false;
                    None
                }
                _ => {
                    if enabled {
                        Some((
                            c.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                            c.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                        ))
                    } else {
                        None
                    }
                }
            }
        })
        .map(|(a, b)| (a * b) as i64)
        .sum()
}
