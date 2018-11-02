//! A library for creating ICalendar files.
//!
//! The library supports the ICalendar specification [RFC5545](https://tools.ietf.org/html/rfc5545) version 2.0 and also [RFC7986](https://tools.ietf.org/html/rfc7986).
//!
//! # Installation
//! To use this library add the library as a dependency in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ics = "0.2"
//! ```
//!
//! By default some features are enabled. If you wish to disable them, specify
//! in your `Cargo.toml`:
//! ```toml
//! [dependencies.ics]
//! version = "0.2"
//! default-features = false
//!
//! // optionally pick features
//! features = ["..."]
//! ```
//!
//! # Features
//! - `fast_text` (enabled by default): faster text processing in methods like
//!   `escape_text` but pulls in dependencies (regex and lazy_static)
//! - `rfc7986` (enabled by default): adds properties from the newer
//!   specification [RFC7986](https://tools.ietf.org/html/rfc7986)
//!
//! # Example
//! ```
//! use ics::properties::{Comment, Status, Summary};
//! use ics::{ICalendar, ToDo};
//! use std::fs::File;
//! use std::io::Write;
//!
//! // The ICalendar object is what is later written to the file.
//! let mut calendar = ICalendar::new("2.0", "ics-rs");
//!
//! // Anthing that can be converted to a Cow<str> is accepted as value which means
//! // &str and String can be used freely. For the sake of demonstrating the UID was
//! // taken from somewhere. Out of security reasons the UID should always be
//! // randomly generated.
//! let mut todo = ToDo::new("d4092ed9-1667-4518-a7c0-bcfaac4f1fc6", "20181021T190000");
//! todo.push(Summary::new("Katarina's Birthday Present"));
//! todo.push(Comment::new("Buy her Imagine Dragons tickets!"));
//! todo.push(Status::new("NEEDS-ACTION"));
//!
//! calendar.add_todo(todo);
//!
//! // Write `calendar` to a file.
//! let data = calendar.to_string();
//! let mut file = File::create("icalendar.ics").expect("Unable to create file");
//! file.write_all(data.as_bytes())
//!     .expect("Unable to write data");
//! ```

#![deny(missing_docs)]

#[cfg(feature = "fast_text")]
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "fast_text")]
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
