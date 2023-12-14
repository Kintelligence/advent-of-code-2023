use std::collections::HashMap;

use shared::{vec2d::Vec2d, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let map = parse(_input);

    let mut sum: usize = 0;
    for x in 0..map.width {
        let mut value = map.height;
        for y in 0..map.height {
            match map[(x, y)] {
                Rock::None => {}
                Rock::Round => {
                    sum += value;
                    value -= 1;
                }
                Rock::Cube => {
                    value = map.height - y - 1;
                }
            }
        }
    }

    sum.into()
}

pub fn part_2(_input: &str) -> Solution {
    let mut map = parse(_input);

    let mut history: HashMap<(usize, usize), usize> = HashMap::new();

    let mut sum: usize = 0;
    for i in 0..1_000_000_000 {
        //println!("{}", &map);
        //println!("North");
        for x in 0..map.width {
            let mut value = 0;
            for y in 0..map.height {
                match map[(x, y)] {
                    Rock::None => {}
                    Rock::Round => {
                        // println!("found round at {}:{} -> {}:{}", x, y, x, value);
                        map[(x, y)] = Rock::None;
                        map[(x, value)] = Rock::Round;

                        value += 1;
                    }
                    Rock::Cube => {
                        value = y + 1;
                    }
                }
            }
        }

        //println!("{}", &map);
        //println!("East");
        for y in 0..map.height {
            let mut value = 0;
            for x in 0..map.width {
                match map[(x, y)] {
                    Rock::None => {}
                    Rock::Round => {
                        // println!("found round at {}:{} -> {}:{}", x, y, value, y);
                        map[(x, y)] = Rock::None;
                        map[(value, y)] = Rock::Round;

                        value += 1;
                    }
                    Rock::Cube => {
                        value = x + 1;
                    }
                }
            }
        }

        let mut a = 0;
        //println!("{}", &map);
        //println!("South");
        for x in 0..map.width {
            let mut value = map.height - 1;
            for y in (0..map.height).rev() {
                match map[(x, y)] {
                    Rock::None => {}
                    Rock::Round => {
                        //println!("found round at {}:{} -> {}:{}", x, y, x, value);
                        a += map.width - x;
                        map[(x, y)] = Rock::None;
                        map[(x, value)] = Rock::Round;

                        value -= 1;
                    }
                    Rock::Cube => {
                        value = y - 1;
                    }
                }
            }
        }

        sum = 0;
        //println!("{}", &map);
        //println!("West");
        for y in 0..map.height {
            let mut value = map.width - 1;
            for x in (0..map.width).rev() {
                match map[(x, y)] {
                    Rock::None => {}
                    Rock::Round => {
                        // println!("found round at {}:{} -> {}:{}", x, y, value, y);
                        sum += map.height - y;
                        map[(x, y)] = Rock::None;
                        map[(value, y)] = Rock::Round;

                        value -= 1;
                    }
                    Rock::Cube => {
                        value = x - 1;
                    }
                }
            }
        }

        if let Some(duplicate) = history.insert((sum, a), i) {
            let left = 1_000_000_000 - i - 1;
            let cycle = i - duplicate;
            let remainder = left % cycle;

            return history
                .iter()
                .find(|(_, value)| **value == duplicate + remainder)
                .unwrap()
                .0
                 .0
                .into();
        }
    }
    sum.into()
}

fn parse(input: &str) -> Vec2d<Rock> {
    let mut bytes = input.bytes();
    let mut vec = Vec::new();
    let mut height = 1;

    while let Some(byte) = bytes.next() {
        match byte {
            b'O' => vec.push(Rock::Round),
            b'#' => vec.push(Rock::Cube),
            b'.' => vec.push(Rock::None),
            b'\n' => height += 1,
            _ => {}
        }
    }

    Vec2d::from_vec(vec, height)
}

enum Rock {
    None,
    Round,
    Cube,
}

impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Rock::None => " ",
                Rock::Round => "o",
                Rock::Cube => "#",
            }
        )
    }
}

impl std::fmt::Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, " "),
            Self::Round => write!(f, "o"),
            Self::Cube => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(113424));
    }

    #[test_case("O....#....
                 O.OO#....#
                 .....##...
                 OO.#O....O
                 .O.....O#.
                 O.#..O.#.#
                 ..O..#O..O
                 .......O..
                 #....###..
                 #OO..#....", 136; "Example")]
    fn test_input(input: &str, result: usize) {
        assert_eq!(part_1(input), Solution::Usize(result));
    }
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(0));
    }

    #[test_case("O....#....
                 O.OO#....#
                 .....##...
                 OO.#O....O
                 .O.....O#.
                 O.#..O.#.#
                 ..O..#O..O
                 .......O..
                 #....###..
                 #OO..#....", 64; "Example")]
    fn test_input(input: &str, result: usize) {
        assert_eq!(part_2(input), Solution::Usize(result));
    }
}
