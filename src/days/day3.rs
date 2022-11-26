const BITS: usize = 12;
const MASK: i64 = 4095;

pub fn part_a(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let counts = count_bits(&lines, &(0..BITS).collect());
    let num: i64 = counts
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &count)| acc + (count << i));

    let inv = !num & MASK;

    inv * num
}

pub fn part_b(input: &str) -> i64 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut most = lines.clone();
    let mut least = lines.clone();

    (0..BITS).for_each(|i| {
        filter_out_lines(&mut most, i, 1);
        filter_out_lines(&mut least, i, 0);
    });

    let most_num = i64::from_str_radix(most[0], 2).unwrap();
    let least_num = i64::from_str_radix(least[0], 2).unwrap();

    return most_num * least_num;
}

fn filter_out_lines(lines: &mut Vec<&str>, index: usize, sig: i64) {
    let most_common = count_bits(&lines, &vec![index])[index];
    let filter_char = if most_common == sig { '1' } else { '0' };

    if lines.len() != 1 {
        lines.retain(|&n| n.chars().nth(index).unwrap() == filter_char);
    }
}

fn count_bits(lines: &Vec<&str>, bits_to_check: &Vec<usize>) -> Vec<i64> {
    let mut counts = vec![0; BITS.try_into().unwrap()];

    lines.into_iter().for_each(|l| {
        let chars = l.chars().collect::<Vec<_>>();

        for &i in bits_to_check {
            let c = chars[i];
            counts[i] += match c {
                '0' => -1,
                '1' => 1,
                _ => panic!("Unknown char {c}"),
            }
        }
    });

    counts.iter().map(|&c| if c >= 0 { 1 } else { 0 }).collect()
}
