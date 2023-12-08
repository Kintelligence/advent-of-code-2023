use shared::{parse::Parsable, *};
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let mut lines = _input.lines();
    let mut times = lines.next().expect("lines has times").bytes();
    let mut distances = lines.next().expect("lines has distances").bytes();
    let races = parse_races(&mut times, &mut distances);

    races
        .iter()
        .map(|race| race.solutions())
        .fold(1, |acc, val| acc * val)
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    let mut lines = _input.lines();
    let mut times = lines
        .next()
        .expect("lines has times")
        .bytes()
        .filter(|char| char.is_ascii_digit());
    let mut distances = lines
        .next()
        .expect("lines has distances")
        .bytes()
        .filter(|char| char.is_ascii_digit());
    let races = parse_races(&mut times, &mut distances);

    races
        .iter()
        .map(|race| race.solutions())
        .fold(1, |acc, val| acc * val)
        .into()
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn solutions(&self) -> u64 {
        let d = (self.time * self.time - 4 * (self.distance + 1)) as f64;
        let d = d.sqrt();
        let mut low = (self.time as f64 - d) / 2.0;
        let mut high = (self.time as f64 + d) / 2.0;

        if low.fract() > f64::EPSILON {
            low = low.ceil();
        }

        if high.fract() < f64::EPSILON {
            high = high.floor();
        }

        (high - low) as u64 + 1
    }
}

fn parse_races<A, B>(times: &mut A, distances: &mut B) -> Vec<Race>
where
    A: Iterator<Item = u8>,
    B: Iterator<Item = u8>,
{
    let mut races = Vec::new();

    while let Some(time) = times.next_number() {
        if let Some(distance) = distances.next_number() {
            races.push(Race { time, distance });
        }
    }

    races
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(_TEST), Solution::U64(288))
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::U64(275724))
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(_TEST), Solution::U64(71503))
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(_INPUT), Solution::U64(37286485))
    }
}
