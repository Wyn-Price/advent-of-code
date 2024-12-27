use itertools::Itertools;

pub fn part_a(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            let mut str = String::new();
            let mut chars = s.chars().collect_vec();
            // Remove first and last "
            chars.remove(0);
            chars.pop();
            let mut i = 0;
            while i < chars.len() {
                let next = match chars[i] {
                    '\\' => {
                        i += 1;
                        match chars[i] {
                            '\\' => '\\',
                            '"' => '"',
                            'x' => {
                                let c = u8::from_str_radix(
                                    format!("{}{}", chars[i + 1], chars[i + 2]).as_str(),
                                    16,
                                )
                                .unwrap() as char;
                                i += 2;
                                c
                            }
                            _ => panic!("Don't know to to escape {}", chars[i]),
                        }
                    }
                    _ => chars[i],
                };
                str += next.to_string().as_str();
                i += 1;
            }

            return (s.to_owned(), str);
        })
        .map(|(o, s)| o.chars().count() as i32 - s.chars().count() as i32)
        .sum()
}

pub fn part_b(input: &str) -> i32 {
    input
        .lines()
        .map(|s| {
            let mut str = String::new();
            let chars = s.chars().collect_vec();
            let mut i = 0;
            while i < chars.len() {
                let c = chars[i];
                let next = match c {
                    '\"' => "\\\"".to_owned(),
                    '\\' => "\\\\".to_owned(),
                    _ => c.to_string(),
                };
                str += next.as_str();
                i += 1;
            }

            str = format!("\"{str}\"");

            return (s.to_owned(), str);
        })
        .map(|(o, s)| s.chars().count() as i32 - o.chars().count() as i32)
        .sum()
}
