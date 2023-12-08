use shared::*;
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (directions, map) = parse(_input);

    let direction_len = directions.len();
    let mut steps: usize = 0;
    let mut current: usize = 0;
    let mut direction_index = 0;
    let target = hash("ZZZ");

    loop {
        if current == target {
            break;
        }

        steps += 1;
        let direction = &directions[direction_index];
        if let Some(node) = map[current] {
            current = match direction {
                Direction::Left => node.left,
                Direction::Right => node.right,
            };

            direction_index += 1;
            if direction_index == direction_len {
                direction_index = 0;
            }
        } else {
            panic!("WTF")
        }
    }

    steps.into()
}

pub fn part_2(_input: &str) -> Solution {
    Solution::None
}

#[derive(Clone, Copy)]
struct Node {
    id: usize,
    left: usize,
    right: usize,
}

fn parse(input: &str) -> (Vec<Direction>, [Option<Node>; 26426]) {
    let mut lines = input.lines();

    let mut direction: Vec<Direction> = Vec::new();

    if let Some(line) = lines.next() {
        for char in line.chars() {
            match char {
                'L' => direction.push(Direction::Left),
                'R' => direction.push(Direction::Right),
                _ => panic!("oh no"),
            }
        }
    }

    lines.next();

    let nodes: Vec<Node> = lines.map(|line| Node::parse(line)).collect();

    let mut map: [Option<Node>; 26426] = [None; 26426];

    for node in nodes {
        map[node.id] = Some(node);
    }

    (direction, map)
}

enum Direction {
    Left,
    Right,
}

impl Node {
    fn parse(input: &str) -> Self {
        let id = hash(&input[0..3]);
        let left = hash(&input[7..10]);
        let right = hash(&input[12..15]);
        Node { id, left, right }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} = ({}, {})",
            reverse_hash(self.id),
            reverse_hash(self.left),
            reverse_hash(self.right)
        )
    }
}

fn hash(input: &str) -> usize {
    let mut value: usize = 0;

    for (i, char) in input.chars().enumerate() {
        value |= (char as usize - 'A' as usize) << i * 5;
    }

    value
}

fn reverse_hash(input: usize) -> String {
    let mut string: String = String::from("");
    for i in 0..3 {
        let current = ((((input & 0b11111 << i * 5) >> i * 5) + 'A' as usize) as u8) as char;
        string.push(current);
    }

    string
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn can_parse_line() {
        let input = "AAA = (BBB, CCC)";
        let node = Node::parse(input);

        assert_eq!(format!("{}", node), input);
    }

    #[test]
    fn can_reverse_hash() {
        let input = "ZZZ";
        let hash = hash(input);
        let reverse = reverse_hash(hash);
        assert_eq!(input, reverse);
    }

    #[test]
    fn part_1_test_1() {
        assert_eq!(
            part_1(
                "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
            ),
            Solution::Usize(2)
        );
    }

    #[test]
    fn part_1_test_2() {
        assert_eq!(
            part_1(
                "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
            ),
            Solution::Usize(6)
        );
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(15517));
    }

    #[test]
    fn part_2_test() {
        assert_eq!(
            part_1(
                "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
            ),
            Solution::Usize(6)
        );
    }
}
