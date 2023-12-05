use shared::*;
extern crate shared;

pub const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|card| {
            let mut dupe: u128 = 0;
            let mut c = 1;

            if let Some((_, numbers)) = card.split_once(':') {
                let mut chs = numbers.chars();
                let mut n = 0;
                while let Some(ch) = chs.next() {
                    if let Some(digit) = ch.to_digit(10) {
                        n = n * 10 + digit;
                    } else if n != 0 {
                        if (1 << n) & dupe > 0 {
                            c *= 2;
                        }

                        dupe |= 1 << n;
                        n = 0;
                    }
                }

                if n != 0 && ((1 << n) & dupe > 0) {
                    c *= 2;
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
            let mut chs = numbers.chars();
            let mut n = 0;
            while let Some(ch) = chs.next() {
                if let Some(digit) = ch.to_digit(10) {
                    n = n * 10 + digit;
                } else if n != 0 {
                    if (1 << n) & dupe > 0 {
                        c += 1;
                        card_count[i + c] += count;
                    }

                    dupe |= 1 << n;
                    n = 0;
                }
            }

            if n != 0 && ((1 << n) & dupe > 0) {
                c += 1;
                card_count[i + c] += count;
            }
        }

        i += 1;
        score += count;
    });

    score.into()
}
