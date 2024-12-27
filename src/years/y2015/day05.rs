use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    input.lines().filter(|&s| {
        let num_vowels = s.chars().filter(|&c| c == 'a' || c == 'e' || c =='i' || c =='o' || c =='u').count() >= 3;
        let reps = s.chars().tuple_windows().any(|(a, b)| a == b);
        let invalid = s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy");

        return num_vowels && reps && !invalid;
    }).count() as i64
}

pub fn part_b(input: &str) -> i64 {
    input.lines().filter(|&s| {
        let pa = s.chars().enumerate().tuple_windows().any(|((i, a), (_, b))| {
            s.chars().tuple_windows().skip(i + 2).any(|(a2, b2)| a == a2 && b == b2)
        });
        let pb = s.chars().tuple_windows().any(|(a, _ , c)| a == c);
        return  pa && pb;
    }).count() as i64
}

