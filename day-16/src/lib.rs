use shared::{
    point_vec2d::{Direction, Point, PointVec2d},
    *,
};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let map = parse(_input);
    solve(&map, (Point::new(0, 0), Direction::East)).into()
}

fn solve(map: &PointVec2d<Tile>, start: (Point, Direction)) -> usize {
    let mut visited =
        vec![PointVec2d::from_vec(vec![false; map.height * map.width], map.height); 5];
    let mut queue = Vec::new();
    queue.push(start);

    while let Some((mut point, mut direction)) = queue.pop() {
        loop {
            if visited[direction as usize][point] {
                break;
            }

            visited[4][point] = true;
            visited[direction as usize][point] = true;

            if let Tile::Mirror(mirror) = map[point] {
                if mirror {
                    direction = match direction {
                        Direction::North => Direction::East,
                        Direction::East => Direction::North,
                        Direction::South => Direction::West,
                        Direction::West => Direction::South,
                    }
                } else {
                    direction = match direction {
                        Direction::North => Direction::West,
                        Direction::East => Direction::South,
                        Direction::South => Direction::East,
                        Direction::West => Direction::North,
                    }
                }
            } else if let Tile::Splitter(split) = map[point] {
                if !split && (direction == Direction::East || direction == Direction::West) {
                    direction = Direction::North;
                    queue.push((point, Direction::South));
                } else if split && (direction == Direction::South || direction == Direction::North)
                {
                    direction = Direction::East;
                    queue.push((point, Direction::West));
                }
            }

            if let Some(next) = map.go(point, direction) {
                point = next;
            } else {
                break;
            }
        }
    }

    visited[4].vec.iter().filter(|n| **n).count().into()
}

enum Tile {
    Empty,
    Mirror(bool),
    Splitter(bool),
}

fn parse(input: &str) -> PointVec2d<Tile> {
    let vec = input
        .bytes()
        .filter_map(|byte| match byte {
            b'\\' => Some(Tile::Mirror(false)),
            b'/' => Some(Tile::Mirror(true)),
            b'|' => Some(Tile::Splitter(false)),
            b'-' => Some(Tile::Splitter(true)),
            b'.' => Some(Tile::Empty),
            _ => None,
        })
        .collect();

    PointVec2d::from_vec(vec, input.lines().count())
}

pub fn part_2(_input: &str) -> Solution {
    let map = parse(_input);

    let mut max: usize = 0;

    for i in 0..map.height {
        max = max.max(solve(&map, (Point::new(0, i), Direction::East)));
        max = max.max(solve(&map, (Point::new(map.width - 1, i), Direction::West)));
    }

    for i in 0..map.width {
        max = max.max(solve(&map, (Point::new(i, 0), Direction::South)));
        max = max.max(solve(
            &map,
            (Point::new(i, map.height - 1), Direction::North),
        ));
    }

    max.into()
}

fn rec_solve(
    map: &PointVec2d<Tile>,
    calculated: &mut Vec<PointVec2d<usize>>,
    point: Point,
    direction: Direction,
) -> usize {
    let mut value = &mut calculated[direction as usize][point];
    if *value != 0 {
        return *value;
    }

    let mut next_direction = direction;
    let mut next_split: Option<Direction> = None;

    if let Tile::Mirror(mirror) = map[point] {
        if mirror {
            next_direction = match direction {
                Direction::North => Direction::East,
                Direction::East => Direction::North,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
            }
        } else {
            next_direction = match direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
            }
        }
    } else if let Tile::Splitter(split) = map[point] {
        if !split && (direction == Direction::East || direction == Direction::West) {
            next_direction = Direction::North;
            next_split = Some(Direction::South);
        } else if split && (direction == Direction::South || direction == Direction::North) {
            next_direction = Direction::East;
            next_split = Some(Direction::West);
        }
    }

    *value += 1;

    todo!()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        46
    )]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), 7498usize.into());
    }
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(
        r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....",
        51
    )]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), 7498usize.into());
    }
}
