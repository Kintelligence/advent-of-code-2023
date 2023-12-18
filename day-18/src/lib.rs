use shared::{parse::Parsable, point_vec2d::Direction, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    solve(_input.lines().map(|line| {
        let mut bytes = line.bytes();

        let direction = match bytes.next().unwrap() {
            b'U' => Direction::North,
            b'D' => Direction::South,
            b'L' => Direction::West,
            b'R' => Direction::East,
            _ => panic!("Oh no"),
        };
        let length: usize = bytes.next_number().unwrap();
        (length, direction)
    }))
}

pub fn part_2(_input: &str) -> Solution {
    solve(_input.lines().map(|line| {
        let (_, hex) = line.split_once('#').unwrap();
        let mut chars = hex.chars();

        let mut length: usize = 0;
        for _ in 0..5 {
            length = length * 16 + chars.next().unwrap().to_digit(16).unwrap() as usize;
        }

        let direction = match chars.next().unwrap() {
            '3' => Direction::North,
            '1' => Direction::South,
            '2' => Direction::West,
            '0' => Direction::East,
            _ => panic!("Oh no"),
        };

        (length, direction)
    }))
}

fn solve<T>(iter: T) -> Solution
where
    T: Iterator<Item = (usize, Direction)>,
{
    let mut current: (isize, isize) = (0, 0);
    let mut prev: (isize, isize) = (0, 0);

    let mut count: usize = 0;
    let mut s: isize = 0;

    for (length, direction) in iter {
        match direction {
            Direction::North => current.0 -= length as isize,
            Direction::South => current.0 += length as isize,
            Direction::West => current.1 -= length as isize,
            Direction::East => current.1 += length as isize,
        }

        s += current.0 * prev.1 - current.1 * prev.0;
        count += length;
        prev = current;
    }

    (s.abs() as usize / 2 + count / 2 + 1).into()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 62)]
    fn example_input_1(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test]
    fn real_input_1() {
        assert_eq!(part_1(_INPUT), 31171usize.into());
    }

    #[test_case(include_str!("_test.txt"), 952408144115)]
    fn example_input_2(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test]
    fn real_input_2() {
        assert_eq!(part_2(_INPUT), 131431655002266usize.into());
    }
}
