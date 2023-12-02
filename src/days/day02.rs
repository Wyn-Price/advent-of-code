use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .filter(|&(game, r, g, b)| r <= 12 && g <= 13 && b <= 14)
        .fold(0, |acc, (game, r, g, b)| acc + game) as i64
}

pub fn part_b(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .fold(0, |acc, (game, r, g, b)| acc + (r * g * b)) as i64
}

fn parse(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let line_regex = Regex::new(r"Game (\d+): (.+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = line_regex.captures(line).unwrap();
            let game_id = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let data = captures.get(2).unwrap().as_str();

            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;

            data.split(";").for_each(|part| {
                let mut lr = 0;
                let mut lb = 0;
                let mut lg = 0;

                part.split(",").map(|s| s.trim()).for_each(|s| {
                    let c = s.split(" ").collect::<Vec<_>>();
                    let amount = c[0].parse::<i32>().unwrap();
                    match c[1] {
                        "red" => lr += amount,
                        "green" => lg += amount,
                        "blue" => lb += amount,
                        _ => panic!("Unknown colour '{}'", c[1]),
                    };
                });

                red = red.max(lr);
                green = green.max(lg);
                blue = blue.max(lb);
            });
            return (game_id, red, green, blue);
        })
        .collect()
}
