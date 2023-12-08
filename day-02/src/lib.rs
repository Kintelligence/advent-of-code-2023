use shared::{parse::Parsable, *};
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut bytes = line.bytes();
            let _: Option<u32> = bytes.next_number();

            while let Some(value) = bytes.next_number() {
                let value: u32 = value;
                match bytes.next().unwrap() {
                    b'r' if value > 12 => return 0,
                    b'g' if value > 13 => return 0,
                    b'b' if value > 14 => return 0,
                    _ => {}
                }
            }
            id + 1
        })
        .sum::<usize>()
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let (mut red, mut green, mut blue) = (0, 0, 0);

            let mut bytes = line.bytes();
            let _: Option<u32> = bytes.next_number();

            while let Some(value) = bytes.next_number() {
                match bytes.next().unwrap() {
                    b'r' if red < value => red = value,
                    b'g' if green < value => green = value,
                    b'b' if blue < value => blue = value,
                    _ => {}
                }
            }

            red * green * blue
        })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(_TEST), Solution::Usize(8))
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(2348))
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(_TEST), Solution::U32(2286))
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(_INPUT), Solution::U32(76008))
    }
}
