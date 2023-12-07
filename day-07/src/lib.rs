use shared::{parse::parse_u32, *};
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
        let mut chars = line.chars();
        let mut strength: u32 = 0;

        let mut cards: [u32; 13] = [0; 13];
        for i in 0..5 {
            if let Some(card) = chars.next() {
                let value = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 9,
                    'T' => 8,
                    num => num.to_digit(10).unwrap() as u32 - 2,
                };
                strength |= value << ((4 - i) * 4);
                cards[value as usize] += 1;
            }
        }

        cards.sort_unstable();
        strength |= calculate_type(cards[12] + jokers, cards[11]) << 20;

        let bid = parse_u32(&mut chars).expect("expected bid");

        Hand { bid, strength }
    }

    fn new_with_jokers(line: &str) -> Self {
        let mut chars = line.chars();
        let mut strength: u32 = 0;
        let mut jokers = 0;

        let mut cards: [u32; 13] = [0; 13];
        for i in 0..5 {
            if let Some(card) = chars.next() {
                let value = match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'J' => 0,
                    'T' => 9,
                    num => num.to_digit(10).unwrap() as u32 - 1,
                };

                if value == 0 {
                    jokers += 1;
                } else {
                    cards[value as usize] += 1;
                }

                strength |= value << ((4 - i) * 4);
            }
        }

        cards.sort_unstable();
        strength |= calculate_type(cards[12] + jokers, cards[11]) << 20;

        let bid = parse_u32(&mut chars).expect("expected bid");

        Hand { bid, strength }
    }
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
