use shared::{point_vec2d::Point, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    solve(&parse(_input, 2)).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;

    #[test]
    fn test_input_1() {
        let input = "...#......
                     ..........
                     .#........";

        assert_eq!(part_1(input), Solution::Usize(6));
    }

    #[test]
    fn test_input_2() {
        let input = "...#......
                     ..........
                     .....#....";

        assert_eq!(part_1(input), Solution::Usize(6));
    }

    #[test]
    fn test_input_3() {
        let input = "...#......
                     ..........
                     .#...#....";

        assert_eq!(part_1(input), Solution::Usize(18));
    }

    #[test]
    fn test_input_4() {
        let input = "...#......
                     ..........
                     ...#......
                     ..........
                     ...#......";

        assert_eq!(part_1(input), Solution::Usize(12));
    }

    #[test]
    fn test_input_5() {
        let input = ".......#..
                     #...#.....";

        assert_eq!(part_1(input), Solution::Usize(13 + 7 + 6));
    }

    #[test]
    fn example_input() {
        let input = "...#......
                     .......#..
                     #.........
                     ..........
                     ......#...
                     .#........
                     .........#
                     ..........
                     .......#..
                     #...#.....";

        assert_eq!(part_1(input), Solution::Usize(374));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(9639160));
    }
}

pub fn part_2(_input: &str) -> Solution {
    solve(&parse(_input, 1_000_000)).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(752936133304));
    }
}

fn solve(galaxies: &Vec<Point>) -> usize {
    let mut sum: usize = 0;

    for i in 0..galaxies.len() {
        let a = &galaxies[i];
        for b in &galaxies[i + 1..] {
            sum += a.x.abs_diff(b.x) + a.y.abs_diff(b.y);
        }
    }

    sum
}

fn parse(input: &str, expansion: usize) -> Vec<Point> {
    let mut galaxies: Vec<Point> = Vec::new();
    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for byte in line.bytes() {
            match byte {
                b'#' => {
                    galaxies.push(Point::new(x, y));
                    x += 1;
                }
                b'.' => {
                    x += 1;
                }
                _ => {}
            }
        }
        y += 1;
    }

    let mut sum_expansion = 0;
    let mut last_y = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.y != last_y {
            sum_expansion += (galaxy.y - last_y - 1) * (expansion - 1);
            last_y = galaxy.y;
        }
        galaxy.y += sum_expansion;
    }

    galaxies.sort_by_key(|galaxy| galaxy.x);

    sum_expansion = 0;
    let mut last_x = 0;
    for galaxy in galaxies.iter_mut() {
        if galaxy.x != last_x {
            sum_expansion += (galaxy.x - last_x - 1) * (expansion - 1);
            last_x = galaxy.x;
        }

        galaxy.x += sum_expansion;
    }

    galaxies
}
