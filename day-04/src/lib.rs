use shared::{parse::Parsable, *};
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|card| {
            let mut dupe: u128 = 0;
            let mut c = 1;

            if let Some((_, numbers)) = card.split_once(':') {
                let mut bytes = numbers.bytes();

                while let Some(number) = bytes.next_number() {
                    let number: u32 = number;
                    if 1 << number & dupe > 0 {
                        c *= 2;
                    } else {
                        dupe |= 1 << number;
                    }
                }
            }

            c >> 1
        })
        .sum::<u32>()
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    let mut card_count: [u32; 250] = [1; 250];
    let mut score = 0;

    let mut i = 0;
    _input.lines().for_each(|card| {
        let count = card_count[i];
        let mut dupe: u128 = 0;
        let mut c = 0;

        if let Some((_, numbers)) = card.split_once(':') {
            let mut bytes = numbers.bytes();

            while let Some(number) = bytes.next_number() {
                let number: u32 = number;
                if 1 << number & dupe > 0 {
                    c += 1;
                    card_count[i + c] += count;
                } else {
                    dupe |= 1 << number;
                }
            }
        }

        i += 1;
        score += count;
    });

    score.into()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(_TEST), Solution::U32(13))
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::U32(25174))
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(_TEST), Solution::U32(30))
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(_INPUT), Solution::U32(6420979))
    }
}
