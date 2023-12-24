use std::{
    io::Write,
    process::{Command, Stdio},
};

use itertools::Itertools;

type Board = Vec<([i64; 3], [i64; 3])>;

pub fn part_a(input: &str) -> i64 {
    let entries = parse(input);

    let imin = 200000000000000.0;
    let imax = 400000000000000.0;

    entries
        .iter()
        .map(|(a, b)| (a[0] as f64, a[1] as f64, b[0] as f64, b[1] as f64))
        .combinations(2)
        .map(|vec| {
            let (axp, ayp, axv, ayv) = vec[0];
            let (bxp, byp, bxv, byv) = vec[1];
            // x' = axp + t*axv
            // bxp + t*bxv = axp + c*axv
            // c*axv - t*bxv = bxp - axp
            // c = (bxp - axp + t*bxv) / axv

            // y' = ayp + t*avp
            // t*byv - c*ayv  = ayp - byp

            // t = (ayp - byp + c*ayv) / (byv)
            // t * byv = ayp - byp + (bxp - axp + t*bxv) * ayv / avx
            // t * byv = ayp - byp + (bxp - axp) * ayv / avx + t*bxv*ayv/avx
            // t * byv - t*bxv*ayv/avx = ayp - byp + (bxp - axp) * ayv / avx
            // t = (ayp - byp + (bxp - axp) * ayv / avx) / (byv - bxv*ayv/avx)

            if axv * byv - ayv * bxv == 0.0 {
                return 0;
            }

            // let num = ayp - byp + (bxp - axp) * (ayv / axv);
            // let den = byv - bxv * ayv / axv;

            let t1 = ((bxp - axp) * byv - (byp - ayp) * bxv) / (axv * byv - ayv * bxv);
            let t2 = ((bxp - axp) * ayv - (byp - ayp) * axv) / (axv * byv - ayv * bxv);

            if t1 < 0.0 || t2 < 0.0 {
                return 0;
            }

            let x = axp + t1 * axv;
            let y = ayp + t1 * ayv;

            if x >= imin && x <= imax && y >= imin && y <= imax {
                return 1;
            }

            return 0;
        })
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    // TODO: I could never get z3 working in rust. There is a crate for it
    // https://crates.io/crates/z3, but it's just bindings around the C++ z3 solver
    // Most of my issues with NixOS related, as the z3-sys crate isn't particularly flexible
    let cmd = Command::new("python")
        .arg("day_24b.py")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    cmd.stdin.as_ref().unwrap().write(input.as_bytes()).unwrap();
    let out = cmd.wait_with_output().unwrap().stdout;
    String::from_utf8(out)
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap()
}

fn parse(input: &str) -> Board {
    input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let ps = p
                .split(", ")
                .map(|p| p.trim().parse::<i64>().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();
            let vs = v
                .split(", ")
                .map(|p| p.trim().parse::<i64>().unwrap())
                .collect_vec()
                .try_into()
                .unwrap();
            return (ps, vs);
        })
        .collect()
}
