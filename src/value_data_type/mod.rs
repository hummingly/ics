#![allow(dead_code)]
mod definitions;

use std::fmt;
use value_data_type::definitions::DurationValue;
use value_data_type::definitions::RecurRule;
use value_data_type::definitions::Sign;

pub struct Binary(String);

pub struct Boolean(bool);

pub struct CalAddress(URI);

pub struct Date {
    year: u32,
    month: u32,
    day: u32
}

pub struct DateTime {
    date: Date,
    time: Time
}

pub struct Duration {
    sign: Sign,
    value: DurationValue
}

pub struct Float(f32);

pub struct Integer(i32);

pub enum Period {
    Explicit(DateTime, DateTime),
    Start(DateTime, Duration)
}

pub struct Recur(Vec<RecurRule>);

pub struct Text(String);

pub struct Time {
    seconds: u32,
    time_utc: bool
}

pub struct URI(String);

pub struct UTCOffset {
    sign: Sign,
    seconds: u32
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                false => "FALSE",
                true => "TRUE"
            }
        )
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.year)?;
        if self.month < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", self.month)?;
        if self.day < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", self.month)
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}P{}", self.sign, self.value)
    }
}

impl fmt::Display for DurationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DurationValue::Date(day, time) => {
                write!(f, "{}D", day)?;
                write_hms(f, time)
            }
            DurationValue::Time(time) => write_hms(f, time),
            DurationValue::Week(week) => write!(f, "{}W", week)
        }
    }
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Period::Explicit(start, end) => write!(f, "{}/{}", start, end),
            Period::Start(start, duration) => write!(f, "{}/{}", start, duration)
        }
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (hours, minutes, seconds) = to_hms(self.seconds);
        if hours < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", hours)?;
        if minutes < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", minutes)?;
        if seconds < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", seconds)?;
        if self.time_utc {
            write!(f, "Z")?;
        }
        Ok(())
    }
}

fn write_hms(f: &mut fmt::Formatter, seconds: u32) -> fmt::Result {
    let (hours, minutes, seconds) = to_hms(seconds);
    let combination = (hours > 0, minutes > 0, seconds > 0);
    match combination {
        (true, true, true) => write!(f, "T{}H{}M{}S", hours, minutes, seconds),
        (true, false, true) => write!(f, "T{}H0M{}S", hours, seconds),
        (true, true, _) => write!(f, "T{}H{}M", hours, minutes),
        (_, true, true) => write!(f, "T{}M{}S", minutes, seconds),
        (true, _, _) => write!(f, "T{}H", hours),
        (_, true, _) => write!(f, "T{}M", minutes),
        _ => write!(f, "T{}S", seconds)
    }
}

fn to_hms(seconds: u32) -> (u32, u32, u32) {
    let (hours, rem) = int_div_with_rem(seconds, 3_600);
    let (minutes, seconds) = int_div_with_rem(rem, 60);
    (hours, minutes, seconds)
}

fn int_div_with_rem(dividend: u32, divisior: u32) -> (u32, u32) {
    let quotient = dividend / divisior;
    let remainder = dividend % divisior;
    (quotient, remainder)
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sign::Minus => "-",
                Sign::Plus => "+"
            }
        )
    }
}
