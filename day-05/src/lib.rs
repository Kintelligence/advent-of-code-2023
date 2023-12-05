use shared::parse::*;
use shared::*;
extern crate shared;

pub const _TEST: &'static str = include_str!("_test.txt");
pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let (mut values, maps) = _input
        .split_once('\n')
        .and_then(|(left, right)| Some((parse_seeds(left), parse_maps(right))))
        .unwrap();

    for map in maps {
        {
            let mut value_iter = values.iter_mut();
            let mut translation_iter = map.iter();

            let mut value = value_iter.next().unwrap();
            let mut translation = translation_iter.next().unwrap();

            loop {
                if *value > translation.end {
                    if let Some(next) = translation_iter.next() {
                        translation = next;
                        continue;
                    } else {
                        break;
                    }
                } else {
                    if *value >= translation.start {
                        *value += translation.offset;
                    }

                    if let Some(next) = value_iter.next() {
                        value = next;
                    } else {
                        break;
                    }
                }
            }
        }
        values.sort_unstable();
    }

    Solution::I64(*values.iter().min().unwrap())
}

fn parse_seeds(input: &str) -> Vec<i64> {
    let mut seeds = Vec::new();
    if let Some((_, seed_line)) = input.split_once(' ') {
        let mut seed_line = seed_line.chars();
        while let Some(value) = parse_number(&mut seed_line) {
            seeds.push(value as i64);
        }
    }
    seeds.sort_unstable();
    seeds
}

fn parse_maps(input: &str) -> Vec<Vec<Translation>> {
    let mut maps = Vec::new();
    let mut lines = input.lines();
    lines.next();

    for _ in 0..7 {
        lines.next();
        let mut map = Vec::new();
        while let Some(translation) = lines.next().and_then(|line| parse_translation(line)) {
            map.push(translation);
        }
        map.sort_unstable_by_key(|t| t.end);
        maps.push(map);
    }

    maps
}

fn parse_translation(line: &str) -> Option<Translation> {
    if line.is_empty() {
        return None;
    }

    let mut line = line.chars();
    let dest = parse_number(&mut line).unwrap() as i64;
    let sorc = parse_number(&mut line).unwrap() as i64;
    let length = parse_number(&mut line).unwrap() as i64;

    Some(Translation {
        start: sorc,
        end: sorc + length - 1,
        offset: dest - sorc,
    })
}

struct Translation {
    start: i64,
    end: i64,
    offset: i64,
}

#[derive(Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn parse(start: u32, length: u32) -> Self {
        return Range {
            start: start as i64,
            end: start as i64 + length as i64 - 1,
        };
    }

    fn new(left: i64, right: i64) -> Self {
        return Range {
            start: left as i64,
            end: right as i64,
        };
    }
}

fn parse_ranges(input: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    if let Some((_, seed_line)) = input.split_once(' ') {
        let mut seed_line = seed_line.chars();

        while let Some(start) = parse_number(&mut seed_line) {
            let length = parse_number(&mut seed_line).unwrap();
            ranges.push(Range::parse(start, length));
        }
    }
    ranges.sort_unstable_by_key(|range| range.start);
    ranges
}

pub fn part_2(_input: &str) -> Solution {
    let (mut ranges, maps) = _input
        .split_once('\n')
        .and_then(|(left, right)| Some((parse_ranges(left), parse_maps(right))))
        .unwrap();

    join_overlapping_ranges(&mut ranges);

    for map in maps {
        let mut next_ranges: Vec<Range> = Vec::new();
        let mut range_iter = ranges.iter_mut();
        let mut translation_iter = map.iter();

        let mut range = range_iter.next().unwrap();
        let mut translation = translation_iter.next().unwrap();

        loop {
            if range.start > translation.end {
                if let Some(next) = translation_iter.next() {
                    translation = next;
                    continue;
                } else {
                    next_ranges.push(*range);
                    for range in range_iter {
                        next_ranges.push(*range);
                    }
                    break;
                }
            } else {
                if range.start < translation.start && range.end > translation.start {
                    next_ranges.push(Range::new(range.start, translation.start - 1));
                    range.start = translation.start;
                }

                if range.start >= translation.start {
                    if range.end > translation.end {
                        next_ranges.push(Range::new(
                            range.start + translation.offset,
                            translation.end + translation.offset,
                        ));
                        range.start = translation.end + 1;
                    } else {
                        next_ranges.push(Range::new(
                            range.start + translation.offset,
                            range.end + translation.offset,
                        ));

                        if let Some(next) = range_iter.next() {
                            range = next;
                        } else {
                            break;
                        }
                    }
                } else {
                    next_ranges.push(*range);
                    if let Some(next) = range_iter.next() {
                        range = next;
                    } else {
                        break;
                    }
                }
            }
        }

        ranges = next_ranges;
        ranges.sort_unstable_by_key(|range| range.start);
        join_overlapping_ranges(&mut ranges);
    }

    ranges.first().unwrap().start.into()
}

fn join_overlapping_ranges(ranges: &mut Vec<Range>) {
    let mut offset = 0;
    let len = ranges.len() - 1;
    for i in 0..len {
        if offset + i > len {
            break;
        }
        let next = ranges[i + 1 - offset];
        let current = ranges.get_mut(i).unwrap();

        if current.end >= next.start {
            current.end = next.end;
            ranges.remove(i + 1 - offset);
            offset += 1;
        }
    }
}
