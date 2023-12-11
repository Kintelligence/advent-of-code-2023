use shared::{point_vec2d::Point, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (data, x_pyramid, y_pyramid) = parse(_input, 2);
    solve(&data, &x_pyramid, &y_pyramid).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;

    #[test]
    fn test_input_1() {
        let input = "...#......
                     ..........
                     .#........";

        assert_eq!(part_1(input), Solution::U128(6));
    }

    #[test]
    fn test_input_2() {
        let input = "...#......
                     ..........
                     .....#....";

        assert_eq!(part_1(input), Solution::U128(6));
    }

    #[test]
    fn test_input_3() {
        let input = "...#......
                     ..........
                     .#...#....";

        assert_eq!(part_1(input), Solution::U128(18));
    }

    #[test]
    fn test_input_4() {
        let input = "...#......
                     ..........
                     ...#......
                     ..........
                     ...#......";

        assert_eq!(part_1(input), Solution::U128(12));
    }

    #[test]
    fn test_input_5() {
        let input = ".......#..
                     #...#.....";

        assert_eq!(part_1(input), Solution::U128(13 + 7 + 6));
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

        assert_eq!(part_1(input), Solution::U128(374));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::U128(9639160));
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (data, x_pyramid, y_pyramid) = parse(_input, 1_000_000);
    solve(&data, &x_pyramid, &y_pyramid).into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::U128(752936133304));
    }
}

fn solve(galaxies: &Vec<Galaxy>, x_pyramid: &Vec<Vec<u128>>, y_pyramid: &Vec<Vec<u128>>) -> u128 {
    let mut sum: u128 = 0;

    for i in 0..galaxies.len() {
        let a = &galaxies[i];
        for b in &galaxies[i + 1..] {
            let min_x = a.ix.min(b.ix);
            let max_x = a.ix.max(b.ix);
            let min_y = a.iy.min(b.iy);
            let max_y = a.iy.max(b.iy);

            let mut dist = 0;

            if min_x != max_x {
                dist += x_pyramid[max_x - min_x - 1][min_x];
            }

            if min_y != max_y {
                dist += y_pyramid[max_y - min_y - 1][min_y];
            }

            sum += dist;
        }
    }

    sum
}

#[derive(Debug)]
struct Galaxy {
    loc: Point,
    ix: usize,
    iy: usize,
}

fn parse(input: &str, expansion: u128) -> (Vec<Galaxy>, Vec<Vec<u128>>, Vec<Vec<u128>>) {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut x_list: Vec<usize> = Vec::new();
    let mut y_list: Vec<usize> = Vec::new();

    let mut y = 0;
    for line in input.lines() {
        let mut x = 0;
        for byte in line.bytes() {
            match byte {
                b'#' => {
                    galaxies.push(Galaxy {
                        loc: Point::new(x, y),
                        ix: 0,
                        iy: 0,
                    });
                    x_list.push(x);
                    y_list.push(y);
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

    x_list.sort_unstable();
    x_list.dedup();

    y_list.sort_unstable();
    y_list.dedup();

    let mut iy = 0;
    for galaxy in galaxies.iter_mut() {
        if y_list[iy] == galaxy.loc.y {
            galaxy.iy = iy;
        } else {
            iy += 1;
            galaxy.iy = iy;
        }
    }

    let mut x_sort_galaxies: Vec<&mut Galaxy> = galaxies.iter_mut().collect();
    x_sort_galaxies.sort_by_key(|g| g.loc.x);

    let mut ix = 0;
    for galaxy in x_sort_galaxies.iter_mut() {
        if x_list[ix] == galaxy.loc.x {
            galaxy.ix = ix;
        } else {
            ix += 1;
            galaxy.ix = ix;
        }
    }

    (
        galaxies,
        build_pyramid(
            x_list
                .windows(2)
                .map(|slice| calculate_gap(slice[0], slice[1], expansion))
                .collect(),
        ),
        build_pyramid(
            y_list
                .windows(2)
                .map(|slice| calculate_gap(slice[0], slice[1], expansion))
                .collect(),
        ),
    )
}

fn build_pyramid(base: Vec<u128>) -> Vec<Vec<u128>> {
    let mut pyramid: Vec<Vec<u128>> = Vec::new();

    let len = base.len();
    pyramid.push(base);

    for i in 1..len {
        pyramid.push(
            pyramid[0]
                .windows(1 + i)
                .map(|slice| slice.iter().sum())
                .collect(),
        );
    }

    pyramid
}

fn calculate_gap(from: usize, to: usize, expansion: u128) -> u128 {
    (to - from - 1) as u128 * expansion + 1
}
