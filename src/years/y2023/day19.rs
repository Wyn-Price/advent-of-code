use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

// Bool -> is_less
type Workflow<'a> = (Vec<(usize, bool, i32, &'a str)>, &'a str);
type Workflows<'a> = HashMap<&'a str, Workflow<'a>>;
type Ratings = Vec<[i32; 4]>;
type Range = (i32, i32);

pub fn part_a(input: &str) -> i64 {
    let (workflow, ratings) = parse(input);
    ratings
        .into_iter()
        .filter(|r| is_accepted(&workflow, r))
        .map(|[x, m, a, s]| (x + m + a + s) as i64)
        .sum()
}

pub fn part_b(input: &str) -> i64 {
    let (workflow, _) = parse(input);
    // (0..=4000)
    //     .cartesian_product(0..=4000)
    //     .cartesian_product(0..=4000)
    //     .cartesian_product(0..=4000)
    //     .filter(|&(((x, m), a), s)| is_accepted(&workflow, &[x, m, a, s]))
    //     .count() as i64

    let mut accepted: Vec<[Range; 4]> = vec![];

    accepted_range(&workflow, "in", &[(1, 4000); 4], &mut accepted);

    accepted
        .into_iter()
        .map(|r| r.iter().map(|(s, e)| (e - s + 1) as i64).product::<i64>())
        .sum::<i64>()
}

fn accepted_range(
    workflows: &Workflows,
    key: &str,
    ratings: &[Range; 4],
    accepted: &mut Vec<[Range; 4]>,
) {
    if key == "R" {
        return;
    }
    if key == "A" {
        accepted.push(ratings.clone());
        return;
    }

    let (flows, fallback) = workflows.get(key).unwrap();

    let mut new_ratings = ratings.clone();
    for &(index, less, test, send) in flows {
        let &range = &new_ratings[index];
        let (range_start, range_end) = range;

        let (new_range, else_range) = if less {
            if test <= range_start {
                (None, Some(range))
            } else if range_end < test {
                (Some(range), None)
            } else {
                (Some((range_start, test - 1)), Some((test, range_end)))
            }
        } else {
            if range_end <= test {
                (None, Some(range))
            } else if test < range_start {
                (Some(range), None)
            } else {
                (Some((test + 1, range_end)), Some((range_start, test)))
            }
        };

        if let Some(new) = new_range {
            new_ratings[index] = new;
            accepted_range(workflows, send, &new_ratings, accepted);
        }

        if let Some(els) = else_range {
            new_ratings[index] = els;
        } else {
            return;
        }
    }

    accepted_range(workflows, fallback, &new_ratings, accepted);
}

fn is_accepted(workflows: &Workflows, rating: &[i32; 4]) -> bool {
    let mut workflow = "in";
    while workflow != "R" && workflow != "A" {
        let (flows, fallback) = workflows.get(workflow).unwrap();
        workflow = fallback;

        for &(index, less, test, send) in flows {
            let value = rating[index];
            let pred = if less { value < test } else { value > test };
            if pred {
                workflow = send;
                break;
            }
        }
    }

    return workflow == "A";
}

fn parse(input: &str) -> (Workflows, Ratings) {
    let (workflows, ratings) = input.split_once("\n\n").unwrap();

    let wr = Regex::new(r"(.+)\{(.+)\}").unwrap();
    let iwr = Regex::new(r"(.)(>|<)(\d+):(.+)").unwrap();

    let workflow_map: Workflows = workflows
        .lines()
        .map(|line| {
            let caps = wr.captures(line).unwrap();
            let key = caps.get(1).unwrap().as_str();
            let body = caps.get(2).unwrap().as_str();

            let mut elements = body.split(",").collect_vec();
            let last = elements.pop().unwrap();

            let vec = elements
                .into_iter()
                .map(|str| {
                    let captures = iwr.captures(str).unwrap();
                    let p = match captures.get(1).unwrap().as_str() {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => panic!(),
                    } as usize;

                    let eq = captures.get(2).unwrap().as_str() == "<";

                    let num = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();

                    let to = captures.get(4).unwrap().as_str();

                    return (p, eq, num, to);
                })
                .collect_vec();

            return (key, (vec, last));
        })
        .collect();

    let rr = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

    let ratings_list: Ratings = ratings
        .lines()
        .map(|line| {
            let caps = rr.captures(line).unwrap();

            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let m = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let a = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let s = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();

            return [x, m, a, s];
        })
        .collect();

    return (workflow_map, ratings_list);
}
