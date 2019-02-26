[![Crates.io](https://img.shields.io/crates/v/ics.svg)](https://crates.io/crates/ics)
![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg)
[![Build Status](https://travis-ci.com/hummingly/ics.svg?branch=master)](https://travis-ci.com/hummingly/ics)
[![Documentation](https://docs.rs/ics/badge.svg)](https://docs.rs/ics)

A library for creating iCalendar files as specified in [RFC5545](https://tools.ietf.org/html/rfc5545) and [RFC7986](https://tools.ietf.org/html/rfc7986).

## Minimum supported rustc
**1.26.0+**

This version is officially supported and tested in CI. Changes to the minimum supported version will be noted in the Changelog.

## Installation
To use this library add the library as a dependency in your `Cargo.toml`:
```toml
[dependencies]
ics = "0.3"
```

Optionally you can disable default features.
```toml
[dependencies.ics]
version = "0.3"
default-features = false
 ```
## Features
- `rfc7986` (enabled by default): adds properties from the newer specification

## Usage
```rust
extern crate ics;

use ics::properties::{Categories, Description, DtEnd, DtStart, Organizer, Status, Summary};
use ics::{escape_text, Event, ICalendar};

fn main() -> std::io::Result<()> {
    // create new iCalendar object
    let mut calendar = ICalendar::new("2.0", "-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN");

    // create event which contains the information regarding the conference
    let mut event = Event::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19960704T120000Z");
    // add properties
    event.push(Organizer::new("mailto:jsmith@example.com"));
    event.push(DtStart::new("19960918T143000Z"));
    event.push(DtEnd::new("19960920T220000Z"));
    event.push(Status::new("CONFIRMED"));
    event.push(Categories::new("CONFERENCE"));
    event.push(Summary::new("Networld+Interop Conference"));
    // values that are "TEXT" must be escaped (only if the text contains a comma,
    // semicolon or backlash)
    event.push(Description::new(escape_text(
        "Networld+Interop Conference and Exhibit\n\
         Atlanta World Congress Center\n\
         Atlanta, Georgia"
    )));
    // add event to calendar
    calendar.add_event(event);

    // write calendar to file
    calendar.save_file("icalendar.ics")?;
    Ok(())

    /* inside icalendar.ics
    BEGIN:VCALENDAR
    PRODID:-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN
    VERSION:2.0
    BEGIN:VEVENT
    CATEGORIES:CONFERENCE
    DESCRIPTION:Networld+Interop Conference and Exhibit
    Atlanta World Congress 
     Center
    Atlanta\, Georgia
    DTEND:19960920T220000Z
    DTSTAMP:19960704T120000Z
    DTSTART:19960918T143000Z
    ORGANIZER:mailto:jsmith@example.com
    STATUS:CONFIRMED
    SUMMARY:Networld+Interop Conference
    UID:b68378cf-872d-44f1-9703-5e3725c56e71
    END:VEVENT
    END:VCALENDAR
    */
}
```

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions are always welcome!
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
