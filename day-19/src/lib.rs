use std::collections::HashMap;

use shared::{parse::Parsable, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (workflows, parts) = parse(_input);
    parts
        .iter()
        .filter_map(|part| {
            let mut id: u32 = 213;
            while let Some(workflow) = workflows.get(&id) {
                let mut rules = workflow.rules.iter();
                while let Some(rule) = rules.next() {
                    if let Some(destination) = rule.apply(&part) {
                        match destination {
                            Destination::Workflow(next) => {
                                id = next;
                                break;
                            }
                            Destination::Reject => return None,
                            Destination::Accept => return Some(part.x + part.m + part.a + part.s),
                        }
                    }
                }
            }

            todo!()
        })
        .sum::<usize>()
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    let (workflows, _) = parse(_input);

    let mut valid: Vec<PartRange> = Vec::new();

    let mut queue: Vec<(PartRange, &Workflow)> = vec![(
        PartRange {
            x: Range::new(1, 4000),
            m: Range::new(1, 4000),
            a: Range::new(1, 4000),
            s: Range::new(1, 4000),
        },
        &workflows[&213],
    )];

    while let Some((mut range, workflow)) = queue.pop() {
        for rule in workflow.rules.iter() {
            let (yes, no) = range.split(&rule);

            if let Some(yes) = yes {
                match rule.destination {
                    Destination::Workflow(id) => queue.push((yes, &workflows[&id])),
                    Destination::Reject => {}
                    Destination::Accept => valid.push(yes),
                }
            }

            if let Some(no) = no {
                range = no;
            } else {
                break;
            }
        }
    }

    valid
        .iter()
        .map(|range| range.range())
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 19114)]
    #[test_case(_INPUT, 0)]
    fn part_1_test(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(include_str!("_test.txt"), 167409079868000)]
    #[test_case(_INPUT, 0)]
    fn part_2_test(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }
}

#[derive(Clone, Debug)]
struct Workflow {
    rules: Vec<Rule>,
}
#[derive(Clone, Copy, Debug)]
struct Rule {
    category: Cat,
    validation: Validation,
    destination: Destination,
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<Destination> {
        let value = match self.category {
            Cat::X => part.x,
            Cat::M => part.m,
            Cat::A => part.a,
            Cat::S => part.s,
        };

        if match self.validation {
            Validation::Less(threshold) => value < threshold,
            Validation::More(threshold) => value > threshold,
            Validation::True => true,
        } {
            return Some(self.destination);
        }

        None
    }
}

#[derive(Clone, Copy, Debug)]
enum Destination {
    Workflow(u32),
    Reject,
    Accept,
}
#[derive(Clone, Copy, Debug)]
enum Validation {
    Less(usize),
    More(usize),
    True,
}
#[derive(Clone, Copy, Debug)]
enum Cat {
    X,
    M,
    A,
    S,
}
#[derive(Clone, Copy, Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}
// hkc{x>2690:R,m<2241:A,a<3454:R,A}
fn parse(input: &str) -> (HashMap<u32, Workflow>, Vec<Part>) {
    let mut workflows = HashMap::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        let (id_input, rest) = line.split_once('{').unwrap();
        let id = parse_id(&mut id_input.bytes());
        let mut rules = Vec::new();

        for rule_input in rest.split([',', '}']) {
            if let Some(rule) = parse_rule(&mut rule_input.bytes()) {
                rules.push(rule);
            } else if let Some(destination) = parse_destination(&mut rule_input.bytes()) {
                rules.push(Rule {
                    destination,
                    category: Cat::X,
                    validation: Validation::True,
                });
                break;
            } else {
                break;
            }
        }

        workflows.insert(id, Workflow { rules });
    }

    let mut parts = Vec::new();
    while let Some(line) = lines.next() {
        let mut bytes = line.bytes();
        parts.push(Part {
            x: bytes.next_number().unwrap(),
            m: bytes.next_number().unwrap(),
            a: bytes.next_number().unwrap(),
            s: bytes.next_number().unwrap(),
        })
    }

    (workflows, parts)
}

