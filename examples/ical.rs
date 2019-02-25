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
