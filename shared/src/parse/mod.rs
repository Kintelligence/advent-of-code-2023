use std::str::Chars;

pub fn parse_number(input: &mut Chars) -> Option<u32> {
    let mut value: Option<u32> = None;
    for char in input {
        if let Some(digit) = char.to_digit(10) {
            if let Some(current) = value {
                value = Some(current * 10 + digit);
            } else {
                value = Some(digit);
            }
        } else {
            return value;
        }
    }

    value
}
