pub fn part_a(input: &str) -> i64 {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{input}{i}"));
        if digest[0] == 0 && digest[1] == 0 && digest[2] & 0xF0 == 0 {
            return i;
        }
        i += 1;
    }
}

pub fn part_b(input: &str) -> i64 {
    let mut i = 0;
    loop {
        let digest = md5::compute(format!("{input}{i}"));
        if digest[0] == 0 && digest[1] == 0 && digest[2] == 0 {
            return i;
        }
        i += 1;
    }
}
