use shared::*;
extern crate shared;

pub const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .enumerate()
        .map(|(id, line)| {
            let mut split = line.split([':', ' ', ';', ',']).skip(3);

            while let Some(v) = split.next() {
                if let Ok(val) = v.parse::<u32>() {
                    match split.next().unwrap().chars().next().unwrap() {
                        'r' if val > 12 => return 0,
                        'g' if val > 13 => return 0,
                        'b' if val > 14 => return 0,
                        _ => {}
                    }
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

            let mut split = line.split([':', ' ', ';', ',']).skip(3);

            while let Some(v) = split.next() {
                if let Ok(val) = v.parse::<u32>() {
                    match split.next().unwrap().chars().next().unwrap() {
                        'r' if red < val => red = val,
                        'g' if green < val => green = val,
                        'b' if blue < val => blue = val,
                        _ => {}
                    }
                }
            }

            red * green * blue
        })
        .sum::<u32>()
        .into()
}
