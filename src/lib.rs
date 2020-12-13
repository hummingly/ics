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
//! use ics::properties::{Comment, DtStamp, ProdID, Status, Summary, UID, Version};
//! use ics::writer::{ICalendar, ToDo};
//! use std::fs::File;
//! use std::io;
//!
//! fn main() -> Result<(), io::Error> {
//!     let file = File::create("birthday.ics")?;
//!     // The ICalendar object is what is later written to the file.
//!     let mut calendar = ICalendar::new(file, Version::new("2.0"), ProdID::new("ics-rs"))?;
//!
//!     // For the sake of demonstrating the UID was taken from somewhere. Out of security
//!     // reasons the UID should always be randomly generated.
//!     let todo = ToDo::new(
//!         UID::new("d4092ed9-1667-4518-a7c0-bcfaac4f1fc6"),
//!         DtStamp::new("20181021T190000"),
//!         |todo| {
//!             todo.write(&Summary::new("Katarina's Birthday Present"))?;
//!             todo.write(&Comment::new("Buy her Imagine Dragons tickets!"))?;
//!             todo.write(&Status::needs_action())
//!         }
//!     );
//!     calendar.write_todo(todo)?;
//!
//!     // Write remaining bits to the file.
//!     calendar.close()?;
//!     Ok(())
//! }
//! ```

// #![forbid(unsafe_code, missing_docs)]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;
mod contentline;
pub mod parameters;
pub mod properties;
mod util;
mod value;
pub mod writer;
