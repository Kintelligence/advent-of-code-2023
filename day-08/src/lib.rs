use node::node::*;
use num::Integer;
use shared::*;
extern crate shared;
mod node;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (directions, map, _) = parse(_input);
    let mut direction_iter = directions.iter().cycle();
    let mut steps: usize = 0;
    let mut current: usize = 0;

    let target = hash("ZZZ");

    loop {
        if current == target {
            break;
        }
        steps += 1;
        let fork = map[current];
        current = fork.go(direction_iter.next().unwrap());
    }

    steps.into()
}

pub fn part_2(_input: &str) -> Solution {
    let (directions, map, mut currents) = parse(_input);
    let mut result: usize = 1;

    for current in currents.iter_mut() {
        let mut direction_iter = directions.iter().cycle();
        let mut steps: usize = 0;

        loop {
            if *current & 0b11111 == 25 {
                break;
            }

            let fork = map[*current];
            *current = fork.go(direction_iter.next().unwrap());

            steps += 1;
        }

        result = result.lcm(&steps);
    }

    result.into()
}

fn parse(input: &str) -> (Vec<Direction>, [Fork; 26426], Vec<usize>) {
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

    let mut map: [Fork; 26426] = [Fork { left: 0, right: 0 }; 26426];
    let mut starts: Vec<usize> = Vec::new();

    for line in lines {
        let node = Node::parse(line);
        if node.id & 0b11111 == 0 {
            starts.push(node.id);
        }
        map[node.id] = node.fork;
    }

    (direction, map, starts)
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

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
    #[serial]
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
    #[serial]
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
    #[serial]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(15517));
    }

    #[test]
    #[serial]
    fn part_2_test() {
        let input = "LR

CCA = (CCB, XXX)
CCB = (XXX, CCZ)
CCZ = (CCB, XXX)
DDA = (DDB, XXX)
DDB = (DDC, DDC)
DDC = (DDZ, DDZ)
DDZ = (DDB, DDB)
XXX = (XXX, XXX)";
        assert_eq!(part_2(input), Solution::Usize(6));
    }

    #[test]
    #[serial]
    fn part_2_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(14935034899483));
    }
}
