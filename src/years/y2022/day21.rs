use std::collections::HashMap;

pub fn part_a(input: &str) -> i64 {
    let map = parse(input);
    get_value(&"root".to_owned(), &map, false).unwrap()
}

pub fn part_b(input: &str) -> i64 {
    let map = parse(input);
    get_humn_value(0, &"root".to_owned(), &map, true)
}

fn get_humn_value(
    target: i64,
    value: &String,
    map: &HashMap<String, Value>,
    is_equals: bool,
) -> i64 {
    if value == "humn" {
        return target;
    }
    let (a, op, b) = match map.get(value).unwrap() {
        Value::Number(v) => panic!("Expected op {v}"),
        Value::Operation(a, op, b) => (a, op, b),
    };
    let a_value = get_value(a, &map, true);
    let b_value = get_value(b, &map, true);

    let unknown_key = if a_value.is_some() { b } else { a };

    let known = a_value.or(b_value).unwrap();

    let new_target = if is_equals {
        known
    } else {
        match op {
            // a + b = target
            // a = target - b
            Operation::Add => target - known,
            // a * b = target
            // a = target / b
            Operation::Multiply => target / known,
            Operation::Subtract => {
                //a - b = target
                if a_value.is_some() {
                    // a - target = b
                    known - target
                } else {
                    // a = target + b
                    target + known
                }
            }
            Operation::Divide => {
                // a / b = target
                if a_value.is_some() {
                    // a / target = b
                    known / target
                } else {
                    // a = target * b
                    target * known
                }
            }
        }
    };

    get_humn_value(new_target, unknown_key, map, false)
}

fn get_value(value: &String, map: &HashMap<String, Value>, break_on_humn: bool) -> Option<i64> {
    if value == "humn" && break_on_humn {
        return None;
    }

    match map.get(value).unwrap() {
        Value::Number(v) => Some(*v),
        Value::Operation(a, op, b) => {
            let av = get_value(a, map, break_on_humn)?;
            let bv = get_value(b, map, break_on_humn)?;

            let v = match op {
                Operation::Add => av + bv,
                Operation::Subtract => av - bv,
                Operation::Divide => av / bv,
                Operation::Multiply => av * bv,
            };
            Some(v)
        }
    }
}

fn parse(input: &str) -> HashMap<String, Value> {
    input
        .lines()
        .map(|l| {
            let (identifier, value_str) = l.split_once(": ").unwrap();
            let value = match value_str.parse::<i64>() {
                Ok(v) => Value::Number(v),
                Err(_) => {
                    let mut split = value_str.split_whitespace();
                    let a = split.next().unwrap();
                    let o = split.next().unwrap();
                    let b = split.next().unwrap();

                    let op = match o {
                        "+" => Operation::Add,
                        "-" => Operation::Subtract,
                        "*" => Operation::Multiply,
                        "/" => Operation::Divide,
                        _ => panic!("Unknown op {o}"),
                    };

                    Value::Operation(a.to_owned(), op, b.to_owned())
                }
            };

            (identifier.to_owned(), value)
        })
        .collect()
}

#[derive(Debug)]
enum Value {
    Number(i64),
    Operation(String, Operation, String),
}

#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}
