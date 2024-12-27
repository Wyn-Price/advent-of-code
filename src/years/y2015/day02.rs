use itertools::Itertools;

pub fn part_a(input: &str) -> i32 {
    let nums = parse(input);
    nums.into_iter().map(|(l, w, h)| {
        let (a, b, c) = (l*w, w*h, h*l);
        return 2*a + 2*b + 2*c + a.min(b).min(c);
    }).sum()
}

pub fn part_b(input: &str) -> i32 {
    let nums = parse(input);
    nums.into_iter().map(|(l, w, h)| {
        let mut ws = vec![l, w, h];
        ws.sort();
        let ma = ws[0];
        let mb = ws[1];
        return 2*ma + 2*mb + l*w*h;
    }).sum()
}

fn parse(input: &str) -> Vec<(i32, i32, i32)> {
    input.lines().map(|l| l.split("x").map(|n| n.parse().unwrap()).collect_tuple().unwrap()).collect_vec()
}
