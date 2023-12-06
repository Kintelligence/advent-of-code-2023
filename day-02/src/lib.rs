use shared::*;
extern crate shared;

const _TEST: &'static str = include_str!("_test.txt");
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

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1(_TEST), Solution::Usize(8))
    }

    #[test]
    fn part_1_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(2348))
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2(_TEST), Solution::U32(2286))
    }

    #[test]
    fn part_2_input() {
        assert_eq!(part_2(_INPUT), Solution::U32(76008))
    }
}
