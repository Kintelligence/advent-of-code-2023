use std::collections::HashMap;

use shared::{parse::Parsable, *};
extern crate shared;

pub const _INPUT: &'static str = include_str!("_input.txt");

pub fn part_1(_input: &str) -> Solution {
    let mut bytes = _input.bytes();
    let mut sum: usize = 0;
    let mut val: u8 = 0;

    while let Some(byte) = bytes.next() {
        if byte == b',' {
            sum += val as usize;
            val = 0;
        } else {
            val = val.wrapping_add(byte);
            val = val.wrapping_add(val << 4);
        }
    }
    sum += val as usize;

    sum.into()
}

#[derive(Clone, Copy)]
struct Lens {
    focal: usize,
    label: usize,
}

pub fn part_2(_input: &str) -> Solution {
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    let mut bytes = _input.bytes();
    let mut val: u8 = 0;
    let mut key: usize = 0;

    while let Some(byte) = bytes.next() {
        if byte == b'-' {
            let b = &mut boxes[val as usize];
            if let Some(index) = b.iter().position(|l| l.label == key) {
                b.remove(index);
            }
        } else if byte == b'=' {
            let focal: usize = bytes.next_number().expect("Should have number");
            let b = &mut boxes[val as usize];
            if let Some(index) = b.iter().position(|l| l.label == key) {
                b[index].focal = focal;
            } else {
                b.push(Lens {
                    focal: focal,
                    label: key,
                });
            }

            val = 0;
            key = 0;
        } else if byte == b',' {
            val = 0;
            key = 0;
        } else {
            key = key << 8 | byte as usize;
            val = val.wrapping_add(byte);
            val = val.wrapping_add(val << 4);
        }
    }

    let mut sum: usize = 0;
    for (i, lenses) in boxes.iter().enumerate() {
        lenses
            .iter()
            .enumerate()
            .for_each(|(n, lens)| sum += (i + 1) * (n + 1) * lens.focal);
    }

    sum.into()
}

#[cfg(test)]
mod part_1_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case("HASH", 52)]
    #[test_case("rn=1", 30)]
    #[test_case("cm-", 253)]
    #[test_case("qp=3", 97)]
    #[test_case("cm=2", 47)]
    #[test_case("qp-", 14)]
    #[test_case("pc=4", 180)]
    #[test_case("ot=9", 9)]
    #[test_case("ab=5", 197)]
    #[test_case("pc-", 48)]
    #[test_case("pc=6", 214)]
    #[test_case("ot=7", 231)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_1(input), Solution::Usize(expected));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_1(_INPUT), Solution::Usize(0));
    }
}

#[cfg(test)]
mod part_2_tests {
    use crate::*;
    use test_case::test_case;

    #[test_case("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7", 145)]
    fn example_input(input: &str, expected: usize) {
        assert_eq!(part_2(input), Solution::Usize(expected));
    }

    #[test]
    fn real_input() {
        assert_eq!(part_2(_INPUT), Solution::Usize(290779));
    }
}
