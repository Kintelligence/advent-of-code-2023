use shared::*;
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .split("\n\n")
        .map(|map| solve(&Map::parse(map), false))
        .sum::<usize>()
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .split("\n\n")
        .map(|map| solve(&Map::parse(map), true))
        .sum::<usize>()
        .into()
}

#[derive(Debug)]
struct Map {
    rows: Vec<u16>,
    columns: Vec<u16>,
}

fn solve(map: &Map, is_smudged: bool) -> usize {
    if let Some(result) = find_mirror(&map.columns, is_smudged) {
        return result;
    }

    if let Some(result) = find_mirror(&map.rows, is_smudged) {
        return result * 100;
    }

    0
}

fn find_mirror(lines: &Vec<u16>, is_smudged: bool) -> Option<usize> {
    let count = lines.len();

    if is_smudged {
        for i in 1..count {
            if is_smudged_mirror(&lines[0..i], &lines[i..]) {
                return Some(i);
            }
        }
    } else {
        for i in 1..count {
            if is_mirror(&lines[0..i], &lines[i..]) {
                return Some(i);
            }
        }
    }

    None
}

fn is_mirror(left: &[u16], right: &[u16]) -> bool {
    let mut l = left.iter().rev();
    let mut r = right.iter();

    while let Some(a) = l.next() {
        if let Some(b) = r.next() {
            if a != b {
                return false;
            }
        } else {
            break;
        }
    }

    true
}

fn is_smudged_mirror(left: &[u16], right: &[u16]) -> bool {
    let mut l = left.iter().rev();
    let mut r = right.iter();

    let mut smudges = 0;

    while let Some(a) = l.next() {
        if let Some(b) = r.next() {
            smudges += (a ^ b).count_ones();
            if smudges > 1 {
                return false;
            }
        } else {
            break;
        }
    }

    smudges == 1
}

impl Map {
    fn parse(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();

        let mut rows = vec![0; height];
        let mut columns = vec![0; width];
        let mut bytes = input.bytes();

        let (mut x, mut y) = (0, 0);
        while let Some(byte) = bytes.next() {
            match byte {
                b'.' => x += 1,
                b'#' => {
                    rows[y] |= 0b1 << x;
                    columns[x] |= 0b1 << y;
                    x += 1;
                }
                b'\n' => {
                    x = 0;
                    y += 1;
                }
                _ => {}
            }
        }

        Map { rows, columns }
    }
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test]
    fn test() {
        assert_eq!(part_1(_INPUT), Solution::Usize(29846));
    }

    #[test_case("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", 405; "example")]
    #[test_case("#...#..##
#.#.#..##
###..##..
#..##.###
..#.#.#..
#####..##
.#....#..", 8; "real 3")]
    fn example_input(input: &str, result: usize) {
        assert_eq!(part_1(input), Solution::Usize(result));
    }
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test]
    fn test() {
        assert_eq!(part_2(_INPUT), Solution::Usize(0));
    }

    #[test_case("##.....##
###.#..##
###..##..
...##.###
..#.#.#..
#####..##
.#....#..", 1; "real 3")]
    #[test_case("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#", 400; "example")]
    fn example_input(input: &str, result: usize) {
        assert_eq!(part_2(input), Solution::Usize(result));
    }
}
