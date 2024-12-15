use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pulse {
    Low,
    High,
}

enum Module {
    FlipFlop(Pulse),
    Conjunction(HashMap<String, Pulse>),
    Broadcaster,
}

pub fn part_a(input: &str) -> i64 {
    let mut map = parse(input);

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut jobs = vec![("button".to_owned(), "broadcaster".to_owned(), Pulse::Low)];
        while !jobs.is_empty() {
            let (from, iden, pulse) = jobs.remove(0);
            // println!("{from} -{pulse:?} -> {iden}");
            match pulse {
                Pulse::Low => low += 1,
                Pulse::High => high += 1,
            }

            if let Some((module, output)) = map.get_mut(&iden) {
                if let Some(next) = module.pulse(pulse, from) {
                    for ele in output {
                        jobs.push((iden.clone(), ele.to_owned(), next));
                    }
                }
            }
        }
    }

    low * high
}

pub fn part_b(input: &str) -> i64 {
    let mut map = parse(input);

    let mut map_to_inputs = HashMap::new();
    for (key, (_, outputs)) in &map {
        for out in outputs {
            map_to_inputs
                .entry(out.clone())
                .or_insert(vec![])
                .push(key.clone())
        }
    }

    let input_to_rx = map_to_inputs.get(&"rx".to_owned()).unwrap().get(0).unwrap();

    let inputs = map_to_inputs.get(input_to_rx).unwrap();

    let mut input_iterations: HashMap<String, Option<i64>> =
        inputs.iter().map(|k| (k.clone(), None)).collect();

    let mut i = 0;
    loop {
        i += 1;
        if input_iterations.values().all(|v| v.is_some()) {
            return input_iterations
                .values()
                .flatten()
                .copied()
                .reduce(|a, b| lcm(a, b))
                .unwrap();
        }
        let mut jobs = vec![("button".to_owned(), "broadcaster".to_owned(), Pulse::Low)];
        while !jobs.is_empty() {
            let (from, iden, pulse) = jobs.remove(0);

            // Assuming all the inputs to hf are ff
            if inputs.contains(&iden) && pulse == Pulse::Low {
                input_iterations.insert(iden.clone(), Some(i));
            }

            if iden == "rx" && pulse == Pulse::Low {
                return i;
            }

            if let Some((module, output)) = map.get_mut(&iden) {
                if let Some(next) = module.pulse(pulse, from) {
                    for ele in output {
                        jobs.push((iden.clone(), ele.to_owned(), next));
                    }
                }
            }
        }
    }
}
fn parse(input: &str) -> HashMap<String, (Module, Vec<String>)> {
    let mut map: HashMap<String, (Module, Vec<String>)> = input
        .lines()
        .map(|l| {
            let (iden, output) = l.split_once(" -> ").unwrap();
            let outputs = output.split(", ").map(|s| s.to_owned()).collect_vec();

            let mut chars = iden.chars();
            let first = chars.next().unwrap();
            let binding = chars.join("");
            let rest = binding.as_str();
            let (module, name) = match first {
                // Flip-flop modules (prefix %) are either on or off; they are initially off
                '%' => (Module::FlipFlop(Pulse::Low), rest),

                '&' => (Module::Conjunction(HashMap::new()), rest),

                'b' => (Module::Broadcaster, "broadcaster"),

                _ => panic!("Unknown {}", first),
            };

            return (name.to_owned(), (module, outputs));
        })
        .collect();

    let keys = map.keys().map(|s| s.clone()).collect_vec();
    for key in keys {
        let (_, outputs) = map.get(&key).unwrap();
        for module in outputs.clone() {
            if let Some((module, _)) = map.get_mut(&module) {
                module.connect_input(key.clone());
            }
        }
    }

    return map;
}

impl Module {
    fn connect_input(&mut self, source: String) {
        match self {
            // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they
            // initially default to remembering a low pulse for each input.
            Self::Conjunction(ref mut inputs) => {
                inputs.insert(source, Pulse::Low);
            }
            _ => {}
        }
    }

    fn pulse(&mut self, pulse: Pulse, source: String) -> Option<Pulse> {
        match self {
            // There is a single broadcast module (named broadcaster). When it receives a pulse, it sends the same pulse to all of its destination modules.
            Self::Broadcaster => Some(pulse),

            // Flip-flop modules (prefix %) are either on or off; they are initially off. If a flip-flop module receives a high pulse, it is ignored and
            // nothing happens. However, if a flip-flop module receives a low pulse, it flips between on and off. If it was off, it turns on and sends a
            // high pulse. If it was on, it turns off and sends a low pulse.
            Self::FlipFlop(memory) => {
                // println!("FLip FLip ({memory:?}) - {pulse:?}");
                match pulse {
                    Pulse::High => None,
                    Pulse::Low => {
                        let flipped = match memory {
                            Pulse::Low => Pulse::High,
                            Pulse::High => Pulse::Low,
                        };
                        *self = Self::FlipFlop(flipped);
                        return Some(flipped);
                    }
                }
            }

            // Conjunction modules (prefix &) remember the type of the most recent pulse received from each of their connected input modules; they
            // initially default to remembering a low pulse for each input. When a pulse is received, the conjunction module first updates its memory
            // for that input. Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
            Self::Conjunction(ref mut inputs) => {
                inputs.insert(source, pulse);
                if inputs.values().all(|s| s.is_high()) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

impl Pulse {
    fn is_low(&self) -> bool {
        match self {
            Self::Low => true,
            Self::High => false,
        }
    }

    fn is_high(&self) -> bool {
        !self.is_low()
    }
}

fn lcm(first: i64, second: i64) -> i64 {
    first * second / gcd(first, second)
}

fn gcd(first: i64, second: i64) -> i64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
