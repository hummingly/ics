extern crate ics;

use ics::properties::{Categories, Description, DtEnd, DtStart, Organizer, Status, Summary};
use ics::values::{DateTime, Month, Text};
use ics::{Event, ICalendar};

fn main() -> std::io::Result<()> {
    // Create event which contains the information regarding the conference
    // The required properties must be a unique identifier which should be random
    // generated and the date stamp which must be in UTC time.
    let mut event = Event::new(
        "b68378cf-872d-44f1-9703-5e3725c56e71",
        DateTime::utc_ymd(1996, Month::July, 4)
            .and_then(|d| d.and_hms(12, 0, 0))
            .unwrap()
    );
    event.push(Organizer::new(Text::new("mailto:jsmith@example.com")));
    event.push(DtStart::utc(
        DateTime::utc_ymd(1996, Month::September, 18)
            .and_then(|d| d.and_hms(14, 30, 0))
            .unwrap()
    ));
    event.push(DtEnd::utc(
        DateTime::utc_ymd(1996, Month::September, 20)
            .and_then(|d| d.and_hms(22, 0, 0))
            .unwrap()
    ));
    event.push(Status::confirmed());
    event.push(Categories::new(Text::new("CONFERENCE")));
    event.push(Summary::new(Text::new("Networld+Interop Conference")));

    // Values that are "TEXT" must be escaped (only if the text contains a comma,
    // semicolon or backlash).
    event.push(Description::new(Text::new(
        "Networld+Interop Conference and Exhibit\n\
         Atlanta World Congress Center\n\
         Atlanta, Georgia"
    )));

    // Create new iCalendar object
    // An iCalendar object must at least consist a component and the VERSION and
    // PRODID property.
    let calendar = ICalendar::new(
        "2.0",
        "-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN",
        event
    );
    // Write calendar to file
    calendar.save_file("event.ics")?;
    Ok(())

    /* inside event.ics
    BEGIN:VCALENDAR
    VERSION:2.0
    PRODID:-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN
    BEGIN:VEVENT
    UID:b68378cf-872d-44f1-9703-5e3725c56e71
    DTSTAMP:19960704T120000Z
    ORGANIZER:mailto:jsmith@example.com
    DTSTART:19960918T143000Z
    DTEND:19960920T220000Z
    STATUS:CONFIRMED
    CATEGORIES:CONFERENCE
    SUMMARY:Networld+Interop Conference
    DESCRIPTION:Networld+Interop Conference and Exhibit
    Atlanta World Congress
     Center
    Atlanta\, Georgia
    END:VEVENT
    END:VCALENDAR
    */
}
