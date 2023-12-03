use shared::*;
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1() -> Solution {
    let mut previous: [bool; 140] = [false; 140];
    let mut remaining: Vec<(usize, usize, u32)> = Vec::new();
    let mut sum: u32 = 0;

    for line in _INPUT.lines() {
        let mut next_remaining: Vec<(usize, usize, u32)> = Vec::new();
        let mut current: [bool; 140] = [false; 140];
        let mut value: u32 = 0;
        let mut is_adjacent: bool = false;
        let mut i: usize = 0;

        for char in line.chars() {
            if char != '.' {
                if char.is_ascii_digit() {
                    value *= 10;
                    value += char.to_digit(10).unwrap();

                    if i > 0 && previous[i - 1] {
                        is_adjacent = true;
                    } else if previous[i] {
                        is_adjacent = true;
                    } else if i < 139 && previous[i + 1] {
                        is_adjacent = true;
                    }
                } else {
                    is_adjacent = true;
                    current[i] = true;

                    if value != 0 {
                        sum += value;
                        value = 0;
                    }

                    remaining.retain(|(start, end, val)| {
                        if i >= *start && i <= *end {
                            sum += val;
                            return false;
                        } else if i > *end {
                            return false;
                        }
                        return true;
                    });
                }
            } else {
                if is_adjacent {
                    if value != 0 {
                        sum += value;
                    }

                    is_adjacent = false;
                } else if value != 0 {
                    next_remaining.push(create_remainder(value, i));
                }

                value = 0;
            }

            i += 1;
        }

        if is_adjacent {
            if value != 0 {
                sum += value;
            }
        } else if value != 0 {
            next_remaining.push(create_remainder(value, i));
        }

        previous = current;
        remaining = next_remaining;
    }

    sum.into()
}

fn create_remainder(value: u32, i: usize) -> (usize, usize, u32) {
    let mut test: u32 = 1;
    let mut digits: usize = 0;
    while test <= value {
        test *= 10;
        digits += 1;
    }

    let start = (i - (digits - 1)).saturating_sub(2);
    (start, i, value)
}

pub const ADJ_EIGHT: [(isize, isize); 8] = [
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
];

const HEIGHT: usize = 140;
const WIDTH: usize = 141;
pub fn part_2() -> Solution {
    let mut sum: u32 = 0;

    let map: Vec<char> = _INPUT.chars().collect();

    for y in 0..HEIGHT {
        for x in 0..WIDTH - 1 {
            if map[y * WIDTH + x] == '*' {
                if let Some(gear) = calculate_gear_ratio(x, y, &map) {
                    sum += gear;
                }
            }
        }
    }

    sum.into()
}

fn calculate_gear_ratio(x: usize, y: usize, map: &Vec<char>) -> Option<u32> {
    let mut count = 0;
    let mut sum = 1;

    for adj in ADJ_EIGHT {
        let nx = x as isize + adj.0;
        let ny = y as isize + adj.1;

        if nx < 0 || nx >= WIDTH as isize || ny < 0 || ny >= HEIGHT as isize {
            continue;
        }

        let ny = ny as usize;
        let nx = nx as usize;

        if y != ny && nx > x.saturating_sub(1) && map[ny * WIDTH + nx - 1].is_ascii_digit() {
            continue;
        }

        if map[ny * WIDTH + nx].is_ascii_digit() {
            if count == 2 {
                return None;
            }

            sum *= expand(nx, ny, &map);
            count += 1;
        }
    }

    (count == 2).then(|| sum)
}

fn expand(x: usize, y: usize, map: &Vec<char>) -> u32 {
    let mut start = x;
    loop {
        if start > 0 && map[y * WIDTH + start - 1].is_ascii_digit() {
            start -= 1;
        } else {
            break;
        }
    }

    let mut value = 0;
    for i in start..WIDTH - 1 {
        if let Some(digit) = map[y * WIDTH + i].to_digit(10) {
            value = value * 10 + digit;
        } else {
            break;
        }
    }

    value
}
