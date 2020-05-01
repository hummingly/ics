//! Property Value Data Types
//!
//! The properties in an iCalendar object are strongly typed. Unless explicitly
//! specified by the VALUE parameter, the type is the default type for this
//! property.
mod encoding;
pub mod error;
mod string;
#[cfg(test)]
mod tests;
mod time;

use std::borrow::Cow;
use std::fmt;
use std::slice;
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
pub type Integer = i32;

/// ICalendar Float
pub type Float = f32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ListValue<'a, T: 'a + Clone> {
    Value(T),
    List(Cow<'a, [T]>)
}

/// Generic List for ICalendar types
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct List<'a, T: Clone + 'a>(ListValue<'a, T>);

impl<'a, T: Clone> List<'a, T> {
    /// Returns ICalendar values as slice.
    pub fn get(&self) -> &[T] {
        match &self.0 {
            // TODO: Change to slice::from_ref (safe wrapper around this line) when updating rustc.
            ListValue::Value(v) => unsafe { slice::from_raw_parts(v, 1) },
            ListValue::List(l) => l
        }
    }
}

impl<'a, T: Clone> From<T> for List<'a, T> {
    fn from(value: T) -> Self {
        List(ListValue::Value(value))
    }
}

impl<'a, T: Clone> From<&'a [T]> for List<'a, T> {
    fn from(value: &'a [T]) -> Self {
        List(ListValue::List(value.into()))
    }
}

impl<'a, T: Clone> From<Vec<T>> for List<'a, T> {
    fn from(value: Vec<T>) -> Self {
        List(ListValue::List(value.into()))
    }
}

// TEXT

impl<'a> From<&'a str> for List<'a, Text<'a>> {
    fn from(value: &'a str) -> Self {
        List(ListValue::Value(Text::from(value)))
    }
}

impl<'a> From<String> for List<'a, Text<'a>> {
    fn from(value: String) -> Self {
        List(ListValue::Value(Text::from(value)))
    }
}

impl<'a, T: Clone + fmt::Display> fmt::Display for List<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ListValue::Value(value) => write!(f, "{}", value),
            ListValue::List(list) => {
                if let Some(first) = list.first() {
                    write!(f, "{}", first)?;
                    for text in &list[1..] {
                        write!(f, ",{}", text)?;
                    }
                }
                Ok(())
            }
        }
    }
}

pub use values::string::Binary;
// pub use self::string::CalAdress;
pub use values::string::Text;
// pub use self::string::Uri;
pub use values::time::Date;
pub use values::time::DateTime;
pub use values::time::Month;
// pub use self::time::Duration;
// pub use self::time::Period;
// pub use self::time::Recur;
pub use values::time::Local;
pub use values::time::Time;
pub use values::time::Utc;
// pub use self::time::UTCOffset;
