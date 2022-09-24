use ics::components::Event;
use ics::properties::{
    Categories, Description, DtEnd, DtStamp, DtStart, Organizer, ProdID, Status, Summary, Version,
    UID,
};
use ics::writer::ICalendar;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    // Crate a writer object to which the iCalendar object is written to.
    let mut file = File::create("event.ics")?;

    // Create a new iCalendar object.
    // An iCalendar object must at least consist of the VERSION and PRODID property.
    let mut calendar = ICalendar::new(
        &mut file,
        Version::new("2.0"),
        ProdID::new("-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN"),
    )?;

    // Create a new event.
    // The required properties must be a unique identifier which should be random
    // generated and the date stamp which must be in UTC time.
    let event = Event::build(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("19960704T120000Z"),
        // The passed parameter for writing the body of the event can either be a function or a
        // closure.
        |event| {
            event.write(&Organizer::new("mailto:jsmith@example.com"))?;
            event.write(&DtStart::new("19960918T143000Z"))?;
            event.write(&DtEnd::new("19960920T220000Z"))?;
            event.write(&Status::CONFIRMED)?;
            event.write(&Categories::new("CONFERENCE"))?;
            event.write(&Summary::new("Networld+Interop Conference"))?;
            // Values that are "TEXT" are escaped by default. To be escaped characters are
            // comma, semicolon, backslash and newline. Additionally, new lines are
            // normalized to a line feed character.
            event.write(&Description::new(
                "Networld+Interop Conference and Exhibit\n\
                Atlanta World Congress Center\n\
                Atlanta, Georgia",
            ))
        },
    );

    // Write the event into the writer.
    calendar.write_event(event)?;

    // Write remaining bits from calendar to file
    calendar.close()?;
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
    DESCRIPTION:Networld+Interop Conference and Exhibit\nAtlanta World Congress
      Center\nAtlanta\, Georgia
    END:VEVENT
    END:VCALENDAR
    */
}
