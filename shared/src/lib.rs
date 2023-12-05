use std::{
    fmt::{Display, Formatter},
    time::{Duration, Instant},
};

#[derive(Clone)]
pub enum Solution {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    Str(String),
    None,
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(x) => x.fmt(f),
            Self::I16(x) => x.fmt(f),
            Self::I32(x) => x.fmt(f),
            Self::I64(x) => x.fmt(f),
            Self::I128(x) => x.fmt(f),
            Self::Isize(x) => x.fmt(f),
            Self::U8(x) => x.fmt(f),
            Self::U16(x) => x.fmt(f),
            Self::U32(x) => x.fmt(f),
            Self::U64(x) => x.fmt(f),
            Self::U128(x) => x.fmt(f),
            Self::Usize(x) => x.fmt(f),
            Self::Str(x) => x.fmt(f),
            Self::None => panic!("Cannot format NOTHING!"),
        }
    }
}

macro_rules! impl_from {
    ($type_:ident, $kind_:ident) => {
        impl From<$type_> for Solution {
            fn from(sol: $type_) -> Self {
                Self::$kind_(sol)
            }
        }
    };
}

impl_from!(i8, I8);
impl_from!(i16, I16);
impl_from!(i32, I32);
impl_from!(i64, I64);
impl_from!(i128, I128);
impl_from!(isize, Isize);
impl_from!(u8, U8);
impl_from!(u16, U16);
impl_from!(u32, U32);
impl_from!(u64, U64);
impl_from!(u128, U128);
impl_from!(usize, Usize);
impl_from!(String, Str);

impl From<&str> for Solution {
    fn from(sol: &str) -> Self {
        Self::Str(sol.to_owned())
    }
}

use colored::Colorize;

pub fn execute(f: &dyn Fn(&str) -> Solution, input: &str, name: &str) -> Duration {
    let start = Instant::now();
    let result = f(input);
    let time = start.elapsed();

    if let Solution::None = result {
        return Duration::ZERO;
    }

    let ratio = time.as_micros() as f64 / (Duration::from_secs(1) / 50).as_micros() as f64;

    let color = (ratio * 255.0).min(255.0) as u8;

    println!(
        "{: >12} {:} ({})",
        String::from(name).cyan().bold(),
        format!("{}", result).bold(),
        format!("{:#?}", time).truecolor(color, 255 - color, 0)
    );

    time
}

pub fn total(time: Duration) {
    let ratio = time.as_micros() as f64 / (Duration::from_secs(1)).as_micros() as f64;

    let color = (ratio * 255.0).min(255.0) as u8;

    let remaining = Duration::from_secs(1) - time;

    println!(
        "{: >12} {}",
        "Total".cyan().bold(),
        format!("{:#?}", time).truecolor(color, 255 - color, 0)
    );
    println!(
        "{: >12} {}",
        "Remaining".cyan().bold(),
        format!("{:#?}", remaining).truecolor(color, 255 - color, 0)
    );
}

pub mod parse;
pub mod vec2d;
