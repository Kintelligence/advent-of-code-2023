use shared::*;
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
const _INPUT: &'static str = include_str!("_input.txt");
const _MANUAL: &'static str = include_str!("_manual.txt");
const _PANDA: &'static str = include_str!("_panda.txt");
const _LONG: &'static str = include_str!("_long.txt");

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
                if char.is_numeric() {
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
                    let mut test: u32 = 1;
                    let mut digits: usize = 0;
                    while test <= value {
                        test *= 10;
                        digits += 1;
                    }

                    let start = (i - (digits - 1)).saturating_sub(2);
                    next_remaining.push((start, i, value));
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
            let mut test: u32 = 1;
            let mut digits: usize = 0;
            while test <= value {
                test *= 10;
                digits += 1;
            }

            let start = (i - (digits - 1)).saturating_sub(2);
            next_remaining.push((start, i, value));
        }

        previous = current;
        remaining = next_remaining;
    }

    sum.into()
}

pub fn part_2() -> Solution {
    let data: Vec<Vec<char>> = _INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut sum: u32 = 0;
    let height = data.len();
    let width = data[0].len();

    let mut y: usize = 0;
    for _ in 0..height {
        let mut x: usize = 0;

        for _ in 0..width {
            if data[y][x] == '*' {
                if let Some(gear) = find_gear(y, x, &data, height, width) {
                    sum += gear;
                }
            }

            x += 1;
        }

        y += 1;
    }

    sum.into()
}

fn find_gear(
    y: usize,
    x: usize,
    data: &Vec<Vec<char>>,
    height: usize,
    width: usize,
) -> Option<u32> {
    let mut first: Option<u32> = None;
    let mut second: Option<u32> = None;

    let x_min = 0.max(x - 1);

    for n_y in 0.max(y - 1)..=height.min(y + 1) {
        for n_x in 0.max(x - 1)..=width.min(x + 1) {
            if data[n_y][n_x].is_numeric() {
                if n_x > x_min && data[n_y][n_x - 1].is_numeric() {
                    continue;
                }

                if first.is_none() {
                    first = Some(get_number_around(n_y, n_x, &data));
                } else if second.is_none() {
                    second = Some(get_number_around(n_y, n_x, &data));
                } else {
                    return None;
                }
            }
        }
    }

    if let Some(a) = first {
        if let Some(b) = second {
            return Some(a * b);
        }
    }

    return None;
}

fn get_number_around(y: usize, x: usize, data: &Vec<Vec<char>>) -> u32 {
    let mut start = x;
    while start > 0 && data[y][start - 1].is_numeric() {
        start -= 1;
    }

    let mut end = x;
    while end < data[0].len() - 1 && data[y][end + 1].is_numeric() {
        end += 1;
    }

    let mut value = 0;

    for x in start..=end {
        value *= 10;
        value += data[y][x].to_digit(10).unwrap();
    }

    value
}