// x>2690:R,m<2241:A,a<3454:R,A}
fn parse_rule<T>(bytes: &mut T) -> Option<Rule>
where
    T: Iterator<Item = u8>,
{
    if let Some(byte) = bytes.next() {
        let category;
        match byte {
            b'x' => category = Cat::X,
            b'm' => category = Cat::M,
            b'a' => category = Cat::A,
            b's' => category = Cat::S,
            _ => return None,
        };

        let validation;
        match bytes.next().unwrap() {
            b'<' => validation = Validation::Less(bytes.next_number().unwrap()),
            b'>' => validation = Validation::More(bytes.next_number().unwrap()),
            _ => return None,
        };

        let destination = parse_destination(bytes).unwrap();

        return Some(Rule {
            category,
            validation,
            destination,
        });
    }
    None
}

fn parse_destination<T>(bytes: &mut T) -> Option<Destination>
where
    T: Iterator<Item = u8>,
{
    let byte = bytes.next().unwrap();

    if byte == b'A' {
        bytes.next();
        return Some(Destination::Accept);
    }
    if byte == b'R' {
        bytes.next();
        return Some(Destination::Reject);
    }
    if !byte.is_ascii_alphabetic() {
        return None;
    }

    let mut id: u32 = (byte - b'a') as u32;

    while let Some(byte) = bytes.next() {
        if !byte.is_ascii_alphabetic() {
            break;
        }

        id = id * 25 + (byte - b'a') as u32;
    }
    Some(Destination::Workflow(id))
}

fn parse_id<T>(bytes: &mut T) -> u32
where
    T: Iterator<Item = u8>,
{
    let mut id: u32 = 0;
    while let Some(byte) = bytes.next() {
        if !byte.is_ascii_alphabetic() {
            break;
        }
        id = id * 25 + (byte - 'a' as u8) as u32;
    }
    id
}

#[derive(Clone, Copy)]
struct PartRange {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl PartRange {
    fn range(&self) -> usize {
        self.x.range() * self.m.range() * self.a.range() * self.s.range()
    }

    fn split(&self, rule: &Rule) -> (Option<Self>, Option<Self>) {
        match rule.validation {
            Validation::Less(threshold) => match rule.category {
                Cat::X => {
                    let (yes, no) = self.x.split(threshold);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.x = yes.unwrap();
                        b.x = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
                Cat::M => {
                    let (yes, no) = self.m.split(threshold);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.m = yes.unwrap();
                        b.m = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
                Cat::A => {
                    let (yes, no) = self.a.split(threshold);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.a = yes.unwrap();
                        b.a = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
                Cat::S => {
                    let (yes, no) = self.s.split(threshold);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.s = yes.unwrap();
                        b.s = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
            },
            Validation::More(threshold) => match rule.category {
                Cat::X => {
                    let (no, yes) = self.x.split(threshold + 1);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.x = yes.unwrap();
                        b.x = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
                Cat::M => {
                    let (no, yes) = self.m.split(threshold + 1);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.m = yes.unwrap();
                        b.m = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
                Cat::A => {
                    let (no, yes) = self.a.split(threshold + 1);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.a = yes.unwrap();
                        b.a = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
                Cat::S => {
                    let (no, yes) = self.s.split(threshold + 1);
                    if yes.is_some() && no.is_some() {
                        let mut a = self.clone();
                        let mut b = self.clone();
                        a.s = yes.unwrap();
                        b.s = no.unwrap();

                        return (Some(a), Some(b));
                    }

                    if yes.is_some() {
                        return (Some(*self), None);
                    }

                    return (None, Some(*self));
                }
            },
            Validation::True => return (Some(*self), None),
        }
    }
}

#[derive(Clone, Copy)]
struct Range {
    min: usize,
    max: usize,
}

impl Range {
    fn new(min: usize, max: usize) -> Self {
        Range { min, max }
    }

    fn range(&self) -> usize {
        self.max - self.min + 1
    }

    fn split(&self, threshold: usize) -> (Option<Self>, Option<Self>) {
        if self.min >= threshold {
            return (None, Some(*self));
        }

        if self.max < threshold {
            return (Some(*self), None);
        }

        (
            Some(Range::new(self.min, threshold - 1)),
            Some(Range::new(threshold, self.max)),
        )
    }
}
