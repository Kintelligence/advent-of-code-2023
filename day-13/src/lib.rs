use shared::*;
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .split("\n\n")
        .enumerate()
        .map(|(i, map)| {
            let m = Map::parse(map);
            let r = solve(&m);
            if r == 0 {
                println!("Failed at {}", i);
            }
            r
        })
        .sum::<usize>()
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .split("\n\n")
        .enumerate()
        .map(|(i, map)| {
            let m = Map::parse(map);
            let r = solve_smudged(&m);
            if r == 0 {
                println!("Failed at {}", i);
            }
            r
        })
        .sum::<usize>()
        .into()
}

#[derive(Debug)]
struct Map {
    rows: Vec<u16>,
    columns: Vec<u16>,
}

fn solve(map: &Map) -> usize {
    if let Some(result) = find_mirror(&map.rows) {
        return result * 100;
    }

    if let Some(result) = find_mirror(&map.columns) {
        return result;
    }

    0
}

fn find_mirror(lines: &Vec<u16>) -> Option<usize> {
    let count = lines.len();

    for i in 1..count {
        if is_mirror(&lines[0..i], &lines[i..]) {
            return Some(i);
        }
    }

    None
}

fn is_mirror(left: &[u16], right: &[u16]) -> bool {
    let mut l = left.iter().rev();
    let mut r = right.iter();

    loop {
        if let Some(a) = l.next() {
            if let Some(b) = r.next() {
                if a != b {
                    return false;
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }

    true
}

fn solve_smudged(map: &Map) -> usize {
    if let Some(result) = find_smudged_mirror(&map.rows) {
        return result * 100;
    }

    if let Some(result) = find_smudged_mirror(&map.columns) {
        return result;
    }

    0
}

fn find_smudged_mirror(lines: &Vec<u16>) -> Option<usize> {
    let count = lines.len();

    for i in 1..count {
        if is_smudged_mirror(&lines[0..i], &lines[i..]) {
            return Some(i);
        }
    }

    None
}

fn is_smudged_mirror(left: &[u16], right: &[u16]) -> bool {
    let mut l = left.iter().rev();
    let mut r = right.iter();

    let mut smudges = 0;

    loop {
        if let Some(a) = l.next() {
            if let Some(b) = r.next() {
                smudges += (a ^ b).count_ones();
                if smudges > 1 {
                    return false;
                }
            } else {
                break;
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

        let mut rows = Vec::new();
        let mut columns = vec![0; width];
        let mut bytes = input.bytes();

        let (mut x, mut y) = (0, 0);

        rows.push(0);
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
                    rows.push(0);
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
