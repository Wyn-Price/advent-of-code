pub fn part_a(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|&[theirs, ours]| -> i64 {
            let win = match theirs {
                theirs if theirs == ours => 1,
                theirs if (ours + 1) % 3 == theirs => 0,
                _ => 2,
            };
            ((win * 3) + ours + 1).into()
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    parse(input)
        .iter()
        .map(|&[theirs, win]| -> i64 {
            let ours = match win {
                0 => (theirs + 2) % 3,
                1 => theirs,
                _ => (theirs + 1) % 3,
            };
            ((win * 3) + ours + 1).into()
        })
        .sum()
}

fn parse(input: &str) -> Vec<[u8; 2]> {
    input
        .lines()
        .map(|l| {
            let chars = l.chars().collect::<Vec<_>>();
            [chars[0] as u8 - b'A', chars[2] as u8 - b'X']
        })
        .collect()
}
