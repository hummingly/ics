//! A library for creating ICalendar files.
//!
//! The library supports the ICalendar specification (RFC 5545) version 2.0.
//!
//! To use this library add the library as a dependency in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ics = "0.1"
//! ```
//! By default some features are enabled for speeding up some processes. If you
//! wish to disable them because you do not want any additional dependencies
//! except for the standard library, specify in your `Cargo.toml`:
//! ```toml
//! [dependencies.ics]
//! version = "0.1"
//! default-features = false
//! ```
//!
//! # Example
//!
//! ```
//! use ics::properties::{Comment, Summary};
//! use ics::{ICalendar, ToDo};
//!
//! // The ICalendar object is what is later written to the file.
//! let mut calendar = ICalendar::new("2.0", "ics-rs");
//!
//! // Anthing that can be converted to a Cow<str> is accepted as value which means
//! // &str and String can be used freely. For the sake of demonstrating the UID
//! // was taken from somewhere. Out of security reasons the UID shall be randomly
//! // generated from another crate.
//! let mut todo = ToDo::new("d4092ed9-1667-4518-a7c0-bcfaac4f1fc6", "20181021T190000");
//! todo.push(Summary::new("Katarina's Birthday Present"));
//! todo.push(Comment::new("Buy her the Imagine Dragons tickets."));
//!
//! calendar.add_todo(todo);
//!
//! // write `calendar` to a file
//! ```

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;
extern crate regex;

#[macro_use]
mod macros;
pub mod components;
mod ical;
pub mod parameters;
pub mod properties;
mod util;

pub use ical::Alarm;
pub use ical::Event;
pub use ical::FreeBusy;
pub use ical::ICalendar;
pub use ical::Journal;
pub use ical::TimeZone;
pub use ical::ToDo;
pub use ical::ZoneTime;

pub use util::escape_text;
