pub fn part_a(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(first, last)| score_char(first.chars().find(|&c| last.contains(c)).unwrap()))
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    input
        .lines()
        .array_chunks::<3>()
        .map(|[a, b, c]| {
            a.chars()
                .find(|&char| b.contains(char) && c.contains(char))
                .unwrap()
        })
        .map(score_char)
        .sum()
}

fn score_char(c: char) -> i64 {
    let cc = c as u8;
    let priority = match cc {
        cc if cc >= b'a' && cc <= b'z' => cc - b'a' + 1,
        cc if cc >= b'A' && cc <= b'Z' => cc - b'A' + 27,
        _ => panic!("Unknown char {cc}"),
    };
    priority as i64
}
