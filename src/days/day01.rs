pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|str| {
            let v = str.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>();
            return format!("{}{}", v.first().unwrap(), v.last().unwrap());
        })
        .map(|str| str.parse::<i64>().unwrap())
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .map(|str| {
            let replaced = str
                .replace("zero", "zero0zero")
                .replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "four4four")
                .replace("five", "five5five")
                .replace("six", "six6six")
                .replace("seven", "seven7seven")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine");
            let v = replaced
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>();
            return format!("{}{}", v.first().unwrap(), v.last().unwrap());
        })
        .map(|str| str.parse::<i64>().unwrap())
        .sum()
}
