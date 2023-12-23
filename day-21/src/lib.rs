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

    result
}

pub fn part_2(_input: &str) -> Solution {
    let (map, start) = parse(_input);
    let size = map.width;
    let full = size - 1;
    let half = full / 2;
    let n = (26501365 - half) / size;

    let reg: usize;
    let off: usize;
    if n % 2 == 1 {
        reg = n * n;
        off = (n - 1) * (n - 1);
    } else {
        reg = (n - 1) * (n - 1);
        off = n * n;
    }

    let mut result: usize = 0;
    // We go 201 steps for the regular squares so the parity fits
    result += reg * solve(&map, start, 201);
    result += off * solve(&map, start, 200);

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

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Rock,
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
}
