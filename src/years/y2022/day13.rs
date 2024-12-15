use std::{borrow::BorrowMut, cmp::Ordering, iter::Peekable};

pub fn part_a(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .enumerate()
        .filter(|(_, (a_list, b_list))| cmp_list(a_list, b_list).unwrap_or(true))
        .map(|(idx, _)| idx as i64 + 1)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let mut vec = parse(input)
        .into_iter()
        .flat_map(|(a, b)| vec![a, b])
        .collect::<Vec<_>>();

    let a = vec![Packet::Packets(vec![Packet::Number(2)])];
    let b = vec![Packet::Packets(vec![Packet::Number(6)])];

    vec.push(a);
    vec.push(b);

    let mut idx_vec = vec.into_iter().enumerate().collect::<Vec<_>>();

    let a_idx = idx_vec.len() - 2;
    let b_idx = idx_vec.len() - 1;

    idx_vec.sort_by(|(_, a), (_, b)| {
        if cmp_list(&a, &b).unwrap_or(true) {
            return Ordering::Less;
        } else {
            return Ordering::Greater;
        }
    });

    let aid = idx_vec.iter().position(|&(id, _)| id == a_idx).unwrap() as i64 + 1;
    let bid = idx_vec.iter().position(|&(id, _)| id == b_idx).unwrap() as i64 + 1;
    aid * bid
}

fn cmp_list(a_list: &Vec<Packet>, b_list: &Vec<Packet>) -> Option<bool> {
    for i in 0..a_list.len() {
        if i >= b_list.len() {
            return Some(false);
        }
        if let Some(a) = cmp(&a_list[i], &b_list[i]) {
            return Some(a);
        }
    }
    if a_list.len() == b_list.len() {
        return None;
    } else {
        return Some(a_list.len() < b_list.len());
    }
}

fn cmp(a: &Packet, b: &Packet) -> Option<bool> {
    match a {
        Packet::Packets(a_list) => match b {
            Packet::Packets(b_list) => cmp_list(a_list, b_list),
            Packet::Number(b_num) => cmp(a, &Packet::Packets(vec![Packet::Number(*b_num)])),
        },
        Packet::Number(a_num) => match b {
            Packet::Number(b_num) => {
                if *a_num == *b_num {
                    return None;
                }
                return Some(*a_num < *b_num);
            }
            Packet::Packets(_) => cmp(&Packet::Packets(vec![Packet::Number(*a_num)]), b),
        },
    }
}

fn parse(input: &str) -> Vec<(PacketList, PacketList)> {
    input
        .split("\n\n")
        .map(|block| {
            let (first, second) = block.split_once("\n").unwrap();
            (parse_top(first), parse_top(second))
        })
        .collect()
}

fn parse_top(input: &str) -> PacketList {
    let res = parse_chunk(input.chars().peekable().borrow_mut()).unwrap();
    match res {
        Packet::Packets(packs) => packs,
        _ => panic!("Expected packets returned"),
    }
}

fn parse_chunk<I: Iterator<Item = char>>(input: &mut Peekable<I>) -> Option<Packet> {
    let mut c = input.next()?;
    if c == ',' {
        c = input.next()?;
    }
    if c == ']' {
        return None;
    }
    if c == '[' {
        let mut list = vec![];
        while let Some(res) = parse_chunk(input) {
            list.push(res);
        }
        return Some(Packet::Packets(list));
    }

    if c >= '0' && c <= '9' {
        let mut number = c.to_string();
        while let Some(peek) = input.peek() {
            if *peek >= '0' && *peek <= '9' {
                number += &input.next().unwrap().to_string().to_owned();
            } else {
                break;
            }
        }
        return Some(Packet::Number(number.parse().unwrap()));
    }

    panic!("Don't know how to handle {c}");
}

type PacketList = Vec<Packet>;

#[derive(Debug)]
enum Packet {
    Packets(PacketList),
    Number(i64),
}

impl ToString for Packet {
    fn to_string(&self) -> String {
        match self {
            Packet::Packets(vec) => {
                "[".to_owned()
                    + &vec
                        .iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<_>>()
                        .join(",")
                        .to_owned()
                    + &"]".to_owned()
            }
            Packet::Number(num) => num.to_string(),
        }
    }
}
