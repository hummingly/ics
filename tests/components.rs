use ics::parameters::{FmtType, Related, Value};
use ics::properties::{
    Attach, Attendee, Categories, Class, Completed, Description, DtEnd, DtStamp, DtStart, Due,
    FreeBusyTime, LastModified, Organizer, Priority, ProdID, RRule, Status, Summary, Transp,
    Trigger, TzID, TzName, TzOffsetFrom, TzOffsetTo, Version, UID, URL
};
use ics::writer::{
    Alarm, Daylight, Event, FreeBusy, ICalendarWriter, Journal, Standard, TimeZone, ToDo
};
use std::io;

fn calendar(capacity: usize) -> Result<ICalendarWriter<Vec<u8>>, io::Error> {
    ICalendarWriter::new(
        Vec::with_capacity(capacity),
        Version::new("2.0"),
        ProdID::new("Mock Calendar")
    )
}

#[test]
fn event() -> Result<(), io::Error> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:Mock Calendar\r\n\
                    BEGIN:VEVENT\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19970901T130000Z\r\n\
                    CATEGORIES:ANNIVERSARY,PERSONAL,SPECIAL OCCASION\r\n\
                    CLASS:CONFIDENTIAL\r\n\
                    DTSTART;VALUE=DATE:19971102\r\n\
                    RRULE:FREQ=YEARLY\r\n\
                    SUMMARY:Our Blissful Anniversary\r\n\
                    TRANSP:TRANSPARENT\r\n\
                    END:VEVENT\r\n\
                    END:VCALENDAR\r\n";

    let mut calendar = calendar(expected.len())?;

    let event = Event::new(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("19970901T130000Z"),
        |event| {
            event.write(&Categories::new("ANNIVERSARY,PERSONAL,SPECIAL OCCASION"))?;
            event.write(&Class::confidential())?;
            let mut date = DtStart::new("19971102");
            date.add(Value::DATE);
            event.write(&date)?;
            event.write(&RRule::new("FREQ=YEARLY"))?;
            event.write(&Summary::new("Our Blissful Anniversary"))?;
            event.write(&Transp::transparent())
        }
    );
    calendar.write_event(event)?;

    let output = calendar.close()?;
    let output = String::from_utf8_lossy(&output);

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn todo() -> Result<(), io::Error> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:Mock Calendar\r\n\
                    BEGIN:VTODO\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:20070514T103211Z\r\n\
                    COMPLETED:20070707T100000Z\r\n\
                    DTSTART:20070514T110000Z\r\n\
                    DUE:20070709T130000Z\r\n\
                    PRIORITY:1\r\n\
                    SUMMARY:Submit Revised Internet-Draft\r\n\
                    STATUS:NEEDS-ACTION\r\n\
                    END:VTODO\r\n\
                    END:VCALENDAR\r\n";

    let mut calendar = calendar(expected.len())?;

    let todo = ToDo::new(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("20070514T103211Z"),
        |todo| {
            todo.write(&Completed::new("20070707T100000Z"))?;
            todo.write(&DtStart::new("20070514T110000Z"))?;
            todo.write(&Due::new("20070709T130000Z"))?;
            todo.write(&Priority::new(1))?;
            todo.write(&Summary::new("Submit Revised Internet-Draft"))?;
            todo.write(&Status::needs_action())
        }
    );
    calendar.write_todo(todo)?;

    let output = calendar.close()?;
    let output = String::from_utf8_lossy(&output);

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn journal() -> Result<(), io::Error> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:Mock Calendar\r\n\
                    BEGIN:VJOURNAL\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19970901T130000Z\r\n\
                    DTSTART;VALUE=DATE:19970317\r\n\
                    SUMMARY:Staff meeting minutes\r\n\
                    DESCRIPTION:1. Staff meeting: Participants include Joe\\, Lisa\\, and Bob. Au\r\n rora project plans were reviewed. There is currently no budget reserves for\r\n  this project. Lisa will escalate to management. Next meeting on Tuesday.\n\
                    2\r\n . Telephone Conference: ABC Corp. sales representative called to discuss ne\r\n w printer. Promised to get us a demo by Friday.\n\
                    3. Henry Miller (Handsoff I\r\n nsurance): Car was totaled by tree. Is looking into a loaner car. 555-2323 \r\n (tel).\r\n\
                    END:VJOURNAL\r\n\
                    END:VCALENDAR\r\n";

    let mut calendar = calendar(expected.len())?;

    let journal = Journal::new(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("19970901T130000Z"),
        |journal| {
            let mut date = DtStart::new("19970317");
            date.add(Value::DATE);
            journal.write(&date)?;
            journal.write(&Summary::new("Staff meeting minutes"))?;
            journal.write(&Description::new(
                "1. Staff meeting: Participants include Joe, Lisa, and Bob. Aurora project plans were reviewed. There is currently no budget reserves for this project. Lisa will escalate to management. Next meeting on Tuesday.\n\
                2. Telephone Conference: ABC Corp. sales representative called to discuss new printer. Promised to get us a demo by Friday.\n\
                3. Henry Miller (Handsoff Insurance): Car was totaled by tree. Is looking into a loaner car. 555-2323 (tel)."
            ))
        }
    );
    calendar.write_journal(journal)?;

    let output = calendar.close()?;
    let output = String::from_utf8_lossy(&output);

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn freebusy() -> Result<(), io::Error> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:Mock Calendar\r\n\
                    BEGIN:VFREEBUSY\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19970901T120000Z\r\n\
                    DTSTART:19980313T141711Z\r\n\
                    DTEND:19980410T141711Z\r\n\
                    FREEBUSY:19980314T233000Z/19980315T003000Z\r\n\
                    FREEBUSY:19980316T153000Z/19980316T163000Z\r\n\
                    FREEBUSY:19980318T030000Z/19980318T040000Z\r\n\
                    ORGANIZER:jsmith@example.com\r\n\
                    URL:http://www.example.com/calendar/busytime/jsmith.ifb\r\n\
                    END:VFREEBUSY\r\n\
                    END:VCALENDAR\r\n";

    let mut calendar = calendar(expected.len())?;

    let freebusy = FreeBusy::new(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("19970901T120000Z"),
        |freebusy| {
            freebusy.write(&DtStart::new("19980313T141711Z"))?;
            freebusy.write(&DtEnd::new("19980410T141711Z"))?;
            freebusy.write(&FreeBusyTime::new("19980314T233000Z/19980315T003000Z"))?;
            freebusy.write(&FreeBusyTime::new("19980316T153000Z/19980316T163000Z"))?;
            freebusy.write(&FreeBusyTime::new("19980318T030000Z/19980318T040000Z"))?;
            freebusy.write(&Organizer::new("jsmith@example.com"))?;
            freebusy.write(&URL::new(
                "http://www.example.com/calendar/busytime/jsmith.ifb"
            ))
        }
    );

    calendar.write_freebusy(freebusy)?;

    let output = calendar.close()?;
    let output = String::from_utf8_lossy(&output);

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn timezone() -> Result<(), io::Error> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:Mock Calendar\r\n\
                    BEGIN:VTIMEZONE\r\n\
                    TZID:America/New_York\r\n\
                    BEGIN:STANDARD\r\n\
                    DTSTART:20071104T020000\r\n\
                    TZOFFSETFROM:-0400\r\n\
                    TZOFFSETTO:-0500\r\n\
                    TZNAME:EST\r\n\
                    END:STANDARD\r\n\
                    LAST-MODIFIED:20050809T050000Z\r\n\
                    BEGIN:DAYLIGHT\r\n\
                    DTSTART:20070311T020000\r\n\
                    TZOFFSETFROM:-0500\r\n\
                    TZOFFSETTO:-0400\r\n\
                    TZNAME:EDT\r\n\
                    END:DAYLIGHT\r\n\
                    END:VTIMEZONE\r\n\
                    END:VCALENDAR\r\n";

    let mut calendar = calendar(expected.len())?;

    let standard = Standard::new(
        DtStart::new("20071104T020000"),
        TzOffsetFrom::new("-0400"),
        TzOffsetTo::new("-0500"),
        |standard| standard.write(&TzName::new("EST"))
    );
    let daylight = Daylight::new(
        DtStart::new("20070311T020000"),
        TzOffsetFrom::new("-0500"),
        TzOffsetTo::new("-0400"),
        |daylight| daylight.write(&TzName::new("EDT"))
    );
    let timezone = TimeZone::standard(TzID::new("America/New_York"), standard, |timezone| {
        timezone.write(&LastModified::new("20050809T050000Z"))?;
        timezone.write_daylight(daylight)
    });
    calendar.write_timezone(timezone)?;

    let output = calendar.close()?;
    let output = String::from_utf8_lossy(&output);

    assert_eq!(output, expected);
    Ok(())
}

