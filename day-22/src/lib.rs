use shared::{parse::Parsable, point_vec2d::Point, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (mut layers, mut bricks) = parse(_input);
    collapse_bricks(&mut layers, &mut bricks);
    let mut result: usize = 0;
    // dbg!(&layers);
    // dbg!(&bricks);

    for i in 0..bricks.len() {
        let brick = &bricks[i];
        if brick
            .bricks_over
            .iter()
            .all(|u| bricks[*u].bricks_under.len() > 1)
        {
            // println!("{}", i);
            result += 1;
        }
    }

    result.into()
}

fn collapse_bricks(layers: &mut Vec<Vec<usize>>, bricks: &mut Vec<Brick>) {
    for z in 2..layers.len() {
        // println!("Z: {}", z);
        let mut offset = 0;
        for i in 0..layers[z].len() {
            let index = i - offset;
            let current = layers[z][index];
            // println!("id: {}, offset: {}", current, offset);
            for z_test in (0..z).rev() {
                // println!("z_test: {}", z_test);
                let bricks_under: Vec<usize> = layers[z_test]
                    .iter()
                    .filter_map(|test_brick_id| {
                        if bricks[*test_brick_id].intersects(&bricks[current]) {
                            Some(*test_brick_id)
                        } else {
                            None
                        }
                    })
                    .collect();

                if bricks_under.len() > 0 || z_test == 0 {
                    // dbg!(&bricks_under);
                    let z_dest = z_test + 1;
                    if z_dest != z {
                        let brick = layers[z].remove(index);
                        layers[z_dest].push(brick);
                        offset += 1;
                    }

                    if !(bricks_under.len() == 1 && bricks_under[0] == current) {
                        for u in bricks_under.iter() {
                            bricks[*u].bricks_over.push(current);
                        }

                        let brick = &mut bricks[current];
                        brick.bricks_under = bricks_under;
                    }

                    break;
                }
            }
        }
    }
}

pub fn part_2(_input: &str) -> Solution {
    let (mut layers, mut bricks) = parse(_input);
    collapse_bricks(&mut layers, &mut bricks);

    let mut result: usize = 0;

    for i in 0..bricks.len() {
        let mut v = remove_layer(&bricks, vec![i]);
        v.sort_unstable();
        v.dedup();
        println!("{}", v.len());
        result += v.len();
    }

    result.into()
}

fn remove_layer(bricks: &Vec<Brick>, previous_layer_removed: Vec<usize>) -> Vec<usize> {
    let mut next_layer_removed: Vec<usize> = previous_layer_removed
        .iter()
        .map(|f| {
            bricks[*f].bricks_over.iter().filter_map(|b| {
                if bricks[*b]
                    .bricks_under
                    .iter()
                    .all(|u| previous_layer_removed.contains(u))
                {
                    Some(*b)
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect();

    if next_layer_removed.len() == 0 {
        return Vec::new();
    }

    next_layer_removed.sort_unstable();
    next_layer_removed.dedup();
    next_layer_removed.append(&mut remove_layer(bricks, next_layer_removed.clone()));
    next_layer_removed
}

fn parse(input: &str) -> (Vec<Vec<usize>>, Vec<Brick>) {
    let mut bytes = input.bytes();
    let mut layers = Vec::new();
    let mut bricks = Vec::new();
    for _ in 0..400 {
        layers.push(Vec::new());
    }

    let mut i = 0;

    while let Some(from_x) = bytes.next_number() {
        let from_y = bytes.next_number().unwrap();
        let from_z: usize = bytes.next_number().unwrap();
        let to_x = bytes.next_number().unwrap();
        let to_y = bytes.next_number().unwrap();
        let to_z: usize = bytes.next_number().unwrap();

        let brick = Brick {
            start: Point::new(from_x, from_y),
            end: Point::new(to_x, to_y),
            bricks_under: Vec::new(),
            bricks_over: Vec::new(),
        };
        bricks.push(brick);

        for z in from_z..=to_z {
            layers[z].push(i);
        }

        i += 1;
    }
    (layers, bricks)
}

#[derive(Debug)]
struct Brick {
    start: Point,
    end: Point,
    bricks_under: Vec<usize>,
    bricks_over: Vec<usize>,
}

impl Brick {
    fn intersects(&self, other: &Self) -> bool {
        self.start.x.max(other.start.x) <= self.end.x.min(other.end.x)
            && self.start.y.max(other.start.y) <= self.end.y.min(other.end.y)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 5)]
    #[test_case(_INPUT, 5)]
    fn test_1(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test_case(include_str!("_test.txt"), 7)]
    #[test_case(_INPUT, 89727)]
    fn test_2(input: &str, expected: usize) {
        assert_eq!(part_2(input), expected.into());
    }
}
