use shared::*;
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    solve_1(_input, 64)
}

fn solve_1(input: &str, steps: usize) -> Solution {
    Solution::None
}

pub fn part_2(_input: &str) -> Solution {
    Solution::None
}

#[cfg(test)]
mod tests {
    use crate::*;
    use test_case::test_case;

    #[test_case(include_str!("_test.txt"), 6, 16)]
    fn part_1_test(input: &str, steps: usize, expected: usize) {
        assert_eq!(solve_1(input, steps), expected.into());
    }
}
