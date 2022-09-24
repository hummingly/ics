#![allow(dead_code)]
use std::{borrow::Cow, marker::PhantomData};

pub type Integer = i32;

pub type Float = f32;

// TODO: Validation?
pub type Uri<'u> = Cow<'u, str>;

// TODO: Validation?
pub type CalAdress<'a> = Cow<'a, str>;

pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}

/// Local/Floating Time Marker
pub enum Local {}
/// Utc Time Marker
pub enum Utc {}

/// ICalendar Time
pub struct Time<T = Local> {
    hour: u8,
    minute: u8,
    second: u8,
    _phantom: PhantomData<T>,
}

pub struct DateTime<T = Local> {
    date: Date,
    time: Time<T>,
}

pub struct UtcOffset {
    hour: i8,
    minute: u8,
    second: u8,
}

enum DurationInner {
    Week(u32),
    Day(u32),
    Time {
        hour: u8,
        minute: u8,
        second: u8,
    },
    DayTime {
        day: u32,
        hour: u8,
        minute: u8,
        second: u8,
    },
}

pub enum Positive {}
pub enum Negative {}

pub struct Duration<T = Positive> {
    inner: DurationInner,
    _phantom: PhantomData<T>,
}

impl<T> Duration<T> {
    fn new(duration: DurationInner) -> Self {
        Duration {
            inner: duration,
            _phantom: PhantomData,
        }
    }

    fn _week(week: u32) -> Self {
        Duration::new(DurationInner::Week(week))
    }

    fn _day(day: u32) -> Self {
        Duration::new(DurationInner::Day(day))
    }

    fn _day_time(day: u32, hour: u8, minute: u8, second: u8) -> Self {
        Duration::new(DurationInner::DayTime {
            day,
            hour,
            minute,
            second,
        })
    }

    fn _time(hour: u8, minute: u8, second: u8) -> Self {
        Duration::new(DurationInner::Time {
            hour,
            minute,
            second,
        })
    }
}

impl Duration {
    pub fn week(week: u32) -> Duration {
        Self::_week(week)
    }

    pub fn day(day: u32) -> Duration {
        Self::_day(day)
    }

    pub fn day_time(day: u32, hour: u8, minute: u8, second: u8) -> Duration {
        Self::_day_time(day, hour, minute, second)
    }

    pub fn time(hour: u8, minute: u8, second: u8) -> Duration {
        Self::_time(hour, minute, second)
    }

    pub fn into_negative(self) -> Duration<Negative> {
        Duration::new(self.inner)
    }
}

impl Duration<Negative> {
    pub fn neg_week(week: u32) -> Duration<Negative> {
        Self::_week(week)
    }

    pub fn neg_day(day: u32) -> Duration<Negative> {
        Self::_day(day)
    }

    pub fn neg_day_time(day: u32, hour: u8, minute: u8, second: u8) -> Duration<Negative> {
        Self::_day_time(day, hour, minute, second)
    }

    pub fn neg_time(hour: u8, minute: u8, second: u8) -> Duration<Negative> {
        Self::_time(hour, minute, second)
    }

    pub fn into_positive(self) -> Duration<Positive> {
        Duration::new(self.inner)
    }
}

enum Period<T = Local> {
    /// The type bound on the type parameters is stricter than the specification
    /// demands. However, if start and end had different parameters, the end
    /// could be before the start when a time zone is added as a parameter
    /// to a property. In practice T will be Utc as only FreeBusy and RDate
    /// use a Period in UTC time.
    Explicit {
        start: DateTime<T>,
        end: DateTime<T>,
    },
    Start {
        start: DateTime<T>,
        duration: Duration<Positive>,
    },
}

// Recur
// List

/// `STATUS` Property Values
///
/// [Format definitions of statuses](https://tools.ietf.org/html/rfc5545#section-3.8.1.11)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StatusValue {
    /// `TENTATIVE`
    ///
    /// Status for a tentative event
    Tentative,
    /// `CONFIRMED`
    ///
    /// Status for a definite event
    Confirmed,
    /// `CANCELLED`
    ///
    /// Status for a cancelled Event, To-Do or Journal
    Cancelled,
    /// `NEEDS-ACTION`
    ///
    /// Status for a To-Do that needs action
    NeedsAction,
    /// `COMPLETED`
    ///
    /// Status for a completed To-Do
    Completed,
    /// `IN-PROCESS`
    ///
    /// Status for an in-process To-Do
    InProcess,
    /// `DRAFT`
    ///
    /// Status for a draft Journal
    Draft,
    /// `FINAL`
    ///
    /// Status for a final Journal
    Final,
}

impl StatusValue {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            StatusValue::Tentative => "TENTATIVE",
            StatusValue::Confirmed => "CONFIRMED",
            StatusValue::Cancelled => "CANCELLED",
            StatusValue::NeedsAction => "NEEDS-ACTION",
            StatusValue::Completed => "COMPLETED",
            StatusValue::InProcess => "IN-PROCESS",
            StatusValue::Draft => "DRAFT",
            StatusValue::Final => "FINAL",
        }
    }
}

/// `Transp` Property Values
///
/// [Format definitions of time transparency](https://tools.ietf.org/html/rfc5545#section-3.8.2.7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TranspValue {
    /// `OPAQUE`
    ///
    /// Blocks or opaque on busy time searches. Default value is OPAQUE.
    Opaque,
    /// `TRANSPARENT`
    ///
    /// Transparent on busy time searches.
    Transparent,
}

impl TranspValue {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            TranspValue::Opaque => "OPAQUE",
            TranspValue::Transparent => "TRANSPARENT",
        }
    }
}

impl Default for TranspValue {
    fn default() -> Self {
        TranspValue::Opaque
    }
}
