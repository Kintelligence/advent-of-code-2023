use shared::*;
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    Solution::None
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case("", 0)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), 0usize.into());
    }
}

pub fn part_2(_input: &str) -> Solution {
    Solution::None
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case("", 0)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), expected.into());
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), 0usize.into());
    }
}
