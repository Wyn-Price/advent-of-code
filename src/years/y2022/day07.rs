use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    compute_sizemap(parse(input))
        .into_values()
        .filter(|&s| s <= 100000)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let values = compute_sizemap(parse(input));
    let total = values.get("").unwrap();
    let unused = 70000000 - total;
    let needed = 30000000 - unused;
    values.into_values().filter(|&v| v >= needed).min().unwrap()
}

fn compute_sizemap(cmd: Vec<Command>) -> HashMap<String, i64> {
    let mut pwd = Vec::new();
    let mut map: HashMap<String, Vec<usize>> = HashMap::new();
    map.insert("".to_owned(), vec![]);

    cmd.iter().for_each(|cmd| match cmd {
        Command::CD(cd) => {
            if cd.starts_with('/') {
                pwd.clear();
            } else if cd == ".." {
                pwd.pop();
            } else {
                pwd.push(cd.clone());
            }
        }
        Command::LS(files) => files.into_iter().for_each(|size| {
            for i in 0..pwd.len() {
                let path = pwd[0..i + 1].join("/");
                if map.contains_key(&path) {
                    map.get_mut(&path).unwrap().push(*size);
                } else {
                    map.insert(path, vec![*size]);
                }
            }
            map.get_mut("").unwrap().push(*size);
        }),
    });

    map.into_iter()
        .map(|(k, v)| (k, v.into_iter().sum::<usize>() as i64))
        .collect::<HashMap<String, i64>>()
}

fn parse(input: &str) -> Vec<Command> {
    input
        .split("$")
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| {
            if l.starts_with("cd ") {
                Command::CD(l[3..].to_owned())
            } else {
                Command::LS(
                    l.lines()
                        .skip(1)
                        .filter(|&f| !f.starts_with("dir"))
                        .map(|f| {
                            let (size, _) = f.split_once(" ").unwrap();
                            return size.parse().unwrap();
                        })
                        .collect(),
                )
            }
        })
        .collect()
}

enum Command {
    CD(String),
    LS(Vec<usize>),
}
