pub fn parse_u32<T>(input: &mut T) -> Option<u32>
where
    T: Iterator<Item = char>,
{
    let mut value: Option<u32> = None;
    for char in input {
        if let Some(digit) = char.to_digit(10) {
            if let Some(current) = value {
                value = Some(current * 10 + digit);
            } else {
                value = Some(digit);
            }
        } else if value.is_some() {
            return value;
        }
    }

    value
}

pub fn parse_u64<T>(input: &mut T) -> Option<u64>
where
    T: Iterator<Item = char>,
{
    let mut value: Option<u64> = None;
    for char in input {
        if let Some(digit) = char.to_digit(10) {
            if let Some(current) = value {
                value = Some(current * 10 + digit as u64);
            } else {
                value = Some(digit as u64);
            }
        } else if value.is_some() {
            return value;
        }
    }

    value
}

pub fn parse_u128<T>(input: &mut T) -> Option<u128>
where
    T: Iterator<Item = char>,
{
    let mut value: Option<u128> = None;
    for char in input {
        if let Some(digit) = char.to_digit(10) {
            if let Some(current) = value {
                value = Some(current * 10 + digit as u128);
            } else {
                value = Some(digit as u128);
            }
        } else if value.is_some() {
            return value;
        }
    }

    value
}
