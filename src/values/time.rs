use std::fmt;
use std::marker::PhantomData;
// use std::ops::*;
// use std::str::FromStr;

// // time units in seconds
// const SECOND: i64 = 1;
// const MINUTE: i64 = 60 * SECOND;
// const HOUR: i64 = 60 * MINUTE;
// const DAY: i64 = 24 * HOUR;
// const WEEK: i64 = 7 * DAY;

fn is_valid_date(year: u16, month: Month, day: u8) -> bool {
    if day == 0 || day > 31 || year > 9999 {
        return false;
    }

    match month {
        Month::February => {
            if is_leap_year(year) {
                day <= month.max_days()
            } else {
                day <= month.max_days() - 1
            }
        }
        _ => day <= month.max_days()
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 > 0)
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Month {
    ///
    January = 1,
    ///
    February = 2,
    ///
    March = 3,
    ///
    April = 4,
    ///
    May = 5,
    ///
    June = 6,
    ///
    July = 7,
    ///
    August = 8,
    ///
    September = 9,
    ///
    October = 10,
    ///
    November = 11,
    ///
    December = 12
}

impl Month {
    fn max_days(&self) -> u8 {
        match self {
            Month::January => 31,
            Month::February => 29,
            Month::March => 31,
            Month::April => 30,
            Month::May => 31,
            Month::June => 30,
            Month::July => 31,
            Month::August => 31,
            Month::September => 30,
            Month::October => 31,
            Month::November => 30,
            Month::December => 31
        }
    }
}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: Month,
    day: u8
}

impl Date {
    ///
    pub fn ymd(year: u16, month: Month, day: u8) -> Option<Self> {
        if !is_valid_date(year, month, day) {
            return None;
        }
        Some(Date { year, month, day })
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}{:02}", self.year, self.month as u8, self.day)
    }
}

// impl FromStr for Date {
//     // TODO: Replace placeholder
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if s.len() != 8 {
//             return Err(());
//         }

//         let year: u16 = s[0..4].parse().unwrap();
//         let month: u8 = s[4..6].parse().unwrap();
//         let day: u8 = s[6..].parse().unwrap();

//         Date::checked_ymd(year, month, day).ok_or(())
//     }
// }

// TODO: Custom PartialOrd and Ord implementation?
///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTime<T = Local> {
    date: Date,
    time: Time<T>
}

impl DateTime {
    ///
    pub fn local(date: Date, time: Time) -> Self {
        DateTime { date, time }
    }

    ///
    pub fn local_ymd(year: u16, month: Month, day: u8) -> Option<Self> {
        Date::ymd(year, month, day).map(|date| DateTime {
            date,
            time: Time {
                hour: 0,
                minute: 0,
                second: 0,
                _phantom: PhantomData
            }
        })
    }

    ///
    pub fn and_hms(self, hour: u8, minute: u8, second: u8) -> Option<Self> {
        Time::local(hour, minute, second).and_then(|time| Some(DateTime { time, ..self }))
    }
}

impl DateTime<Utc> {
    ///
    pub fn utc(date: Date, time: Time<Utc>) -> Self {
        DateTime { date, time }
    }

    ///
    pub fn utc_ymd(year: u16, month: Month, day: u8) -> Option<Self> {
        Date::ymd(year, month, day).map(|date| DateTime {
            date,
            time: Time {
                hour: 0,
                minute: 0,
                second: 0,
                _phantom: PhantomData
            }
        })
    }

    ///
    pub fn and_hms(self, hour: u8, minute: u8, second: u8) -> Option<Self> {
        Time::utc(hour, minute, second).map(|time| DateTime { time, ..self })
    }
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

impl fmt::Display for DateTime<Utc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}T{}", self.date, self.time)
    }
}

// // TODO: Check for std::i64::MIN
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Duration(i64);

// impl Duration {
//     pub const SECOND: Duration = Duration(SECOND);
//     pub const MINUTE: Duration = Duration(MINUTE);
//     pub const HOUR: Duration = Duration(HOUR);
//     pub const DAY: Duration = Duration(DAY);
//     pub const WEEK: Duration = Duration(WEEK);

//     fn new(seconds: i64) -> Self {
//         Duration(seconds)
//     }

//     pub fn hours(hours: i64) -> Self {
//         Duration::checked_hours(hours).unwrap()
//     }

