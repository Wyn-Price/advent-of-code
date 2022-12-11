pub fn part_a(input: &str) -> i64 {
    let mut monkeys = parse(input, true);
    compute_rounds(20, monkeys)
}

pub fn part_b(input: &str) -> i64 {
    let mut monkeys = parse(input, false);
    let mod_by = monkeys
        .iter()
        .map(|m| m.div_test)
        .reduce(|a, b| a * b)
        .unwrap();
    monkeys.iter_mut().for_each(|m| m.normalize = mod_by);
    compute_rounds(10000, monkeys)
}

fn compute_rounds(rounds: usize, mut monkeys: Vec<Monkey>) -> i64 {
    let len = monkeys.len();

    for r in 0..rounds {
        for mid in 0..monkeys.len() {
            let mut next_rounds: Vec<Vec<i64>> = vec![vec![]; len];

            monkeys[mid].compute_round(&mut next_rounds);
            monkeys[mid].items.clear();

            next_rounds.iter_mut().enumerate().for_each(|(i, om)| {
                monkeys[i].items.append(om);
            })
        }
    }

    let mut vec = monkeys
        .into_iter()
        .map(|m| m.inspected_items as i64)
        .collect::<Vec<_>>();

    vec.sort();
    vec.reverse();

    vec.into_iter().take(2).reduce(|a, b| a * b).unwrap()
}

fn parse(input: &str, part_a: bool) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|m| {
            let (id, starting, op_str, op_value, div_test, when_true, when_false) = scan_fmt!(
                m,
                "Monkey {d}:
  Starting items: {/.+/}
  Operation: new = old  {[+*]} {/old|(\\d+)/}
  Test: divisible by {d}
     If true: throw to monkey {d}
     If false: throw to monkey {d}",
                u8,
                String,
                String,
                String,
                i64,
                u8,
                u8
            )
            .unwrap();

            Monkey {
                idx: id,
                normalize: 3,
                part_a,
                inspected_items: 0,
                items: starting.split(", ").map(|l| l.parse().unwrap()).collect(),
                operation: (
                    if op_str == "+" {
                        Operation::Add
                    } else {
                        Operation::Multiply
                    },
                    if op_value == "old" {
                        OpValue::Old
                    } else {
                        OpValue::Num(op_value.parse().unwrap())
                    },
                ),
                div_test,
                when_true,
                when_false,
            }
        })
        .collect()
}

#[derive(Debug)]
struct Monkey {
    idx: u8,
    normalize: i64,
    part_a: bool,
    inspected_items: usize,
    items: Vec<i64>,
    operation: (Operation, OpValue),
    div_test: i64,
    when_true: u8,
    when_false: u8,
}

#[derive(Debug)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
enum OpValue {
    Old,
    Num(i64),
}

impl Monkey {
    fn compute_round(&mut self, mut next_round: &mut Vec<Vec<i64>>) {
        let (t, v) = &self.operation;
        self.items.iter().for_each(|item| {
            self.inspected_items += 1;

            let other = match v {
                OpValue::Old => item,
                OpValue::Num(n) => n,
            };

            let mut new = match t {
                Operation::Add => item + other,
                Operation::Multiply => item * other,
            };

            if self.part_a {
                new = new / self.normalize;
            } else {
                new = new % self.normalize;
            }

            let next_index = if new % self.div_test == 0 {
                self.when_true
            } else {
                self.when_false
            };

            next_round.get_mut(next_index as usize).unwrap().push(new);
        });
    }
}
