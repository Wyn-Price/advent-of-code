use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};
use itertools::Itertools;
use petgraph::{algo::dijkstra, Graph};
use regex::Regex;

pub fn part_a(input: &str) -> i64 {
    let machines = parse(input);

    let mut sum = 0;

    for machine in machines {
        let mut g = Graph::new_undirected();
        let nodes = (0..=machine.len).map(|_| g.add_node(1)).collect_vec();

        for button in machine.buttons {
            for i in 0..=machine.len {
                g.add_edge(nodes[i], nodes[i ^ button], 1);
            }
        }

        let map = dijkstra(&g, nodes[0], Some(nodes[machine.end]), |_| 1);
        sum += map[&nodes[machine.end]];
    }

    return sum;
}

pub fn part_b(input: &str) -> i64 {
    let machines = parse(input);

    let mut sum = 0;

    // whatever, integer linear solver
    for (mi, machine) in machines.iter().enumerate() {
        println!("{mi}");

        let mut vars = variables!();
        let variables = machine
            .buttons_nums
            .iter()
            .map(|_| vars.add(variable().integer().min(0)))
            .collect_vec();

        let mut problem = vars
            .minimise(variables.iter().sum::<Expression>())
            .using(default_solver);

        for i in 0..machine.joltage.len() {
            let mut expr = Expression::from(0);
            for (btn_idx, btn) in machine.buttons_nums.iter().enumerate() {
                if btn.contains(&i) {
                    expr = expr + variables[btn_idx];
                }
            }
            problem = problem.with(expr.eq(machine.joltage[i] as i32));
        }

        let solution = problem.solve().unwrap();
        let presses: i64 = variables.iter().map(|v| solution.value(*v) as i64).sum();
        sum += presses;
    }

    return sum;
}
#[derive(Debug)]
struct Machine {
    len: usize,
    end: usize,
    buttons: Vec<usize>,
    buttons_nums: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse(input: &str) -> Vec<Machine> {
    let brackets = Regex::new(r"[\[\]{}()]").unwrap();
    input
        .lines()
        .map(|l| {
            let binding = brackets.replace_all(l, "");
            let mut chunks = binding.split_whitespace().collect_vec();
            let end = chunks.remove(0);
            let joltage = chunks.pop().unwrap();

            let end_len = end.len();
            let end_usize = end.char_indices().fold(0_usize, |acc, (i, char)| {
                if char == '.' {
                    return acc;
                }
                return acc + (1 << i);
            });

            let buttons = chunks
                .iter()
                .map(|&b| {
                    b.split(",").fold(0_usize, |acc, n| {
                        let num = n.parse::<usize>().unwrap();
                        return acc + (1 << num);
                    })
                })
                .collect_vec();

            let buttons_vec = chunks
                .iter()
                .map(|&b| {
                    b.split(",")
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let joltage = joltage.split(",").map(|j| j.parse().unwrap()).collect_vec();

            return Machine {
                len: 2_usize.pow(end_len as u32) - 1,
                end: end_usize,
                buttons,
                buttons_nums: buttons_vec,
                joltage,
            };
        })
        .collect_vec()
}