//     pub fn checked_hours(hours: i64) -> Option<Self> {
//         HOUR.checked_mul(hours).map(Duration::new)
//     }

//     pub fn minutes(minutes: i64) -> Self {
//         Duration::checked_minutes(minutes).unwrap()
//     }

//     pub fn checked_minutes(minutes: i64) -> Option<Self> {
//         MINUTE.checked_mul(minutes).map(Duration::new)
//     }

//     pub fn seconds(seconds: i64) -> Self {
//         Duration::checked_seconds(seconds).unwrap()
//     }

//     pub fn checked_seconds(seconds: i64) -> Option<Self> {
//         SECOND.checked_mul(seconds).map(Duration::new)
//     }

//     pub fn days(days: i64) -> Self {
//         Duration::checked_days(days).unwrap()
//     }

//     pub fn checked_days(days: i64) -> Option<Self> {
//         DAY.checked_mul(days).map(Duration::new)
//     }

//     pub fn weeks(weeks: i64) -> Self {
//         Duration::checked_weeks(weeks).unwrap()
//     }

//     pub fn checked_weeks(weeks: i64) -> Option<Self> {
//         WEEK.checked_mul(weeks).map(Duration::new)
//     }
// }

// fn write_time(f: &mut fmt::Formatter, time: i64) -> fmt::Result {
//     if time == 0 {
//         return Ok(());
//     }

//     write!(f, "T")?;
//     let (hours, m) = modulus(time, HOUR);
//     let (minutes, seconds) = modulus(m, MINUTE);

//     if hours > 0 {
//         write!(f, "{}H", hours)?;

//         if seconds > 0 {
//             return write!(f, "{}M{}S", minutes, seconds);
//         }
//     }

//     if minutes > 0 {
//         write!(f, "{}M", minutes)?;
//     }

//     if seconds > 0 {
//         write!(f, "{}S", seconds)?;
//     }

//     Ok(())
// }

// fn modulus(value: i64, divider: i64) -> (i64, i64) {
//     (value / divider, value % divider)
// }

// impl fmt::Display for Duration {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.0 == 0 {
//             return write!(f, "PT0S");
//         }

//         if self.0.is_negative() {
//             write!(f, "-")?;
//         }
//         write!(f, "P")?;

//         if self.0 % WEEK == 0 {
//             write!(f, "{}W", self.0 / WEEK)
//         } else if self.0.abs() >= DAY {
//             let (days, time) = modulus(self.0, DAY);
//             write!(f, "{}D", days)?;
//             write_time(f, time)
//         } else {
//             write_time(f, self.0 % DAY)
//         }
//     }
// }

// impl Neg for Duration {
//     type Output = Self;

//     fn neg(self) -> Self::Output {
//         Duration::seconds(-self.0)
//     }
// }

// impl Add for Duration {
//     type Output = Self;

//     fn add(self, rhs: Self) -> Self::Output {
//         Duration::seconds(self.0 + rhs.0)
//     }
// }

// impl Sub for Duration {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Duration::seconds(self.0 - rhs.0)
//     }
// }

// impl Mul<i64> for Duration {
//     type Output = Self;

//     fn mul(self, rhs: i64) -> Self::Output {
//         Duration::seconds(self.0 * rhs)
//     }
// }

// impl Div<i64> for Duration {
//     // The division of rational numbers is a closed operation.
//     type Output = Self;

//     fn div(self, rhs: i64) -> Self::Output {
//         Duration::seconds(self.0 / rhs)
//     }
// }

// // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Period<T> {
//     start: DateTime,
//     end: T
// }

// impl Period<DateTime> {
//     pub fn date(start: DateTime, end: DateTime) -> Self {
//         Period { start, end }
//     }
// }

// impl Period<Duration> {
//     pub fn duration(start: DateTime, duration: Duration) -> Self {
//         Period {
//             start,
//             end: duration
//         }
//     }
// }

// impl fmt::Display for Period<DateTime> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}/{}", self.start, self.end)
//     }
// }

// impl fmt::Display for Period<Duration> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}/{}", self.start, self.end)
//     }
// }

/// Marker trait for Time and DateTime
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Local {}
/// Marker trait for Time and DateTime
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Utc {}

///
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Time<T = Local> {
    hour: u8,
    minute: u8,
    second: u8,
    _phantom: PhantomData<T>
}

