use shared::*;
extern crate shared;

pub const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            for ch in line.chars() {
                if ch.is_ascii_digit() {
                    first = ch.to_digit(10).unwrap();
                    break;
                }
            }

            for ch in line.chars().rev() {
                if ch.is_ascii_digit() {
                    last = ch.to_digit(10).unwrap();
                    break;
                }
            }

            first * 10 + last
        })
        .sum::<u32>()
        .into()
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| {
            let mut first = 0;
            let mut last = 0;

            let mut chars = line.chars();
            loop {
                if let Some(o) = parse_digit_start(chars.as_str()) {
                    first = o;
                    break;
                }

                if let Some(ch) = chars.next() {
                    if ch.is_ascii_digit() {
                        first = ch.to_digit(10).unwrap();
                        break;
                    }
                } else {
                    break;
                }
            }

            chars = line.chars();
            loop {
                if let Some(o) = parse_digit_end(chars.as_str()) {
                    last = o;
                    break;
                }

                if let Some(ch) = chars.next_back() {
                    if ch.is_ascii_digit() {
                        last = ch.to_digit(10).unwrap();
                        break;
                    }
                } else {
                    break;
                }
            }

            first * 10 + last
        })
        .sum::<u32>()
        .into()
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_digit_end(str: &str) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if str.ends_with(n) {
            return Some(i as u32 + 1);
        }
    }
    None
}

fn parse_digit_start(str: &str) -> Option<u32> {
    for (i, n) in NUMBERS.iter().enumerate() {
        if str.starts_with(n) {
            return Some(i as u32 + 1);
        }
    }
    None
}
