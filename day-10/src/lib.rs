use map::{parse, Connection, Connection::*};
use shared::{
    point_vec2d::{Direction, Point, PointVec2d, DIRECTIONS},
    *,
};

use crate::map::{next_direction, Tile};
extern crate shared;
mod map;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (start, map): (Point, PointVec2d<Tile>) = parse(_input);
    let mut steps: usize = 0;
    let mut current = start;
    let mut heading: Direction = Direction::North;

    for direction in DIRECTIONS {
        if let Some(next) = map.go(start, direction) {
            if let Tile::Pipe(connection) = map.index(next) {
                if let Some(_) = next_direction(*connection, direction) {
                    heading = direction;
                    break;
                }
            }
        }
    }

    loop {
        if let Some(next) = map.go(current, heading) {
            if let Tile::Pipe(connection) = map.index(next) {
                if let Some(next_heading) = next_direction(*connection, heading) {
                    heading = next_heading;
                    current = next;
                    steps += 1;
                    continue;
                }
            } else if let Tile::Start = map.index(next) {
                steps += 1;
                break;
            }
        }

        panic!("oh no");
    }

    (steps / 2).into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;

    #[test]
    fn example_input() {
        assert_eq!(
            part_1(
                ".....
            .S-7.
            .|.|.
            .L-J.
            .....",
            ),
            Solution::Usize(4)
        );
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(4));
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (start, map): (Point, PointVec2d<Tile>) = parse(_input);
    let mut current = start;
    let mut heading: Direction = Direction::North;
    let mut start_connection: Connection = Connection::NS;
    let mut found = false;

    for direction in DIRECTIONS {
        if let Some(next) = map.go(start, direction) {
            if let Tile::Pipe(connection) = map.index(next) {
                if let Some(_) = next_direction(*connection, direction) {
                    if found {
                        start_connection = Connection::from_directions(heading, direction);
                    } else {
                        heading = direction;
                        found = true;
                    }
                }
            }
        }
    }

    let mut visited = PointVec2d::from_vec(vec![Tile::Empty; map.width * map.height], map.height);
    *visited.index_mut(start) = Tile::Pipe(start_connection);

    loop {
        if let Some(next) = map.go(current, heading) {
            if let Tile::Pipe(connection) = map.index(next) {
                *visited.index_mut(next) = Tile::Pipe(*connection);
                if let Some(next_heading) = next_direction(*connection, heading) {
                    heading = next_heading;
                    current = next;
                    continue;
                }
            } else if let Tile::Start = map.index(next) {
                break;
            }
        }

        panic!("oh no");
    }

    let mut area: usize = 0;

    for i in 0..visited.height {
        let mut inside_loop = false;

        for tile in visited.row(i).iter() {
            match tile {
                Tile::Start => panic!("Should not have a start"),
                Tile::Pipe(connection) => {
                    if *connection == NS || *connection == NE || *connection == NW {
                        inside_loop = !inside_loop;
                    }
                }
                Tile::Empty => {
                    if inside_loop {
                        area += 1;
                    }
                }
            }
        }
    }

    area.into()
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;

    #[test]
    fn example_input_1() {
        assert_eq!(
            part_2(
                "...........
                .S-------7.
                .|F-----7|.
                .||.....||.
                .||.....||.
                .|L-7.F-J|.
                .|..|.|..|.
                .L--J.L--J.
                ...........",
            ),
            Solution::Usize(4)
        );
    }

    #[test]
    fn example_input_2() {
        assert_eq!(
            part_2(
                ".F----7F7F7F7F-7....
                .|F--7||||||||FJ....
                .||.FJ||||||||L7....
                FJL7L7LJLJ||LJ.L-7..
                L--J.L7...LJS7F-7L7.
                ....F-J..F7FJ|L7L7L7
                ....L7.F7||L7|.L7L7|
                .....|FJLJ|FJ|F7|.LJ
                ....FJL-7.||.||||...
                ....L---J.LJ.LJLJ...",
            ),
            Solution::Usize(8)
        );
    }

    #[test]
    fn example_input_3() {
        assert_eq!(
            part_2(
                "FF7FSF7F7F7F7F7F---7
                L|LJ||||||||||||F--J
                FL-7LJLJ||||||LJL-77
                F--JF--7||LJLJ7F7FJ-
                L---JF-JLJ.||-FJLJJ7
                |F|F-JF---7F7-L7L|7|
                |FFJF7L7F-JF7|JL---7
                7-L-JL7||F7|L7F-7F7|
                L.L7LFJ|||||FJL7||LJ
                L7JLJL-JLJLJL--JLJ.L",
            ),
            Solution::Usize(10)
        );
    }

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(433));
    }
}
