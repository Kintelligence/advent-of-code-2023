use std::collections::VecDeque;

use shared::{
    point_vec2d::{Point, PointVec2d},
    *,
};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (map, start) = parse(_input);
    solve(&map, start, 64).into()
}

fn solve(map: &PointVec2d<Tile>, start: Point, steps: usize) -> usize {
    let mut visited = PointVec2d::from_vec(vec![0; map.height * map.width], map.height);

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let parity = steps % 2;

    let mut result: usize = 0;

    while let Some((node, i)) = queue.pop_front() {
        if visited[node] == 0 {
            if i % 2 == parity {
                result += 1;
            }
            visited[node] = 1;
            if i < steps {
                for neighbour in map.neighbours(node) {
                    if let Tile::Empty = map[neighbour] {
                        if visited[neighbour] == 0 {
                            queue.push_back((neighbour, i + 1));
                        }
                    }
                }
            }
        }
    }

    println!("From {} for {}", start, steps);
    //println!("{}", visited);

    result
}

pub fn part_2(_input: &str) -> Solution {
    let (map, start) = parse(_input);
    let size = map.width;
    let full = size - 1;
    let half = full / 2;
    let n = (26501365 - half) / size;

    //println!("{} {} {}", size, full, half);

    let mut result: usize = 0;
    result += (extrapolate_squares(n as f64) as usize) * solve(&map, start, 200);

    for point in [
        Point::new(0, 0),
        Point::new(full, 0),
        Point::new(0, full),
        Point::new(full, full),
    ] {
        result += (n - 1) * solve(&map, point, half + full);
        result += n * solve(&map, point, half - 1);
    }

    for point in [
        Point::new(0, half),
        Point::new(full, half),
        Point::new(half, 0),
        Point::new(half, full),
    ] {
        result += solve(&map, point, full);
    }

    result.into()
}
//532 053 855 213
//532 053 855 213
//532 023 510 213
fn extrapolate_squares(x: f64) -> f64 {
    let f = vec![(1.0, 1.0), (2.0, 5.0), (3.0, 13.0)];
    let n = f.len();
    let mut result = 0.0;
    for i in 0..n {
        let mut term = f[i].1;
        for j in 0..n {
            if i != j {
                term *= (x - f[j].0) / (f[i].0 - f[j].0);
            }
        }
        result += term;
    }

    result
}

fn parse(input: &str) -> (PointVec2d<Tile>, Point) {
    let mut height = 1;
    let (mut x, mut y) = (0, 0);
    let mut start: Point = Point { x: 0, y: 0 };
    let vec: Vec<Tile> = input
        .bytes()
        .filter_map(|byte| match byte {
            b'.' => {
                x += 1;
                Some(Tile::Empty)
            }
            b'#' => {
                x += 1;
                Some(Tile::Rock)
            }
            b'S' => {
                start.x = x;
                start.y = y;
                x += 1;
                Some(Tile::Empty)
            }
            b'\n' => {
                y += 1;
                x = 0;
                height += 1;
                None
            }
            _ => None,
        })
        .collect();

    (PointVec2d::from_vec(vec, height), start)
}

fn parse_big(input: &str) -> (PointVec2d<Tile>, Point) {
    let (map, _) = parse(input);

    let mut vec = Vec::new();
    for y in 0..map.height {
        for _ in 0..5 {
            vec.append(&mut map.row(y).iter().map(|t| *t).collect());
        }
    }

    let mut map_vec = Vec::new();
    for _ in 0..5 {
        map_vec.append(&mut vec.clone());
    }

    let height = map.height * 5;
    let start = Point::new(height / 2, height / 2);

    (PointVec2d::from_vec(map_vec, height), start)
}

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Rock,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Rock => write!(f, "#"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 6, 16)]
    #[test_case(_INPUT, 64, 3773)]
    fn part_1_test(input: &str, steps: usize, expected: usize) {
        let (map, start) = parse(input);
        assert_eq!(solve(&map, start, steps), expected.into());
    }

    #[test_case(_INPUT, 625628021226274)]
    fn part_2_test(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test_case(4.0, 25.0)]
    fn extrapolate_squares_tests(x: f64, expected: f64) {
        assert_eq!(extrapolate_squares(x), expected);
    }

    #[test_case(_INPUT, Point::new(0, 130), 64)]
    #[test_case(_INPUT, Point::new(0, 130), 195)]
    #[test_case(_INPUT, Point::new(0, 65), 130)]
    #[test_case(_INPUT, Point::new(0, 0), 64)]
    fn print(input: &str, start: Point, steps: usize) {
        let (map, _) = parse(input);
        solve(&map, start, steps);
    }
}
