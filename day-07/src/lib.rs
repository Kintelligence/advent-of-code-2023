use shared::{parse::Parsable, *};
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let mut hands: Vec<Hand> = _input.lines().map(|line| Hand::new(line)).collect();

    hands.sort_unstable_by_key(|hand| hand.strength);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    let mut hands: Vec<Hand> = _input
        .lines()
        .map(|line| Hand::new_with_jokers(line))
        .collect();

    hands.sort_unstable_by_key(|hand| hand.strength);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + ((i + 1) * hand.bid as usize))
        .into()
}

struct Hand {
    bid: u32,
    strength: u32,
}

// [22-20] [19-16] [15-12] [11-8] [7-4] [3-0]
// type    1        2       3     4     5
impl Hand {
    fn new(line: &str) -> Self {
        let mut bytes = line.bytes();
        let mut strength: u32 = 0;

        let mut counts: [u32; 13] = [0; 13];
        for i in 0..5 {
            if let Some(card) = bytes.next() {
                let value = match card {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 9,
                    b'T' => 8,
                    n => n - b'0' - 2,
                };
                strength |= (value as u32) << ((4 - i) * 4);
                counts[value as usize] += 1;
            }
        }

        let (max, sec) = find_two_highest(&counts);
        strength |= calculate_type(max, sec) << 20;

        let bid = bytes.next_number().expect("expected bid");

        Hand { bid, strength }
    }

    fn new_with_jokers(line: &str) -> Self {
        let mut bytes = line.bytes();
        let mut strength: u32 = 0;
        let mut jokers = 0;

        let mut counts: [u32; 13] = [0; 13];
        for i in 0..5 {
            if let Some(card) = bytes.next() {
                let value = match card {
                    b'A' => 12,
                    b'K' => 11,
                    b'Q' => 10,
                    b'J' => 0,
                    b'T' => 9,
                    n => n - b'0' - 1,
                };

                if value == 0 {
                    jokers += 1;
                } else {
                    counts[value as usize] += 1;
                }

                strength |= (value as u32) << ((4 - i) * 4);
            }
        }

        let (max, sec) = find_two_highest(&counts);
        strength |= calculate_type(max + jokers, sec) << 20;

        let bid = bytes.next_number().expect("expected bid");

        Hand { bid, strength }
    }
}

fn find_two_highest(counts: &[u32; 13]) -> (u32, u32) {
    let (mut max, mut sec) = (0, 0);

    for count in counts.iter() {
        if *count > max {
            sec = max;
            max = *count;
        } else if *count > sec {
            sec = *count;
        }
    }

    (max, sec)
}

fn calculate_type(most_matches: u32, second_most_matches: u32) -> u32 {
    match most_matches {
        5 => 6,
        4 => 5,
        3 if second_most_matches == 2 => 4,
        3 => 3,
        2 if second_most_matches == 2 => 2,
        2 => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn part_1_test() {
        assert_eq!(part_1(_TEST), Solution::Usize(6440))
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(247961593))
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(_TEST), Solution::Usize(5905))
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(248750699))
    }
}
