use ics::parameters::{FmtType, PartStat};
use ics::properties::{
    Attach, Attendee, Categories, Description, DtEnd, DtStart, Due, Duration, Organizer, Repeat,
    Sequence, Status, Summary, Trigger,
};
use ics::{escape_text, Alarm, Event, ICalendar, ToDo};

#[test]
fn icalendar_event() {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN\r\n\
                    BEGIN:VEVENT\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19960704T120000Z\r\n\
                    ORGANIZER:mailto:jsmith@example.com\r\n\
                    DTSTART:19960918T143000Z\r\n\
                    DTEND:19960920T220000Z\r\n\
                    STATUS:CONFIRMED\r\n\
                    CATEGORIES:CONFERENCE\r\n\
                    SUMMARY:Networld+Interop Conference\r\n\
                    DESCRIPTION:Networld+Interop Conference and Exhibit\\n\
                    Atlanta World Congress\r\n  Center\\n\
                    Atlanta\\, Georgia\r\n\
                    END:VEVENT\r\n\
                    END:VCALENDAR\r\n";

    let mut event = Event::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19960704T120000Z");
    event.push(Organizer::new("mailto:jsmith@example.com"));
    event.push(DtStart::new("19960918T143000Z"));
    event.push(DtEnd::new("19960920T220000Z"));
    event.push(Status::confirmed());
    event.push(Categories::new("CONFERENCE"));
    event.push(Summary::new("Networld+Interop Conference"));
    event.push(Description::new(escape_text(
        "Networld+Interop Conference and Exhibit\n\
         Atlanta World Congress Center\n\
         Atlanta, Georgia",
    )));

    let mut calendar = ICalendar::new("2.0", "-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN");
    calendar.add_event(event);

    assert_eq!(calendar.to_string(), expected);
}

#[test]
fn icalendar_todo() {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n\
                    BEGIN:VTODO\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19980130T134500Z\r\n\
                    ORGANIZER:mailto:unclesam@example.com\r\n\
                    ATTENDEE;PARTSTAT=ACCEPTED:mailto:jqpublic@example.com\r\n\
                    DUE:19980415T000000\r\n\
                    STATUS:NEEDS-ACTION\r\n\
                    SUMMARY:Submit Income Taxes\r\n\
                    SEQUENCE:2\r\n\
                    BEGIN:VALARM\r\n\
                    ACTION:AUDIO\r\n\
                    TRIGGER:19980403T120000Z\r\n\
                    ATTACH;FMTTYPE=audio/basic:http://example.com/pub/audio-files/ssbanner.aud\r\n\
                    REPEAT:4\r\n\
                    DURATION:PT1H\r\n\
                    END:VALARM\r\n\
                    END:VTODO\r\n\
                    END:VCALENDAR\r\n";

    let mut todo = ToDo::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19980130T134500Z");
    todo.push(Organizer::new("mailto:unclesam@example.com"));
    let mut attendee = Attendee::new("mailto:jqpublic@example.com");
    attendee.add(PartStat::ACCEPTED);
    todo.push(attendee);
    todo.push(Due::new("19980415T000000"));
    todo.push(Status::needs_action());
    todo.push(Summary::new("Submit Income Taxes"));
    todo.push(Sequence::new("2"));
    let mut alarm = Alarm::audio(Trigger::new("19980403T120000Z"));
    let mut attach = Attach::new("http://example.com/pub/audio-files/ssbanner.aud");
    attach.add(FmtType::new("audio/basic"));
    alarm.push(attach);
    alarm.push(Repeat::new("4"));
    alarm.push(Duration::new("PT1H"));
    todo.add_alarm(alarm);

    let mut calendar = ICalendar::new("2.0", "-//ABC Corporation//NONSGML My Product//EN");
    calendar.add_todo(todo);

    assert_eq!(calendar.to_string(), expected);
}
