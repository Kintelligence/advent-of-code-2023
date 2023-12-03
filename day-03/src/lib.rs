use shared::*;
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
const _INPUT: &'static str = include_str!("_input.txt");
const _MANUAL: &'static str = include_str!("_manual.txt");
const _PANDA: &'static str = include_str!("_panda.txt");
const _LONG: &'static str = include_str!("_long.txt");

pub fn part_1() -> Solution {
    let data: Vec<Vec<char>> = _INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut sum: u32 = 0;
    let height = data.len();
    let width = data[0].len();

    let mut y: usize = 0;
    for _ in 0..height {
        let mut x: usize = 0;
        let mut visited: bool = false;

        for _ in 0..width {
            if data[y][x].is_numeric() {
                if !visited {
                    let (value, start, end) = get_number_around(y, x, &data);
                    if has_symbol_around(y, start, end, &data, height, width) {
                        sum += value;
                    }
                    visited = true;
                }
            } else {
                visited = false;
            }

            x += 1;
        }

        y += 1;
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

    for n_y in y.saturating_sub(1)..=(height - 1).min(y + 1) {
        for n_x in x.saturating_sub(1)..=(width - 1).min(x + 1) {
            if data[n_y][n_x].is_numeric() {
                if n_x > x_min && data[n_y][n_x - 1].is_numeric() {
                    continue;
                }

                if first.is_none() {
                    first = Some(get_number_around(n_y, n_x, &data).0);
                } else if second.is_none() {
                    second = Some(get_number_around(n_y, n_x, &data).0);
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

fn has_symbol_around(
    y: usize,
    start: usize,
    end: usize,
    data: &Vec<Vec<char>>,
    height: usize,
    width: usize,
) -> bool {
    for n_y in y.saturating_sub(1)..=(height - 1).min(y + 1) {
        for n_x in start.saturating_sub(1)..=(width - 1).min(end + 1) {
            let char = data[n_y][n_x];
            if !char.is_numeric() && char != '.' {
                return true;
            }
        }
    }

    false
}

fn get_number_around(y: usize, x: usize, data: &Vec<Vec<char>>) -> (u32, usize, usize) {
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

    (value, start, end)
}
