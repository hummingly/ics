use ics::parameters::{FmtType, PartStat};
use ics::properties::{
    Attach, Attendee, DtStamp, Due, Duration, Organizer, ProdID, Repeat, Sequence, Status, Summary,
    Trigger, Version, UID
};
use ics::writer::{Alarm, ICalendarWriter, ToDo};
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    // Crate a writer object to which the iCalendar object is written to.
    let file = File::create("todo.ics")?;

    // Create a new iCalendar object.
    // An iCalendar object must at least consist of the VERSION and PRODID property.
    let mut calendar = ICalendarWriter::new(
        file,
        Version::new("2.0"),
        ProdID::new("-//ABC Corporation//NONSGML My Product//EN")
    )?;

    // Create a simple todo.
    // The required properties must be a unique identifier which should be random
    // generated and the date stamp which must be in UTC time.
    let todo = ToDo::new(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("19980130T134500Z"),
        |todo| {
            todo.write(&Organizer::new("mailto:unclesam@example.com"))?;
            let mut attendee = Attendee::new("mailto:jqpublic@example.com");
            attendee.add(PartStat::ACCEPTED);
            todo.write(&attendee)?;
            todo.write(&Due::new("19980415T000000"))?;
            todo.write(&Status::needs_action())?;
            todo.write(&Summary::new("Submit Income Taxes"))?;
            todo.write(&Sequence::new(2))?;

            // Write an audio alarm into the todo.
            todo.write_alarm(Alarm::audio(Trigger::new("19980403T120000Z"), |alarm| {
                let mut attach = Attach::new("http://example.com/pub/audio-files/ssbanner.aud");
                attach.add(FmtType::new("audio/basic"));
                alarm.write(&attach)?;
                alarm.write(&Repeat::new(4))?;
                alarm.write(&Duration::new("PT1H"))
            }))
        }
    );

    // Write the todo into the writer.
    calendar.write_todo(todo)?;

    // Write remaining bits from calendar to file
    calendar.close()?;
    Ok(())

    /* inside todo.ics
    BEGIN:VCALENDAR
    VERSION:2.0
    PRODID:-//ABC Corporation//NONSGML My Product//EN
    BEGIN:VTODO
    UID:b68378cf-872d-44f1-9703-5e3725c56e71
    DTSTAMP:19980130T134500Z
    ORGANIZER:mailto:unclesam@example.com
    ATTENDEE;PARTSTAT=ACCEPTED:mailto:jqpublic@example.com
    DUE:19980415T000000
    STATUS:NEEDS-ACTION
    SUMMARY:Submit Income Taxes
    SEQUENCE:2
    BEGIN:VALARM
    ACTION:AUDIO
    TRIGGER:19980403T120000Z
    ATTACH;FMTTYPE=audio/basic:http://example.com/pub/audio-files/ssbanner.aud
    REPEAT:4
    DURATION:PT1H
    END:VALARM
    END:VTODO
    END:VCALENDAR
    */
}
