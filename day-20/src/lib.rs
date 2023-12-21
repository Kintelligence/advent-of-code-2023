use std::collections::{HashMap, VecDeque};

use num::{complex::ComplexFloat, Integer};
use shared::*;
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_example_1.txt"), 32000000)]
    #[test_case(include_str!("_example_2.txt"), 11687500)]
    #[test_case(_INPUT, 818649769)]
    fn part_1_test(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(_INPUT, 246313604784977)]
    fn part_2_test(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }
}

pub fn part_1(_input: &str) -> Solution {
    let mut machine = parse(_input);

    let clicks = 1000;
    let a = Vec::new();
    for _ in 0..clicks {
        machine.click(&a);
    }

    (machine.low * machine.high).into()
}

pub fn part_2(_input: &str) -> Solution {
    let mut machine = parse(_input);

    let a = machine
        .modules
        .iter()
        .filter(|module| module.destinations.contains(&1))
        .last()
        .unwrap();

    if let Kind::Conjunction(conj) = &a.kind {
        let ids: Vec<usize> = conj.connections.keys().map(|k| *k).collect();

        let mut hist: HashMap<usize, usize> = HashMap::new();
        let mut cycle_lengths: HashMap<usize, usize> = HashMap::new();
        let mut i: usize = 0;
        loop {
            i += 1;
            let hits = machine.click(&ids);
            for id in hits {
                // println!("found {} at {}", id, i);
                if let Some(dup) = hist.insert(id, i) {
                    if dup != i {
                        // println!("found duplicate for {} at {}", id, dup);
                        if !cycle_lengths.contains_key(&id) {
                            cycle_lengths.insert(id, i - dup);
                            if cycle_lengths.len() == ids.len() {
                                let mut result = 1;
                                for id in ids.iter() {
                                    result = result.lcm(&cycle_lengths[id]);
                                }
                                return result.into();
                            }
                        }
                    }
                }
            }
        }
    } else {
        panic!("Expected a to be conjunction");
    }
}

fn parse(input: &str) -> Machine {
    let mut map = HashMap::new();
    map.insert("broadcast", 0);
    map.insert("rx", 1);
    let mut i: usize = 2;

    let mut modules = Vec::new();

    for line in input.lines() {
        let (name, destinations) = line.split_once(" -> ").unwrap();
        let mut module = Module {
            id: 0,
            destinations: parse_destinations(destinations, &mut map, &mut i),
            kind: match name.chars().next().unwrap() {
                'b' => Kind::Broadcast,
                '&' => Kind::Conjunction(Conjunction {
                    connections: HashMap::new(),
                }),
                '%' => Kind::FlipFlop,
                _ => panic!("unexpected kind"),
            },
            signal: false,
        };

        if let Kind::Broadcast = module.kind {
            modules.push(module);
        } else {
            let name = &name[1..];
            if let Some(id) = map.get(name) {
                module.id = *id;
                modules.push(module);
            } else {
                module.id = i;
                map.insert(name, i);
                modules.push(module);
                i += 1;
            }
        }
    }

    let debugs: Vec<&usize> = map
        .values()
        .filter(|id| !modules.iter().any(|m| m.id == **id))
        .collect();

    for debug_id in debugs {
        modules.push(Module {
            id: *debug_id,
            kind: Kind::Debug,
            destinations: Vec::new(),
            signal: false,
        });
    }

    modules.sort_unstable_by_key(|module| module.id);

    for i in 0..modules.len() {
        for d in 0..modules[i].destinations.len() {
            let dest = modules[i].destinations[d];
            if let Kind::Conjunction(ref mut conj) = &mut modules[dest].kind {
                conj.connections.insert(i, false);
            }
        }
    }

    Machine {
        modules,
        low: 0,
        high: 0,
    }
}

fn parse_destinations<'a>(
    input: &'a str,
    map: &mut HashMap<&'a str, usize>,
    i: &mut usize,
) -> Vec<usize> {
    input
        .split(", ")
        .map(|name| {
            if let Some(id) = map.get(name) {
                return *id;
            } else {
                map.insert(name, *i);
                *i += 1;
                return *i - 1;
            }
        })
        .collect()
}

#[derive(Clone, Debug)]
struct Machine {
    modules: Vec<Module>,
    low: usize,
    high: usize,
}

impl Machine {
    fn state(&self) -> u64 {
        let mut state: u64 = 0;
        for module in self.modules.iter() {
            state = state << 1 | if module.signal { 0b1 } else { 0b0 };
        }
        state
    }

    fn click(&mut self, watch: &Vec<usize>) -> Vec<usize> {
        let mut queue: VecDeque<(usize, bool, usize)> = VecDeque::new();
        queue.push_back((0, false, 0));

        let mut hits = Vec::new();

        while let Some((id, signal, sender)) = queue.pop_front() {
            if signal {
                self.high += 1;
            } else {
                self.low += 1;
                if watch.contains(&id) {
                    hits.push(id);
                }
            }
            if let Some(next) = self.modules[id].receive(signal, sender) {
                for (next_id, next_signal) in next {
                    queue.push_back((next_id, next_signal, id));
                }
            }
        }

        hits
    }
}

#[derive(Clone, Debug)]
struct Module {
    id: usize,
    kind: Kind,
    destinations: Vec<usize>,
    signal: bool,
}

impl Module {
    fn receive(&mut self, signal: bool, sender: usize) -> Option<Vec<(usize, bool)>> {
        match &mut self.kind {
            Kind::Broadcast => {}
            Kind::FlipFlop => {
                if signal {
                    return None;
                }
                self.signal = !self.signal;
            }
            Kind::Conjunction(ref mut conj) => {
                conj.connections.insert(sender, signal);
                if conj.connections.values().all(|signal| *signal) {
                    self.signal = false;
                } else {
                    self.signal = true;
                }
            }
            Kind::Debug => return None,
        }

        Some(
            self.destinations
                .iter()
                .map(|dest| (*dest, self.signal))
                .collect(),
        )
    }
}

#[derive(Clone, Debug)]
enum Kind {
    Broadcast,
    FlipFlop,
    Conjunction(Conjunction),
    Debug,
}

#[derive(Clone, Debug)]
struct Conjunction {
    connections: HashMap<usize, bool>,
}