impl Time {
    ///
    pub fn local(hour: u8, minute: u8, second: u8) -> Option<Self> {
        if hour > 23 || minute > 59 || second > 60 {
            return None;
        }
        Some(Time {
            hour,
            minute,
            second,
            _phantom: PhantomData
        })
    }
}

impl Time<Utc> {
    ///
    pub fn utc(hour: u8, minute: u8, second: u8) -> Option<Self> {
        if hour > 23 || minute > 59 || second > 60 {
            return None;
        }
        Some(Time {
            hour,
            minute,
            second,
            _phantom: PhantomData
        })
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}{:02}{:02}", self.hour, self.minute, self.second)
    }
}

impl fmt::Display for Time<Utc> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}{:02}{:02}Z", self.hour, self.minute, self.second)
    }
}

// pub struct UTCOffset(i32);

// impl UTCOffset {
//     fn new(seconds: i32) -> Self {
//         UTCOffset(seconds)
//     }

//     pub fn east(hour: u8, minute: u8, second: u8) -> Self {
//         UTCOffset::checked_east(hour, minute, second).unwrap()
//     }

//     pub fn checked_east(hour: u8, minute: u8, second: u8) -> Option<Self> {
//         if hour > 23 || minute > 59 || second > 59 {
//             return None;
//         }
//         let (h, m, s) = (hour as i32, minute as i32, second as i32);
//         Some(UTCOffset::new(h * HOUR as i32 + m * MINUTE as i32 + s))
//     }

//     pub fn west(hour: u8, minute: u8, second: u8) -> Self {
//         UTCOffset::checked_west(hour, minute, second).unwrap()
//     }

//     pub fn checked_west(hour: u8, minute: u8, second: u8) -> Option<Self> {
//         if hour > 23 || minute > 59 || second > 59 {
//             return None;
//         }
//         let (h, m, s) = (hour as i32, minute as i32, second as i32);
//         Some(UTCOffset::new(-(h * HOUR as i32 + m * MINUTE as i32 + s)))
//     }
// }

// impl fmt::Display for UTCOffset {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         if self.0 == 0 {
//             return write!(f, "+0000");
//         }

//         let (hours, m) = modulus(self.0 as i64, HOUR);
//         let (minutes, seconds) = modulus(m as i64, MINUTE);

//         write!(f, "{}", if self.0.is_positive() { "+" } else { "-" })?;
//         write!(f, "{}{}", hours, minutes)?;
//         if seconds > 0 {
//             write!(f, "{}", seconds)?;
//         }
//         Ok(())
//     }
// }

// impl FromStr for UTCOffset {
//     // TODO: Replace placeholder
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         if s.len() < 5 {
//             return Err(());
//         }

//         let sign = &s[0..1];
//         let hour: u8 = s[1..3].parse().unwrap();
//         let minute: u8 = s[3..5].parse().unwrap();
//         let second: u8 = if s.len() == 7 {
//             s[5..7].parse().unwrap()
//         } else {
//             0
//         };

//         match sign {
//             "+" => UTCOffset::checked_east(hour, minute, second).ok_or(()),
//             "-" => UTCOffset::checked_west(hour, minute, second).ok_or(()),
//             _ => Err(())
//         }
//     }
// }

// pub struct Recur;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn time_utc() {
        let expected = "173000Z";
        let time = Time::utc(17, 30, 0).unwrap();
        assert_eq!(time.to_string(), expected);
    }

    #[test]
    fn time_local() {
        let expected = "173000";
        let time = Time::local(17, 30, 0).unwrap();
        assert_eq!(time.to_string(), expected);
    }

    #[test]
    fn datetime_local() {
        let expected = "19970714T173000";
        let date = Date::ymd(1997, Month::July, 14).unwrap();
        let time = Time::local(17, 30, 0).unwrap();
        let datetime = DateTime::local(date, time);
        assert_eq!(datetime.to_string(), expected);
    }

    #[test]
    fn datetime_utc() {
        let expected = "19970714T173000Z";
        let date = Date::ymd(1997, Month::July, 14).unwrap();
        let time = Time::utc(17, 30, 0).unwrap();
        let datetime = DateTime::utc(date, time);
        assert_eq!(datetime.to_string(), expected);
    }
}
