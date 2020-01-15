//! Property Value Data Types
//!
//! The properties in an iCalendar object are strongly typed. Unless explicitly
//! specified by the VALUE parameter, the type is the default type for this
//! property.
mod encoding;
pub mod error;
mod string;
// mod time;

use std::fmt;
use std::str::FromStr;
use values::error::ParseBoolError;

/// ICalendar Boolean
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

/// ICalendar Integer
///
/// This type maps perfectly to a Rust integer.
pub type Integer = i32;

/// ICalendar Float
///
/// This type maps perfectly to a Rust float point.
pub type Float = f32;

pub use values::string::Binary;
// pub use self::string::CalAdress;
pub use values::string::Text;
// pub use self::string::Uri;
// pub use self::time::Date;
// pub use self::time::DateTime;
// pub use self::time::Duration;
// pub use self::time::Period;
// pub use self::time::Recur;
// pub use self::time::Time;
// pub use self::time::UTCOffset;
