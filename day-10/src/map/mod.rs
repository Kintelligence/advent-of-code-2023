use shared::point_vec2d::{Direction, Direction::*, Point, PointVec2d};

pub fn parse(input: &str) -> (Point, PointVec2d<Tile>) {
    let height = input.lines().count();

    let mut i = 0;
    let mut found = false;
    let vec = input
        .bytes()
        .filter_map(|byte| {
            let tile = parse_tile(byte);
            if !found {
                if let Some(ref inner) = tile {
                    if let Tile::Start = inner {
                        found = true;
                    } else {
                        i += 1;
                    }
                }
            }
            tile
        })
        .collect();

    let map = PointVec2d::from_vec(vec, height);
    let start = Point::new(i % map.width, i / map.width);
    (start, map)
}

pub fn parse_tile(byte: u8) -> Option<Tile> {
    match byte {
        b'|' => Some(Tile::Pipe(Connection::NS)),
        b'-' => Some(Tile::Pipe(Connection::EW)),
        b'L' => Some(Tile::Pipe(Connection::NE)),
        b'J' => Some(Tile::Pipe(Connection::NW)),
        b'7' => Some(Tile::Pipe(Connection::SW)),
        b'F' => Some(Tile::Pipe(Connection::SE)),
        b'S' => Some(Tile::Start),
        b'.' => Some(Tile::Empty),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Start,
    Pipe(Connection),
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Connection {
    NS,
    EW,
    NE,
    NW,
    SE,
    SW,
}

impl Connection {
    pub fn from_directions(a: Direction, b: Direction) -> Self {
        if a == North || b == North {
            if a == East || b == East {
                return Self::NE;
            }
            if a == West || b == West {
                return Self::NW;
            }
            if a == South || b == South {
                return Self::NS;
            }
        }

        if a == South || b == South {
            if a == East || b == East {
                return Self::SE;
            }
            if a == West || b == West {
                return Self::SW;
            }
        }

        if a == East || b == East {
            if a == West || b == West {
                return Self::EW;
            }
        }

        panic!("No matching connection");
    }
}

#[cfg(test)]
mod parsing_tests {
    use crate::*;

    #[test]
    fn parses_correctly() {
        let input = "......
        ..F7..
        ..S|..
        ..LJ..
        ......";

        let (start, map) = parse(input);
        println!("{:?}", start);
        println!("{}", map);
    }
}

pub fn next_direction(connection: Connection, from: Direction) -> Option<Direction> {
    match connection {
        Connection::NS => {
            if let North = from {
                return Some(North);
            } else if let South = from {
                return Some(South);
            }
        }
        Connection::EW => {
            if let East = from {
                return Some(East);
            } else if let West = from {
                return Some(West);
            }
        }
        Connection::NW => {
            if let South = from {
                return Some(West);
            } else if let East = from {
                return Some(North);
            }
        }
        Connection::SE => {
            if let North = from {
                return Some(East);
            } else if let West = from {
                return Some(South);
            }
        }
        Connection::NE => {
            if let South = from {
                return Some(East);
            } else if let West = from {
                return Some(North);
            }
        }
        Connection::SW => {
            if let North = from {
                return Some(West);
            } else if let East = from {
                return Some(South);
            }
        }
    }

    None
}
