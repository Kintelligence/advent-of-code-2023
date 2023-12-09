use shared::{parse::Parsable, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| predict(&parse_line(line)))
        .sum::<i32>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;

    #[test]
    fn example_input() {
        assert_eq!(
            part_1(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            Solution::I32(114)
        );
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::I32(1877825184));
    }
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| reverse_predict(&parse_line(line)))
        .sum::<i32>()
        .into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;

    #[test]
    fn example_input() {
        assert_eq!(
            part_2(
                "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            ),
            Solution::I32(2)
        );
    }

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::I32(1108));
    }
}

fn reverse_predict(sequence: &Vec<i32>) -> i32 {
    let deltas = deltas(sequence);
    let first = sequence[0];
    if deltas.iter().all(|d| *d == 0) {
        return first;
    }

    return first - reverse_predict(&deltas);
}

#[cfg(test)]
mod reverse_predict_tests {
    use crate::*;

    #[test]
    fn regular_input() {
        assert_eq!(reverse_predict(&vec![-3, 0, 3, 6, 9, 12, 15]), -6);
    }

    #[test]
    fn example_input_3() {
        assert_eq!(reverse_predict(&vec![10, 13, 16, 21, 30, 45]), 5);
    }
}

fn predict(sequence: &Vec<i32>) -> i32 {
    let deltas = deltas(sequence);
    let last = sequence.last().unwrap();
    if deltas.iter().all(|d| *d == 0) {
        return *last;
    }

    return last + predict(&deltas);
}

#[cfg(test)]
mod predict_tests {
    use crate::*;

    #[test]
    fn regular_input() {
        assert_eq!(predict(&vec![-3, 0, 3, 6, 9, 12, 15]), 18);
    }

    #[test]
    fn example_input_3() {
        assert_eq!(predict(&vec![10, 13, 16, 21, 30, 45]), 68);
    }
}

fn deltas(sequence: &Vec<i32>) -> Vec<i32> {
    sequence
        .windows(2)
        .map(|window| window[1] - window[0])
        .collect()
}

#[cfg(test)]
mod deltas_tests {
    use crate::*;

    #[test]
    fn regular_input() {
        assert_eq!(
            deltas(&vec![-3, 0, 3, 6, 9, 12, 15]),
            vec![3, 3, 3, 3, 3, 3]
        );
    }

    #[test]
    fn stable_input() {
        assert_eq!(deltas(&vec![1, 1, 1]), vec![0, 0]);
    }
}

fn parse_line(line: &str) -> Vec<i32> {
    let mut list: Vec<i32> = Vec::new();
    let mut bytes = line.bytes();
    while let Some(value) = bytes.next_number() {
        list.push(value)
    }
    list
}

#[cfg(test)]
mod parse_line_tests {
    use crate::*;

    #[test]
    fn regular_input() {
        assert_eq!(parse_line("0 3 6 9 12 15"), vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(parse_line("5 0 -5 -10 -15"), vec![5, 0, -5, -10, -15]);
    }
}
