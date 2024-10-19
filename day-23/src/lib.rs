use shared::{
    point_vec2d::{Direction, Direction::*, Neighbours, Point, PointVec2d},
    *,
};
use Tile::*;

extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let map = parse(_input);
    let start = Point::new(1, 0);
    let goal = Point::new(map.width - 2, map.height - 1);

    traverse(
        &map,
        &mut PointVec2d::from_vec(vec![false; map.width * map.height], map.height),
        0,
        start,
        &goal,
    )
        .into()
}

fn traverse(
    map: &PointVec2d<Tile>,
    visited: &mut PointVec2d<bool>,
    cost: usize,
    point: Point,
    goal: &Point,
) -> usize {
    let mut point = point;
    let mut cost = cost;
    loop {
        if visited[point] {
            return 0;
        }

        visited[point] = true;

        if point == *goal {
            return cost;
        }

        if let Slope(direction) = map[point] {
            return traverse(
                map,
                visited,
                cost + 1,
                map.go(point, direction).unwrap(),
                goal,
            );
        }

        let neighbours: Vec<Point> = map
            .neighbours(point)
            .filter(|n| !visited[*n] && map[*n] != Forest)
            .collect();

        if neighbours.len() > 1 {
            return neighbours
                .iter()
                .skip(1)
                .map(|n| traverse(map, &mut visited.clone(), cost + 1, *n, goal))
                .max()
                .unwrap()
                .max(traverse(
                    map,
                    visited,
                    cost + 1,
                    *neighbours.first().unwrap(),
                    goal,
                ));
        }

        if neighbours.len() == 1 {
            point = *neighbours.last().unwrap();
            cost += 1;
            continue;
        }

        return 0;
    }
}

pub fn part_2(_input: &str) -> Solution {
    let map = parse(_input);
    let start = Point::new(1, 0);
    let goal = Point::new(map.width - 2, map.height - 1);

    traverse2(
        &map,
        &mut PointVec2d::from_vec(vec![false; map.width * map.height], map.height),
        0,
        start,
        &goal,
    )
        .into()
}

fn traverse2(
    map: &PointVec2d<Tile>,
    visited: &mut PointVec2d<bool>,
    cost: usize,
    point: Point,
    goal: &Point,
) -> usize {
    let mut point = point;
    let mut cost = cost;
    loop {
        if visited[point] {
            return 0;
        }

        visited[point] = true;

        if point == *goal {
            return cost;
        }

        let neighbours: Vec<Point> = map
            .neighbours(point)
            .filter(|n| !visited[*n] && map[*n] != Forest)
            .collect();

        if neighbours.len() > 1 {
            return neighbours
                .iter()
                .skip(1)
                .map(|n| traverse2(map, &mut visited.clone(), cost + 1, *n, goal))
                .max()
                .unwrap()
                .max(traverse2(
                    map,
                    visited,
                    cost + 1,
                    *neighbours.first().unwrap(),
                    goal,
                ));
        } else if neighbours.len() == 1 {
            point = *neighbours.last().unwrap();
            cost += 1;
            continue;
        }

        return 0;
    }
}

fn parse(input: &str) -> PointVec2d<Tile> {
    let mut height = 1;
    let vec = input
        .bytes()
        .filter_map(|byte| match byte {
            b'.' => Some(Path),
            b'#' => Some(Forest),
            b'v' => Some(Slope(South)),
            b'>' => Some(Slope(East)),
            b'\n' => {
                height += 1;
                None
            }
            _ => None,
        })
        .collect();
    PointVec2d::from_vec(vec, height)
}

#[derive(PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 94; "Example")]
    #[test_case(_INPUT, 2414; "Real")]
    fn part_1_tests(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(include_str!("_test.txt"), 154; "Example")]
    #[test_case(_INPUT, 94; "Real")]
    fn part_2_tests(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }
}
