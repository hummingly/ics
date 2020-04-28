use std::fmt;
use std::marker::PhantomData;
// use std::ops::*;
use std::str::FromStr;
use values::error::ParseDateError;

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
                day < month.max_days()
            }
        }
        _ => day <= month.max_days()
    }
}

fn is_leap_year(year: u16) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 > 0)
}

/// Month value for Date and DateTime
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Month {
    /// Month Value 1
    January = 1,
    /// Month Value 2
    February = 2,
    /// Month Value 3
    March = 3,
    /// Month Value 4
    April = 4,
    /// Month Value 5
    May = 5,
    /// Month Value 6
    June = 6,
    /// Month Value 7
    July = 7,
    /// Month Value 8
    August = 8,
    /// Month Value 9
    September = 9,
    /// Month Value 10
    October = 10,
    /// Month Value 11
    November = 11,
    /// Month Value 12
    December = 12
}

impl Month {
    fn max_days(self) -> u8 {
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

impl FromStr for Month {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 {
            return Err(ParseDateError::InvalidFormatting);
        }
        let month: u8 = s[4..6]
            .parse()
            .map_err(|_| ParseDateError::InvalidInteger)?;

        Ok(match month {
            1 => Month::January,
            2 => Month::February,
            3 => Month::March,
            4 => Month::April,
            5 => Month::May,
            6 => Month::June,
            7 => Month::July,
            8 => Month::August,
            9 => Month::September,
            10 => Month::October,
            11 => Month::November,
            12 => Month::December,
            _ => return Err(ParseDateError::OutOfRange)
        })
    }
}

/// ICalendar Date
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: Month,
    day: u8
}

impl Date {
    /// Creates a new date from a year, month and day.
    ///
    /// Returns a valid date if the year is in the range of 0 to 9999
    /// (inclusive) and the day and month match as well with the year.
    pub fn new(year: u16, month: Month, day: u8) -> Option<Self> {
        if !is_valid_date(year, month, day) {
            return None;
        }
        Some(Date { year, month, day })
    }

    /// Creates a new date time from this date and current time values if the
    /// time values are in range.
    pub fn and_hms<T>(self, hour: u8, minute: u8, second: u8) -> Option<DateTime<T>> {
        Time::new(hour, minute, second).map(|time| DateTime { date: self, time })
    }

    /// Returns the year value which is a value in the range of 0 to 9999
    /// (inclusive).
    pub fn year(self) -> u16 {
        self.year
    }

    /// Returns the month value.
    pub fn month(self) -> Month {
        self.month
    }

    /// Returns the day value which is value in the range of 1 to 31
    /// (inclusive).
    pub fn day(self) -> u8 {
        self.day
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:04}{:02}{:02}", self.year, self.month as u8, self.day)
    }
}

impl FromStr for Date {
    type Err = ParseDateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(ParseDateError::InvalidFormatting);
        }
        let year = s[0..4]
            .parse()
            .map_err(|_| ParseDateError::InvalidInteger)?;
        let month = s[4..6].parse()?;
        let day = s[6..].parse().map_err(|_| ParseDateError::InvalidInteger)?;

        Date::new(year, month, day).ok_or(ParseDateError::OutOfRange)
    }
}

/// ICalendar Date Time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DateTime<T = Local> {
    date: Date,
    time: Time<T>
}

impl<T> DateTime<T> {
    /// Creates a new date time.
    pub fn new(date: Date, time: Time<T>) -> Self {
        DateTime { date, time }
    }

    /// Creates a new date time with the time values set to 0.
    pub fn ymd(year: u16, month: Month, day: u8) -> Option<Self> {
        Date::new(year, month, day).map(|date| DateTime::new(date, Time::zero()))
    }

    /// Return a reference to the date.
    pub fn date(&self) -> &Date {
        &self.date
    }

    /// Return a reference to the time.
    pub fn time(&self) -> &Time<T> {
        &self.time
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

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// enum PeriodEnd<T = Local> {
//     DateTime(DateTime<T>),
//     Duration(Duration),
// }

// impl fmt::Display for PeriodEnd {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PeriodEnd::DateTime(d) => write!(f, "{}", d),
//             PeriodEnd::Duration(d) => write!(f, "{}", d),
//         }
//     }
// }

// impl fmt::Display for PeriodEnd<Utc> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             PeriodEnd::DateTime(d) => write!(f, "{}", d),
//             PeriodEnd::Duration(d) => write!(f, "{}", d),
//         }
//     }
// }

// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Period<T = Local> {
//     start: DateTime<T>,
//     end: PeriodEnd<T>,
// }

// impl Period {
//     pub fn local(start: DateTime, end: DateTime) -> Option<Self> {
//         if start >= end {
//             return None;
//         }
//         Some(Period { start, end: PeriodEnd::DateTime(end) })
//     }

//     pub fn duration(start: DateTime, duration: Duration) -> Option<Self> {
//         if duration.0 <= 0 {
//             return None;
//         }
//         Some(Period {
//             start,
//             end: PeriodEnd::Duration(duration)
//         })
//     }
// }

// impl Period<Utc> {
//     pub fn utc(start: DateTime<Utc>, end: DateTime<Utc>) -> Option<Self> {
//         if start >= end {
//             return None;
//         }
//         Some(Period { start, end: PeriodEnd::DateTime(end) })
//     }
// }

// impl fmt::Display for Period {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}/{}", self.start, self.end)
//     }
// }

// impl fmt::Display for Period<Utc> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}/{}", self.start, self.end)
//     }
// }

/// Local/Floating Time Marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Local {}
/// Utc Time Marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Utc {}

/// ICalendar Time
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Time<T = Local> {
    hour: u8,
    minute: u8,
    second: u8,
    _phantom: PhantomData<T>
}

impl Time {
    /// Creates a new local time value.
    pub fn local(hour: u8, minute: u8, second: u8) -> Option<Self> {
        Time::new(hour, minute, second)
    }
}

impl Time<Utc> {
    /// Creates a new time value in UTC.
    pub fn utc(hour: u8, minute: u8, second: u8) -> Option<Self> {
        Time::new(hour, minute, second)
    }
}

impl<T> Time<T> {
    /// Creates a new time value.
    pub fn new(hour: u8, minute: u8, second: u8) -> Option<Self> {
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

    /// Creates a time value with all values set to zero.
    pub fn zero() -> Self {
        Time {
            hour: 0,
            minute: 0,
            second: 0,
            _phantom: PhantomData
        }
    }

    /// Returns the hour value.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Returns the minute value.
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Returns the second value.
    pub fn second(&self) -> u8 {
        self.second
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
        let date = Date::new(1997, Month::July, 14).unwrap();
        let time = Time::local(17, 30, 0).unwrap();
        let datetime = DateTime::new(date, time);
        assert_eq!(datetime.to_string(), expected);
    }

    #[test]
    fn datetime_utc() {
        let expected = "19970714T173000Z";
        let date = Date::new(1997, Month::July, 14).unwrap();
        let time = Time::utc(17, 30, 0).unwrap();
        let datetime = DateTime::new(date, time);
        assert_eq!(datetime.to_string(), expected);
    }
}
