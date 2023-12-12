use rayon::iter::{ParallelBridge, ParallelIterator};
use std::collections::HashMap;
use Condition::*;

use shared::{parse::Parsable, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .par_bridge()
        .map(|line| memoized_count(&parse_line(line), 0, 0, &mut HashMap::new()))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case("???.### 1,1,3", 1; "Line 1")]
    #[test_case(".??..??...?##. 1,1,3", 4; "Line 2")]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1; "Line 3")]
    #[test_case("????.#...#... 4,1,1", 1; "Line 4")]
    #[test_case("????.######..#####. 1,6,5", 4; "Line 5")]
    #[test_case("?###???????? 3,2,1", 10; "Line 6")]
    fn example_input_lines(input: &str, count: usize) {
        assert_eq!(part_1(input), Solution::Usize(count));
    }

    #[test]
    fn example_input() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part_1(input), Solution::Usize(21));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(7017));
    }
}

pub fn part_2(_input: &str) -> Solution {
    _input
        .lines()
        .par_bridge()
        .map(|line| memoized_count(&unfold_row(&parse_line(line)), 0, 0, &mut HashMap::new()))
        .sum::<usize>()
        .into()
}

mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case("???.### 1,1,3", 1; "Line 1")]
    #[test_case(".??..??...?##. 1,1,3", 16384; "Line 2")]
    #[test_case("?#?#?#?#?#?#?#? 1,3,1,6", 1; "Line 3")]
    #[test_case("????.#...#... 4,1,1", 16; "Line 4")]
    #[test_case("????.######..#####. 1,6,5", 2500; "Line 5")]
    #[test_case("?###???????? 3,2,1", 506250; "Line 6")]
    #[test_case("?#?##????#??.#?# 5,4,1,1", 3125; "Line 15")]
    fn example_input_lines(input: &str, count: usize) {
        assert_eq!(part_2(input), Solution::Usize(count));
    }

    #[test_case("..?.????#?????????? 1,1,1,1,1,4", 3916284121; "Line 16")]
    fn problem_input_lines(input: &str, count: usize) {
        assert_eq!(part_2(input), Solution::Usize(count));
    }

    #[test]
    fn example_input() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

        assert_eq!(part_2(input), Solution::Usize(525152));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(527570479489));
    }
}

fn memoized_count(
    row: &Row,
    spring_index: usize,
    count_index: usize,
    memoization: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(memo) = memoization.get(&(spring_index, count_index)) {
        return *memo;
    }
    let value = count_permutations(row, spring_index, count_index, memoization);
    memoization.insert((spring_index, count_index), value);
    value
}

fn count_permutations(
    row: &Row,
    spring_index: usize,
    count_index: usize,
    memoization: &mut HashMap<(usize, usize), usize>,
) -> usize {
    let window = row.counts[count_index];
    let next_window = row.counts.get(count_index + 1);

    let mut sum = 0;

    for (start, springs) in row.springs[spring_index..].windows(window).enumerate() {
        let window_start = start + spring_index;
        let window_end = window_start + window - 1;

        if window_start > 0 && row.springs[window_start - 1] == Operational {
            break;
        }

        if window_end + 1 <= row.springs.len() - 1 && row.springs[window_end + 1] == Operational {
            continue;
        }

        if springs.iter().any(|spring| *spring == Damaged) {
            continue;
        }

        if let Some(next) = next_window {
            if window_end + 1 + next - 1 > row.springs.len() - 1 {
                break;
            }

            let mut count = 0;
            let mut pot = 0;
            for (offset, spring) in row.springs[window_end + 2..].iter().enumerate() {
                match spring {
                    Damaged => {
                        if count != 0 {
                            break;
                        }
                        continue;
                    }
                    Operational => {
                        count += 1;
                        pot += 1;
                    }
                    Unknown => pot += 1,
                }

                if pot >= *next {
                    let next_start = window_end + 2 + offset - (next - 1);

                    sum += memoized_count(row, next_start, count_index + 1, memoization);
                    break;
                }
            }
        } else {
            if !row.springs[window_end + 1..]
                .iter()
                .any(|spring| *spring == Operational)
            {
                sum += 1;
            }
        }
    }
    sum
}

struct Row {
    springs: Vec<Condition>,
    counts: Vec<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Condition {
    Unknown,
    Operational,
    Damaged,
}

fn unfold_row(row: &Row) -> Row {
    let mut unfolded: Row = Row {
        springs: row.springs.clone(),
        counts: row.counts.clone(),
    };

    for _ in 0..4 {
        unfolded.springs.push(Unknown);
        unfolded.springs.append(&mut row.springs.clone());
        unfolded.counts.append(&mut row.counts.clone());
    }

    unfolded
}

fn parse_line(line: &str) -> Row {
    let mut bytes = line.bytes();

    let mut springs: Vec<Condition> = Vec::new();
    while let Some(byte) = bytes.next() {
        match byte {
            b'#' => springs.push(Operational),
            b'.' => springs.push(Damaged),
            b'?' => springs.push(Unknown),
            _ => {
                break;
            }
        }
    }

    let mut counts: Vec<usize> = Vec::new();
    while let Some(number) = bytes.next_number() {
        counts.push(number);
    }

    Row { springs, counts }
}
