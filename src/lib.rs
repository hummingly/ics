//! A library for creating ICalendar files.
//!
//! The library supports the ICalendar specification [RFC5545](https://tools.ietf.org/html/rfc5545) version 2.0 and also [RFC7986](https://tools.ietf.org/html/rfc7986).
//!
//! # Installation
//! To use this library add the library as a dependency in your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! ics = "0.4"
//! ```
//!
//! By default some features are enabled. If you wish to disable them, specify
//! in your `Cargo.toml`:
//! ```toml
//! [dependencies.ics]
//! version = "0.4"
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
//! use ics::values::{DateTime, Month, Text};
//!
//! fn main() -> std::io::Result<()> {
//!     // Anything that can be converted to a Cow<str> is accepted as value which means
//!     // &str and String can be used freely. For the sake of demonstrating the UID was
//!     // taken from somewhere. Out of security reasons the UID should always be
//!     // randomly generated.
//!     let mut todo = ToDo::new(
//!         "d4092ed9-1667-4518-a7c0-bcfaac4f1fc6",
//!         DateTime::utc_ymd(2018, Month::October, 21)
//!             .and_then(|d| d.and_hms(19, 0, 0))
//!             .unwrap()
//!     );
//!     todo.push(Summary::new(Text::new("Katarina's Birthday Present")));
//!     todo.push(Comment::new(Text::new("Buy her Imagine Dragons tickets!")));
//!     todo.push(Status::needs_action());
//!
//!     // The ICalendar object is what is later written to the file.
//!     let calendar = ICalendar::new("2.0", "ics-rs", todo);
//!
//!     // Write `calendar` to a file.
//!     calendar.save_file("birthday.ics")?;
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code, missing_docs)]

#[macro_use]
mod macros;
mod core;
mod ical;
pub mod values;

pub use core::components;
pub use core::parameters;
pub use core::properties;

pub use ical::Alarm;
pub use ical::Daylight;
pub use ical::Event;
pub use ical::FreeBusy;
pub use ical::ICalendar;
pub use ical::Journal;
pub use ical::Standard;
pub use ical::TimeZone;
pub use ical::ToDo;
pub use ical::ZoneTime;
