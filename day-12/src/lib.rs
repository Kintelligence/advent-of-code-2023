use std::collections::HashMap;

use shared::{parse::Parsable, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    _input
        .lines()
        .map(|line| count_permutations(&parse_line(line), 0, 0, 0, &mut HashMap::new()))
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
        .map(|line| {
            count_permutations(&unfold_row(&parse_line(line)), 0, 0, 0, &mut HashMap::new())
        })
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
    #[ignore = "todo"]
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
    #[ignore = "reason"]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(527570479489));
    }
}

fn count_permutations(
    row: &Row,
    spring_index: usize,
    count_index: usize,
    current_count: usize,
    memoization: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(memo) = memoization.get(&(spring_index, count_index, current_count)) {
        return *memo;
    }

    let mut current_count = current_count;
    let mut spring_index = spring_index;
    let mut count_index = count_index;

    loop {
        if spring_index >= row.springs.len() {
            if (count_index > row.counts.len() - 1 && current_count == 0)
                || (count_index == row.counts.len() - 1 && current_count == row.counts[count_index])
            {
                memoization.insert((spring_index, count_index, current_count), 1);
                return 1;
            }

            memoization.insert((spring_index, count_index, current_count), 0);
            return 0;
        }

        match row.springs[spring_index] {
            Condition::Unknown => break,
            Condition::Operational => {
                if count_index > row.counts.len() - 1 {
                    memoization.insert((spring_index, count_index, current_count), 0);
                    return 0;
                }

                if current_count == row.counts[count_index] {
                    memoization.insert((spring_index, count_index, current_count), 0);
                    return 0;
                }

                current_count += 1;
                spring_index += 1;
            }
            Condition::Damaged => {
                if current_count == 0 {
                    spring_index += 1;
                } else if current_count == row.counts[count_index] {
                    spring_index += 1;
                    current_count = 0;
                    count_index += 1;
                } else {
                    memoization.insert((spring_index, count_index, current_count), 0);
                    return 0;
                }
            }
        }
    }

    let mut sum = 0;

    if current_count == 0 {
        if count_index < row.counts.len() {
            sum += count_permutations(row, spring_index + 1, count_index, 1, memoization)
                + count_permutations(row, spring_index + 1, count_index, 0, memoization);
        } else {
            sum += count_permutations(row, spring_index + 1, count_index, 0, memoization);
        }
    } else if count_index > row.counts.len() - 1 {
        sum += 0;
    } else if current_count == row.counts[count_index] {
        sum += count_permutations(row, spring_index + 1, count_index + 1, 0, memoization);
    } else {
        sum += count_permutations(
            row,
            spring_index + 1,
            count_index,
            current_count + 1,
            memoization,
        );
    }

    memoization.insert((spring_index, count_index, current_count), sum);
    return sum;
}

struct Row {
    springs: Vec<Condition>,
    counts: Vec<usize>,
}

#[derive(Clone, Copy)]
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
        unfolded.springs.push(Condition::Unknown);
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
            b'#' => springs.push(Condition::Operational),
            b'.' => springs.push(Condition::Damaged),
            b'?' => springs.push(Condition::Unknown),
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
