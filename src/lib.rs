//! A library for creating ICalendar files.
//!
//! The library supports the ICalendar specification [RFC5545](https://tools.ietf.org/html/rfc5545) version 2.0 and also [RFC7986](https://tools.ietf.org/html/rfc7986).
//!
//! # Installation
//! To use this library add the library as a dependency in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ics = "0.5"
//! ```
//!
//! By default some features are enabled. If you wish to disable them, specify
//! in your `Cargo.toml`:
//! ```toml
//! [dependencies.ics]
//! version = "0.5"
//! default-features = false
//! ```
//!
//! # Features
//! - `rfc7986` (enabled by default): adds properties from the newer
//!   specification [RFC7986](https://tools.ietf.org/html/rfc7986)
//!
//! # Example
//! ```
//! use ics::properties::{Comment, Status, Summary};
//! use ics::{ICalendar, ToDo};
//!
//! fn main() -> std::io::Result<()> {
//!     // Anything that can be converted to a Cow<str> is accepted as value which means
//!     // &str and String can be used freely. For the sake of demonstrating the UID was
//!     // taken from somewhere. Out of security reasons the UID should always be
//!     // randomly generated.
//!     let mut todo = ToDo::new("d4092ed9-1667-4518-a7c0-bcfaac4f1fc6", "20181021T190000");
//!     todo.push(Summary::new("Katarina's Birthday Present"));
//!     todo.push(Comment::new("Buy her Imagine Dragons tickets!"));
//!     todo.push(Status::needs_action());
//!
//!     // The ICalendar object is what is later written to the file.
//!     let mut calendar = ICalendar::new("2.0", "ics-rs");
//!     calendar.add_todo(todo);
//!
//!     // Write `calendar` to a file.
//!     calendar.save_file("birthday.ics")?;
//!     Ok(())
//! }
//! ```

// #![forbid(unsafe_code, missing_docs)]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;
pub mod components;
pub mod contentline;
mod ical;
pub mod parameters;
pub mod properties;
mod util;
mod value;
mod writer;

pub use crate::ical::{
    Alarm, Daylight, Event, FreeBusy, ICalendar, Journal, Standard, TimeZone, ToDo
};

pub use crate::util::escape_text;
