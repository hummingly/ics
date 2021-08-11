extern crate ics;

use ics::parameters::{FmtType, Related, Value};
use ics::properties::{
    Attach, Attendee, Categories, Class, Completed, Description, DtEnd, DtStart, Due, FreeBusyTime,
    LastModified, Organizer, Priority, RRule, Status, Summary, Transp, Trigger, TzName, URL
};
use ics::{escape_text, Alarm, Daylight, Event, FreeBusy, Journal, Standard, TimeZone, ToDo};

#[test]
fn event() {
    let expected = "BEGIN:VEVENT\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19970901T130000Z\r\n\
                    CATEGORIES:ANNIVERSARY,PERSONAL,SPECIAL OCCASION\r\n\
                    CLASS:CONFIDENTIAL\r\n\
                    DTSTART;VALUE=DATE:19971102\r\n\
                    RRULE:FREQ=YEARLY\r\n\
                    SUMMARY:Our Blissful Anniversary\r\n\
                    TRANSP:TRANSPARENT\r\n\
                    END:VEVENT\r\n";

    let mut event = Event::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19970901T130000Z");
    event.push(Categories::new("ANNIVERSARY,PERSONAL,SPECIAL OCCASION"));
    event.push(Class::confidential());
    let mut date = DtStart::new("19971102");
    date.add(Value::DATE);
    event.push(date);
    event.push(RRule::new("FREQ=YEARLY"));
    event.push(Summary::new("Our Blissful Anniversary"));
    event.push(Transp::transparent());

    assert_eq!(event.to_string(), expected);
}

#[test]
fn todo() {
    let expected = "BEGIN:VTODO\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:20070514T103211Z\r\n\
                    COMPLETED:20070707T100000Z\r\n\
                    DTSTART:20070514T110000Z\r\n\
                    DUE:20070709T130000Z\r\n\
                    PRIORITY:1\r\n\
                    SUMMARY:Submit Revised Internet-Draft\r\n\
                    STATUS:NEEDS-ACTION\r\n\
                    END:VTODO\r\n";

    let mut todo = ToDo::new("b68378cf-872d-44f1-9703-5e3725c56e71", "20070514T103211Z");
    todo.push(Completed::new("20070707T100000Z"));
    todo.push(DtStart::new("20070514T110000Z"));
    todo.push(Due::new("20070709T130000Z"));
    todo.push(Priority::new("1"));
    todo.push(Summary::new("Submit Revised Internet-Draft"));
    todo.push(Status::needs_action());

    assert_eq!(todo.to_string(), expected);
}

#[test]
fn journal() {
    let expected = "BEGIN:VJOURNAL\r\n\
    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
    DTSTAMP:19970901T130000Z\r\n\
    DTSTART;VALUE=DATE:19970317\r\n\
    SUMMARY:Staff meeting minutes\r\n\
    DESCRIPTION:1. Staff meeting: Participants include Joe\\, Lisa\\, and Bob. Au\r\n rora project plans were reviewed. There is currently no budget reserves for\r\n  this project. Lisa will escalate to management. Next meeting on Tuesday.\\n\
    \r\n 2. Telephone Conference: ABC Corp. sales representative called to discuss n\r\n ew printer. Promised to get us a demo by Friday.\\n\
    3. Henry Miller (Handsoff\r\n  Insurance): Car was totaled by tree. Is looking into a loaner car. 555-232\r\n 3 (tel).\r\n\
    END:VJOURNAL\r\n";

    let mut journal = Journal::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19970901T130000Z");
    let mut date = DtStart::new("19970317");
    date.add(Value::DATE);
    journal.push(date);
    journal.push(Summary::new("Staff meeting minutes"));
    journal.push(Description::new(escape_text("1. Staff meeting: Participants include Joe, Lisa, and Bob. Aurora project plans were reviewed. There is currently no budget reserves for this project. Lisa will escalate to management. Next meeting on Tuesday.\n\
    2. Telephone Conference: ABC Corp. sales representative called to discuss new printer. Promised to get us a demo by Friday.\n\
    3. Henry Miller (Handsoff Insurance): Car was totaled by tree. Is looking into a loaner car. 555-2323 (tel).")));

    assert_eq!(journal.to_string(), expected);
}

