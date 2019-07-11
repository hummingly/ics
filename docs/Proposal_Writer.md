# Writer as Stream of ICalendar Objects

## Motivation

Before anything is written, the user creates an `ICalendar` instance and adds properties and components. However, this comes with a cost at runtime. Depending on how many properties and components an iCalendar object contains, memory grows linearly until the object is dropped. This might or might not be a problem depending on the type of application and requirements of the environment but with Rust it should be possible to build abstractions that are memory efficient and also performant.

Furthermore, it is actually possible to write several iCalendar objects into one stream, e.g. a file. The current design does not actively support that and makes it more cumbersome if needed.

## Design/Idea

One way is to avoid the overhead of storing properties and components in memory longer than necessary and write them directly to a stream of bytes.
Although, that solves memory inefficiencies, it would make the API verbose because the user would work with bytes again. The user should not need to think in terms of writing bytes but objects. A nice API that does abstract over a stream (`File`) is [pdf-canvas](https://github.com/kaj/rust-pdf). To render a page, the user provides a functions that takes as parameter a mutable reference to a canvas object and then calls the methods on this reference. The canvas object is only a temporary wrapper around the writer but with the methods that guide the user.
Another good example for a writer API is [csv](https://github.com/BurntSushi/rust-csv). Maybe it could be possible to write a whole struct as component with a proc macro.

## Implementation

### Writing

`ICalendarWriter` is a wrapper around a writer. The method `write_icalendar` creates an iCalendar object that has a mutable reference to the stream and is the main writing method. This method can be called several times but each time it creates a new iCalendar object in the stream.

```rust
pub struct ICalendarWriter {
    writer: dyn Write,
    ...
}

impl ICalendarWriter {
    // This is how all write methods will look in general
    pub fn write_icalendar<F>(&mut self, icalendar: F, ...) -> Result<()>
    where
        F: Fn(&mut ICalendar) -> Result<()>
    {
        // Write begin delimiter for component (VCALENDAR)
        write!(self.writer, "BEGIN:VCALENDAR\r\n")?;
        // Create mutable reference of write object (the component)
        // and pass mutable reference of writer. TH
        //
        // The passed function defines what methods are called
        // on the write object
        icalendar(&mut ICalendar::new(&mut self.writer, ...))?;
        // Write end delimiter for component (VCALENDAR)
        // if successful
        write!(self.writer, "END:VCALENDAR\r\n")?;
    }
}
```

#### Rewriting TODO Example

```rust
extern crate ics;

use ics::properties::*;
use ics::parameters::*;
use ics::*;

fn main() -> std::io::Result<()> {
    let mut writer = ICalendarWriter::new(
        "2.0", "-//ABC Corporation//NONSGML My Product//EN",
        File::create("icalendar.ics")?
    );

    writer.write_icalendar(|icalendar| {
        icalendar.write_todo("b68378cf-872d-44f1-9703-5e3725c56e71", "19980130T134500Z", |t| {
            t.write(Organizer::mailto("unclesam@example.com"))?;
            let mut attendee = Attendee::mailto("jqpublic@example.com");
            attendee.add(PartStat::ACCEPTED);
            t.write(attendee)?;
            t.write(Due::ymd(1998, 4, 15).hms(0, 0, 0))?;
            t.write(Status::needs_action())?;
            t.write(Summary::new("Submit Income Taxes"))?;
            t.write(Sequence::new(2))?;

            t.write_audio_alarm(Trigger::ymd(1998, 4, 3).utc(12, 0, 0), |a| {
                let mut attach = Attach::new("http://example.com/pub/audio-files/ssbanner.aud");
                attach.add(FmtType::new("audio", "basic"));
                a.write(attach)?;
                a.write(Repeat::new(4));
                a.write(Duration::hour(1));
            })
        })
    })
}

```

## Breaking Changes

The implementation is going to change the way the crate quite a lot, so backward compatible changes should land first (constants and constructors) and the type system. The minimum supported rustc version should not change, this will happen in 1.0 or 2.0 when upgrading to edition 2018. Names of components, properties and parameters should stay the same

## Alternatives

Keeping it the current way and adding methods could work to write several iCalendar objects but the memory overhead would still exist.
