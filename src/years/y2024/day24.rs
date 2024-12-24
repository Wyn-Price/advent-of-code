use std::collections::HashMap;

use itertools::Itertools;

pub fn part_a(input: &str) -> i64 {
    let (mut def, mut op) = parse(input);
    while op.len() != 0 {
        op.retain(|&k, (l, o, r)| {
            if def.contains_key(l) && def.contains_key(r) {
                let left = def[*l];
                let right = def[*r];
                let res = match o {
                    Op::AND => left && right,
                    Op::OR => left || right,
                    Op::XOR => left != right,
                };
                def.insert(k, res);
                return false;
            }
            return true;
        });
    }

    reduce_to_num(def.iter(), "z")
}

fn reduce_to_num<'a, I>(i: I, start: &str) -> i64
where
    I: Iterator<Item = (&'a &'a str, &'a bool)>,
{
    i.filter(|&(&k, _)| k.starts_with(start))
        .sorted_by_key(|&(&k, _)| k)
        .enumerate()
        .fold(0, |acc, (i, (_, &v))| {
            if v {
                return acc | (1 << i);
            }
            return acc;
        })
}

pub fn part_b(input: &str) -> String {
    let (_, op) = parse(input);
    let rev_op = op
        .iter()
        .flat_map(|(&k, &(l, o, r))| vec![((l, o, r), k), ((r, o, l), k)])
        .collect::<HashMap<_, _>>();
    let mut swapped = vec![];
    let mut carry_reg_opt = None;
    // Adder inputs: x_i, y_i, carry_reg
    // Adder outputs: z_i, carry_reg (replaced)
    // Half adder 1:
    //  XOR: x_i y_i -> s_1
    //  AND: x_i y_y -> c_1
    // Half adder 2:
    //  XOR: s_1, carry_reg -> z_i
    //  AND: s_1, carry_reg -> c_2
    // Full Adder Carry:
    //  OR: c_1, c_2 -> carry_reg
    // https://www.researchgate.net/profile/Khairurrijal-Khairurrijal-2/publication/300918291/figure/fig4/AS:355855208861733@1461853901971/mplementation-of-full-adder-using-two-half-adder-and-OR-gate.png
    for i in 0..45 {
        let x_i = format!("x{:0>2}", i).leak();
        let y_i = format!("y{:0>2}", i).leak();

        // Get the initial regstiers for this nth input
        let mut s_1 = *rev_op.get(&(x_i, Op::XOR, y_i)).unwrap();
        let mut c_1 = *rev_op.get(&(x_i, Op::AND, y_i)).unwrap();

        // let mut c_2 = None;

        // No carry on the first
        if let Some(carry_reg) = carry_reg_opt {
            let mut z_io = rev_op.get(&(carry_reg, Op::XOR, s_1));
            // s_1 and c_1 have been swapped?
            if z_io.is_none() {
                (s_1, c_1) = (c_1, s_1);
                swapped.append(&mut vec![c_1, s_1]);
                z_io = rev_op.get(&(carry_reg, Op::XOR, s_1));
            }

            let mut z_i = *z_io.unwrap();

            let mut c_2 = *rev_op.get(&(carry_reg, Op::AND, s_1)).unwrap();

            // It should be now that s_1, c_1, c_2 are all random registers, and z_i is the zth register
            // But this won't be the case if it's swapped

            if s_1.starts_with("z") {
                (s_1, z_i) = (z_i, s_1);
                swapped.append(&mut vec![s_1, z_i]);
            }

            if c_1.starts_with("z") {
                (c_1, z_i) = (z_i, c_1);
                swapped.append(&mut vec![c_1, z_i]);
            }

            if c_2.starts_with("z") {
                (c_2, z_i) = (z_i, c_2);
                swapped.append(&mut vec![c_2, z_i]);
            }

            let mut c_n = *rev_op.get(&(c_1, Op::OR, c_2)).unwrap();

            if c_n.starts_with("z") {
                (c_n, z_i) = (z_i, c_n);
                swapped.append(&mut vec![c_n, z_i]);
            }

            carry_reg_opt = Some(c_n);
        } else {
            carry_reg_opt = Some(c_1);
        }
    }

    swapped.into_iter().sorted().join(",")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Op {
    AND,
    OR,
    XOR,
}

type Definitions<'a> = HashMap<&'a str, bool>;
type Operations<'a> = HashMap<&'a str, (&'a str, Op, &'a str)>;

fn parse(input: &str) -> (Definitions, Operations) {
    let (def_str, op_str) = input.split_once("\n\n").unwrap();
    let def = def_str
        .lines()
        .map(|l| {
            let (n, v) = l.split_once(": ").unwrap();
            return (
                n,
                match v {
                    "0" => false,
                    "1" => true,
                    _ => panic!("Unknown {v}"),
                },
            );
        })
        .collect();

    let op = op_str
        .lines()
        .map(|l| {
            let (left, oper, right, _, res) = l.split_whitespace().collect_tuple().unwrap();
            return (
                res,
                (
                    left,
                    match oper {
                        "XOR" => Op::XOR,
                        "AND" => Op::AND,
                        "OR" => Op::OR,
                        _ => panic!("Unknown op {oper}"),
                    },
                    right,
                ),
            );
        })
        .collect();

    return (def, op);
}

// Scroll down for some fun :)

// // Like whatever
// // Absoloutly fuck this shiet

// pub fn part_b_fuck_This(input: &str) -> String {
//     let (mut sdef, sop) = parse(input);
//     let xs = reduce_to_num(sdef.iter(), "x");
//     let ys = reduce_to_num(sdef.iter(), "y");
//     let zs = xs + ys;

//     let binding = sop.clone();
//     // let keys = binding.keys().combinations(4);

//     // let len = keys.try_len().unwrap();
//     let len = 1;
//     let allows = vec!["ctg", "dmh", "dvq", "rpb", "rpv", "z11", "z31", "z38"];
//     let keys = vec![sop
//         .iter()
//         .filter(|(_, (l, _, r))| allows.contains(l) && allows.contains(r))
//         .map(|(&k, _)| k)
//         .collect_vec()];

//     println!("{keys:?}");

//     let f = keys.into_iter().enumerate().find(|(i, to_perm)| {
//         if i % 10000 == 0 {
//             println!("{i} / {len}");
//         }
//         for (s1, s2, s3, s4) in vec![(0, 1, 2, 3), (0, 2, 1, 3), (0, 3, 1, 2)] {
//             let mut op = sop.clone();
//             let sw1 = op[to_perm[s1]];
//             let sw2 = op[to_perm[s2]];
//             let sw3 = op[to_perm[s3]];
//             let sw4 = op[to_perm[s4]];

//             op.insert(to_perm[s1], sw2);
//             op.insert(to_perm[s2], sw1);
//             op.insert(to_perm[s3], sw4);
//             op.insert(to_perm[s4], sw3);

//             let mut def = sdef.clone();

//             op.retain(|&k, (l, o, r)| {
//                 if def.contains_key(l) && def.contains_key(r) {
//                     let left = def[*l];
//                     let right = def[*r];
//                     let res = match o {
//                         Op::AND => left && right,
//                         Op::OR => left || right,
//                         Op::XOR => left != right,
//                     };
//                     def.insert(k, res);
//                     return false;
//                 }
//                 return true;
//             });

//             // println!(
//             //     " - Swapping {}-{} {}-{}",
//             //     to_perm[s1], to_perm[s2], to_perm[s3], to_perm[s4]
//             // );

//             // let mut to_handle = op.keys().collect_vec();
//             // let mut def = sdef.clone();
//             // while to_handle.len() != 0 {
//             //     let mut changed = false;
//             //     to_handle.retain(|&&k| {
//             //         let (l, o, r) = op[k];
//             //         if def.contains_key(l) && def.contains_key(r) {
//             //             let left = def[l];
//             //             let right = def[r];
//             //             let res = match o {
//             //                 Op::AND => left && right,
//             //                 Op::OR => left || right,
//             //                 Op::XOR => left != right,
//             //             };
//             //             def.insert(k, res);
//             //             changed = true;
//             //             return false;
//             //         }
//             //         return true;
//             //     });
//             //     if !changed {
//             //         break;
//             //     }
//             // }

//             // if to_handle.len() == 0 {
//             //     let tz = reduce_to_num(def.iter(), "z");
//             //     // println!("   - {zs} = {}", tz);
//             //     if tz == zs {
//             //         return true;
//             //     }
//             // }

//             // op.insert(to_perm[s1], sw1);
//             // op.insert(to_perm[s2], sw2);
//             // op.insert(to_perm[s3], sw3);
//             // op.insert(to_perm[s4], sw4);
//         }
//         return false;
//     });

//     f.unwrap()
//         .1
//         .into_iter()
//         .flat_map(|g| vec![sop[g].0, sop[g].2])
//         .sorted()
//         .join(",")
// }

// pub fn part_b_bruh(input: &str) -> String {
//     let (mut def, mut op) = parse(input);
//     let xs = reduce_to_num(def.iter(), "x");
//     let ys = reduce_to_num(def.iter(), "y");
//     let zs = xs + ys;

//     let wrong_bit = compute_wrong_bits(&mut def, &mut op, zs).unwrap();
//     let mut mask = 0;
//     for i in 0..=wrong_bit {
//         mask |= 1 << i;
//     }

//     let sw1 = get_all_swaps(&def, &op, mask, zs);
//     dbg!(&sw1);
//     println!("{}", sw1.len());

//     panic!("")
// }

// fn compute_wrong_bits<'a>(defs: &'a Definitions, ops: &'a Operations, zs: i64) -> Option<i64> {
//     let mut op = ops.clone();
//     let mut def = defs.clone();
//     while op.len() != 0 {
//         op.retain(|&k, (l, o, r)| {
//             if def.contains_key(l) && def.contains_key(r) {
//                 let left = def[*l];
//                 let right = def[*r];
//                 let res = match o {
//                     Op::AND => left && right,
//                     Op::OR => left || right,
//                     Op::XOR => left != right,
//                 };
//                 def.insert(k, res);
//                 return false;
//             }
//             return true;
//         });
//     }

//     // let z = reduce_to_num(def.iter(), "z");
//     // let mut wrong_bits = vec![];
//     // for i in 0..64 {
//     //     if (z >> i) & 1 != (zs >> i) & 1 {
//     //         wrong_bits.push(i);
//     //     }
//     // }
//     let z = reduce_to_num(def.iter(), "z");
//     return (0..64).find(|i| (z >> i) & 1 != (zs >> i) & 1);
// }

// fn get_all_swaps<'a>(
//     defs: &'a Definitions,
//     ops: &'a Operations,
//     mask: i64,
//     zs: i64,
// ) -> Vec<(&'a str, &'a str, i64)> {
//     ops.keys()
//         .tuple_combinations()
//         .filter_map(|(&ka, &kb)| {
//             let mut op = ops.clone();
//             let mut def = defs.clone();

//             let s = op[ka];
//             op.insert(ka, op[kb]);
//             op.insert(kb, s);

//             while op.len() != 0 {
//                 let mut changed = false;
//                 op.retain(|&k, (l, o, r)| {
//                     if def.contains_key(l) && def.contains_key(r) {
//                         let left = def[*l];
//                         let right = def[*r];
//                         let res = match o {
//                             Op::AND => left && right,
//                             Op::OR => left || right,
//                             Op::XOR => left != right,
//                         };
//                         def.insert(k, res);
//                         changed = true;
//                         return false;
//                     }
//                     return true;
//                 });
//                 if !changed {
//                     return None;
//                 }
//             }

//             let z = reduce_to_num(def.iter(), "z");
//             if zs & mask == z & mask {
//                 return Some((ka, kb, z));
//             }

//             return None;
//         })
//         .collect_vec()
// }

// pub fn part_b_old(input: &str) -> String {
//     let (mut def, mut op) = parse(input);
//     let xs = reduce_to_num(def.iter(), "x");
//     let ys = reduce_to_num(def.iter(), "y");
//     let zs = xs + ys;

//     let mut starting_def = def.clone();
//     let starting_op = op.clone();

//     while op.len() != 0 {
//         op.retain(|&k, (l, o, r)| {
//             if def.contains_key(l) && def.contains_key(r) {
//                 let left = def[*l];
//                 let right = def[*r];
//                 let res = match o {
//                     Op::AND => left && right,
//                     Op::OR => left || right,
//                     Op::XOR => left != right,
//                 };
//                 def.insert(k, res);
//                 return false;
//             }
//             return true;
//         });
//     }

//     let z = reduce_to_num(def.iter(), "z");
//     let mut wrong_bits = vec![];
//     for i in 0..64 {
//         if (z >> i) & 1 != (zs >> i) & 1 {
//             wrong_bits.push(i);
//         }
//     }

//     let mut wrong_gates = HashMap::new();
//     let mut wrong_values = wrong_bits
//         .into_iter()
//         .map(|bit| {
//             let zkey = format!("z{:0>2}", bit);
//             let got = (z >> bit) & 1;
//             let expect = (zs >> bit) & 1;
//             let (l, o, r) = starting_op[zkey.as_str()];
//             println!(
//                 "e: {}, g: {}, {} {:?} {} -> {} {:?} {}",
//                 expect, got, l, o, r, def[l], o, def[r]
//             );
//             let s: &str = zkey.clone().leak();
//             wrong_gates.insert(s, 1);
//             return (zkey, expect);
//         })
//         .collect_vec();

//     while let Some((v, expect)) = wrong_values.pop() {
//         let (l, o, r) = starting_op[v.as_str()];
//         let mut to_swap = vec![];
//         match o {
//             Op::AND => {
//                 if expect == 1 {
//                     if !def[l] {
//                         to_swap.push((l, 1));
//                     }
//                     if !def[r] {
//                         to_swap.push((r, 1));
//                     }
//                 } else {
//                     // If wrong, both are true
//                     to_swap.push((l, 0));
//                     to_swap.push((r, 0));
//                 }
//             }
//             Op::OR => {
//                 if expect == 1 {
//                     // If wrong, both are false
//                     to_swap.push((l, 1));
//                     to_swap.push((r, 1));
//                 } else {
//                     if def[l] {
//                         to_swap.push((l, 0));
//                     }
//                     if def[r] {
//                         to_swap.push((r, 0));
//                     }
//                 }
//             }
//             Op::XOR => {
//                 to_swap.push((l, 0));
//                 to_swap.push((l, 1));
//                 to_swap.push((r, 0));
//                 to_swap.push((r, 1));
//             }
//         }

//         for (k, v) in to_swap {
//             if k.starts_with("x") || k.starts_with("y") {
//                 continue;
//             }
//             wrong_gates.entry(k).and_modify(|e| *e += 1).or_insert(1);
//             wrong_values.push((k.to_string(), v));
//         }
//     }

//     dbg!(&wrong_values);
//     dbg!(&wrong_gates);
//     println!("{:?}", wrong_gates.len());

//     let deps = starting_op
//         .iter()
//         .fold(HashMap::new(), |mut map, (&k, (l, _, r))| {
//             map.entry(*l).or_insert_with(|| vec![]).push(k);
//             map.entry(*r).or_insert_with(|| vec![]).push(k);
//             return map;
//         });

//     let mut op = starting_op.clone();
//     let keys = wrong_gates
//         .into_keys()
//         .combinations(4)
//         .enumerate()
//         .find_map(move |(i, to_perm)| {
//             println!("{i}");
//             for (s1, s2, s3, s4) in vec![(0, 1, 2, 3), (0, 2, 1, 3), (0, 3, 1, 2)] {
//                 let p1 = to_perm[s1];
//                 let p2 = to_perm[s2];
//                 let p3 = to_perm[s3];
//                 let p4 = to_perm[s4];

//                 // println!(" - {} {} {} {}", p1, p2, p3, p4);

//                 let os1 = op[p1];
//                 let os2 = op[p2];
//                 let os3 = op[p3];
//                 let os4 = op[p4];

//                 op.insert(p1, os2);
//                 op.insert(p2, os1);

//                 op.insert(p3, os4);
//                 op.insert(p4, os3);

//                 let mut to_handle = starting_def.keys().collect_vec();
//                 let mut def = HashMap::new();
//                 while let Some(&key) = to_handle.pop() {
//                     for &k in &deps[key] {
//                         let (l, o, r) = op[k];
//                         if def.contains_key(l) && def.contains_key(r) {
//                             let left = def[l];
//                             let right = def[r];
//                             let res = match o {
//                                 Op::AND => left && right,
//                                 Op::OR => left || right,
//                                 Op::XOR => left != right,
//                             };
//                             def.insert(k, res);
//                         }
//                     }
//                 }

//                 let tz = reduce_to_num(def.iter(), "z");
//                 if tz == zs {
//                     return Some(to_perm);
//                 }

//                 // Swap back
//                 op.insert(p1, os1);
//                 op.insert(p2, os2);

//                 op.insert(p3, os3);
//                 op.insert(p4, os4);
//             }

//             None
//         });

//     keys.unwrap().into_iter().sorted().join(",")
// }
