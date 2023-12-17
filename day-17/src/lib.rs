use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

use shared::{
    parse::ToDigit,
    point_vec2d::{Direction, Point, PointVec2d},
    *,
};
extern crate shared;

pub const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let map = parse(_input);
    solve(&map, 1, 3)
}

fn solve(map: &PointVec2d<u8>, min: usize, max: usize) -> Solution {
    let mut heap: BinaryHeap<Path> = BinaryHeap::new();

    for d in 0..2 {
        let mut cost: usize = 0;
        for i in 1..min {
            let current = match d {
                0 => Point::new(i, 0),
                _ => Point::new(0, i),
            };
            cost += map[current] as usize;
        }

        for i in min..=max {
            let (current, direction) = match d {
                0 => (Point::new(i, 0), Direction::East),
                _ => (Point::new(0, i), Direction::South),
            };

            cost += map[current] as usize;
            let heurestic = map.height - 1 + map.width - 1 - current.x - current.y;

            heap.push(Path {
                cost,
                estimate: cost + heurestic,
                state: State { current, direction },
            })
        }
    }

    let v = vec![usize::MAX; map.width * map.height];
    let mut visited = [
        PointVec2d::from_vec(v.clone(), map.height),
        PointVec2d::from_vec(v, map.height),
    ];

    while let Some(path) = heap.pop() {
        if path.state.current.x == map.width - 1 && path.state.current.y == map.height - 1 {
            return path.cost.into();
        }

        for next in path.next_paths(&map, min, max) {
            let d = if next.state.direction == Direction::South {
                0
            } else {
                1
            };

            let last_cost = visited[d][next.state.current];
            if last_cost > next.cost {
                visited[d][next.state.current] = next.cost;
                heap.push(next);
            }
        }
    }

    Solution::None
}

#[derive(PartialEq, Eq, Debug)]
struct Path {
    state: State,
    cost: usize,
    estimate: usize,
}

impl Path {
    fn next_paths(&self, map: &PointVec2d<u8>, min: usize, max: usize) -> Vec<Path> {
        let mut vec = Vec::new();
        for d in [
            [Direction::East, Direction::North],
            [Direction::West, Direction::South],
        ] {
            let mut cost: usize = self.cost;
            let mut current = self.state.current;
            let mut direction: Direction;
            for _ in 1..min {
                if self.state.direction == Direction::South {
                    if let Some(next) = map.go(current, d[0]) {
                        current = next;
                    } else {
                        break;
                    }
                } else {
                    if let Some(next) = map.go(current, d[1]) {
                        current = next;
                    } else {
                        break;
                    }
                }
                cost += map[current] as usize;
            }

            for _ in min..=max {
                if self.state.direction == Direction::South {
                    if let Some(next) = map.go(current, d[0]) {
                        direction = Direction::East;
                        current = next;
                    } else {
                        break;
                    }
                } else {
                    if let Some(next) = map.go(current, d[1]) {
                        direction = Direction::South;
                        current = next;
                    } else {
                        break;
                    }
                }

                cost += map[current] as usize;
                let heurestic = map.height - 1 + map.width - 1 - current.x - current.y;
                vec.push(Path {
                    cost,
                    estimate: cost + heurestic,
                    state: State { current, direction },
                })
            }
        }
        vec
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimate.cmp(&self.estimate)
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.estimate.partial_cmp(&self.estimate)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct State {
    current: Point,
    direction: Direction,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u8(self.current.x as u8);
        state.write_u8(self.current.y as u8);
        state.write_u8(self.direction as u8);
    }
}

fn parse(input: &str) -> PointVec2d<u8> {
    PointVec2d::from_vec(
        input.bytes().filter_map(|b| b.to_digit()).collect(),
        input.lines().count(),
    )
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(_TEST, 102)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), 668usize.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    let map = parse(_input);
    solve(&map, 4, 10)
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(_TEST, 94)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), 788usize.into());
    }
}
