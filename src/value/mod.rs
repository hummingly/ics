//! Value Types
mod encoding;
pub mod error;
mod string;
// mod time;

use std::borrow::Cow;
use std::fmt;
use std::str::FromStr;
use value::error::ParseBoolError;

/// ICalendar Boolean value type
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Boolean(bool);

impl Boolean {
    /// Creates Boolean.
    pub fn new(b: bool) -> Boolean {
        Boolean(b)
    }

    /// Returns bool value.
    pub fn get(self) -> bool {
        self.0
    }
}

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0 {
            write!(f, "TRUE")
        } else {
            write!(f, "FALSE")
        }
    }
}

impl FromStr for Boolean {
    type Err = ParseBoolError;

    fn from_str(s: &str) -> Result<Boolean, ParseBoolError> {
        match s {
            "TRUE" => Ok(Boolean(true)),
            "FALSE" => Ok(Boolean(false)),
            _ => Err(ParseBoolError::new())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Resource<'a> {
    Link(Cow<'a, str>),
    Data(Binary)
}

impl<'a> Resource<'a> {
    pub(crate) fn into_value(self) -> Cow<'a, str> {
        match self {
            Resource::Link(uri) => uri,
            Resource::Data(binary) => Cow::Owned(binary.to_string())
        }
    }
}

impl<'a> fmt::Display for Resource<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TRUE")
    }
}

pub use value::string::Binary;
// pub use self::string::CalAdress;
// pub use self::string::Text;
// pub use self::string::Uri;
// pub use self::time::Date;
// pub use self::time::DateTime;
// pub use self::time::Duration;
// pub use self::time::Period;
// pub use self::time::Recur;
// pub use self::time::Time;
// pub use self::time::UTCOffset;