#[test]
fn freebusy() {
    let expected = "BEGIN:VFREEBUSY\r\n\
                    UID:b68378cf-872d-44f1-9703-5e3725c56e71\r\n\
                    DTSTAMP:19970901T120000Z\r\n\
                    DTSTART:19980313T141711Z\r\n\
                    DTEND:19980410T141711Z\r\n\
                    FREEBUSY:19980314T233000Z/19980315T003000Z\r\n\
                    FREEBUSY:19980316T153000Z/19980316T163000Z\r\n\
                    FREEBUSY:19980318T030000Z/19980318T040000Z\r\n\
                    ORGANIZER:jsmith@example.com\r\n\
                    URL:http://www.example.com/calendar/busytime/jsmith.ifb\r\n\
                    END:VFREEBUSY\r\n";

    let mut freebusy = FreeBusy::new("b68378cf-872d-44f1-9703-5e3725c56e71", "19970901T120000Z");
    freebusy.push(DtStart::new("19980313T141711Z"));
    freebusy.push(DtEnd::new("19980410T141711Z"));
    freebusy.push(FreeBusyTime::new("19980314T233000Z/19980315T003000Z"));
    freebusy.push(FreeBusyTime::new("19980316T153000Z/19980316T163000Z"));
    freebusy.push(FreeBusyTime::new("19980318T030000Z/19980318T040000Z"));
    freebusy.push(Organizer::new("jsmith@example.com"));
    freebusy.push(URL::new(
        "http://www.example.com/calendar/busytime/jsmith.ifb"
    ));

    assert_eq!(freebusy.to_string(), expected);
}

#[test]
fn time() {
    let expected = "BEGIN:VTIMEZONE\r\n\
                    TZID:America/New_York\r\n\
                    LAST-MODIFIED:20050809T050000Z\r\n\
                    BEGIN:STANDARD\r\n\
                    DTSTART:20071104T020000\r\n\
                    TZOFFSETFROM:-0400\r\n\
                    TZOFFSETTO:-0500\r\n\
                    TZNAME:EST\r\n\
                    END:STANDARD\r\n\
                    BEGIN:DAYLIGHT\r\n\
                    DTSTART:20070311T020000\r\n\
                    TZOFFSETFROM:-0500\r\n\
                    TZOFFSETTO:-0400\r\n\
                    TZNAME:EDT\r\n\
                    END:DAYLIGHT\r\n\
                    END:VTIMEZONE\r\n";

    let mut standard = Standard::new("20071104T020000", "-0400", "-0500");
    standard.push(TzName::new("EST"));
    let mut daylight = Daylight::new("20070311T020000", "-0500", "-0400");
    daylight.push(TzName::new("EDT"));

    let mut timezone = TimeZone::standard("America/New_York", standard);
    timezone.push(LastModified::new("20050809T050000Z"));
    timezone.add_daylight(daylight);

    assert_eq!(timezone.to_string(), expected);
}

#[test]
fn alarm() {
    let expected = "BEGIN:VALARM\r\n\
                    ACTION:EMAIL\r\n\
                    TRIGGER;RELATED=END:-P2D\r\n\
                    DESCRIPTION:A draft agenda needs to be sent out to the attendees to the wee\r\n kly managers meeting (MGR-LIST). Attached is a pointer the document templat\r\n e for the agenda file.\r\n\
                    SUMMARY:*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***\r\n\
                    ATTENDEE:mailto:john_doe@example.com\r\n\
                    ATTACH;FMTTYPE=application/msword:http://example.com/templates/agenda.doc\r\n\
                    END:VALARM\r\n";

    let mut trigger = Trigger::new("-P2D");
    trigger.add(Related::End);
    let mut alarm = Alarm::email(trigger, Description::new("A draft agenda needs to be sent out to the attendees to the weekly managers meeting (MGR-LIST). Attached is a pointer the document template for the agenda file."), Summary::new("*** REMINDER: SEND AGENDA FOR WEEKLY STAFF MEETING ***"));
    alarm.push(Attendee::new("mailto:john_doe@example.com"));
    let mut attach = Attach::new("http://example.com/templates/agenda.doc");
    attach.add(FmtType::new("application/msword"));
    alarm.push(attach);

    assert_eq!(alarm.to_string(), expected);
}
