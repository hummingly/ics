use ics::parameters::{FmtType, PartStat};
use ics::properties::{
    Attach, Attendee, Due, Duration, Organizer, Repeat, Sequence, Status, Summary, Trigger
};
use ics::{Alarm, ICalendar, ToDo};

fn main() -> std::io::Result<()> {
    // Create simple todo
    // The required properties must be a unique identifier which should be random
    // generated and the date stamp which must be in UTC time.
    let mut todo = ToDo::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19980130T134500Z");
    todo.push(Organizer::new("mailto:unclesam@example.com"));
    let mut attendee = Attendee::new("mailto:jqpublic@example.com");
    attendee.add(PartStat::ACCEPTED);
    todo.push(attendee);
    todo.push(Due::new("19980415T000000"));
    todo.push(Status::needs_action());
    todo.push(Summary::new("Submit Income Taxes"));
    todo.push(Sequence::new("2"));
    // Add alarm to todo
    let mut alarm = Alarm::audio(Trigger::new("19980403T120000Z"));
    let mut attach = Attach::new("http://example.com/pub/audio-files/ssbanner.aud");
    attach.add(FmtType::new("audio/basic"));
    alarm.push(attach);
    alarm.push(Repeat::new("4"));
    alarm.push(Duration::new("PT1H"));
    todo.add_alarm(alarm);

    // Create new iCalendar object
    // An iCalendar object must at least consist a component and the VERSION and
    // PRODID property.
    let mut calendar = ICalendar::new("2.0", "-//ABC Corporation//NONSGML My Product//EN");
    calendar.add_todo(todo);
    // Write calendar to file
    calendar.save_file("todo.ics")?;
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
