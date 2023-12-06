macro_rules! parse_number {
    ($name:tt, $type:ident) => {
        pub fn $name<T>(input: &mut T) -> Option<$type>
        where
            T: Iterator<Item = char>,
        {
            let mut value: Option<$type> = None;
            for char in input {
                if let Some(digit) = char.to_digit(10) {
                    if let Some(current) = value {
                        value = Some(current * 10 + digit as $type);
                    } else {
                        value = Some(digit as $type);
                    }
                } else if value.is_some() {
                    return value;
                }
            }

            value
        }
    };
}

parse_number!(parse_u32, u32);
parse_number!(parse_u64, u64);
parse_number!(parse_u128, u128);