#[test]
fn alarm() -> Result<(), io::Error> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:Mock Calendar\r\n\
                    BEGIN:VEVENT\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19970901T130000Z\r\n\
                    DTSTART:19960918T143000Z\r\n\
                    DTEND:19960920T220000Z\r\n\
                    BEGIN:VALARM\r\n\
                    ACTION:EMAIL\r\n\
                    TRIGGER;RELATED=END:-P2D\r\n\
                    DESCRIPTION:A draft agenda needs to be sent out to the attendees to the wee\r\n kly managers meeting (MGR-LIST). Attached is a pointer the document templat\r\n e for the agenda file.\r\n\
                    SUMMARY:*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***\r\n\
                    ATTENDEE:mailto:john_doe@example.com\r\n\
                    ATTACH;FMTTYPE=application/msword:http://example.com/templates/agenda.doc\r\n\
                    END:VALARM\r\n\
                    END:VEVENT\r\n\
                    END:VCALENDAR\r\n";

    let mut calendar = calendar(expected.len())?;

    let event = Event::new(
        UID::new("b68378cf-872d-44f1-9703-5e3725c56e71"),
        DtStamp::new("19970901T130000Z"),
        |event| {
            event.write(&DtStart::new("19960918T143000Z"))?;
            event.write(&DtEnd::new("19960920T220000Z"))?;

            let mut trigger = Trigger::new("-P2D");
            trigger.add(Related::End);
            event.write_alarm(Alarm::email(
                trigger,
                Description::new(
                    "A draft agenda needs to be sent out to the attendees to the weekly managers meeting (MGR-LIST). Attached is a pointer the document template for the agenda file."
                ),
                Summary::new("*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***"),
                |alarm| {
                    alarm.write(&Attendee::new("mailto:john_doe@example.com"))?;
                    let mut attach = Attach::new("http://example.com/templates/agenda.doc");
                    attach.add(FmtType::new("application/msword"));
                    alarm.write(&attach)
                }
            ))
        }
    );
    calendar.write_event(event)?;

    let output = calendar.close()?;
    let output = String::from_utf8_lossy(&output);

    assert_eq!(output, expected);
    Ok(())
}
