extern crate ics;

use ics::escape_text;
use ics::parameters::{FmtType, PartStat};
use ics::properties::{
    Attach, Attendee, Categories, Class, Description, DtEnd, DtStart, Due, Duration, FreeBusyTime,
    Organizer, Repeat, Sequence, Status, Summary, Trigger, URL
};
use ics::writer::*;

#[test]
fn event() -> std::io::Result<()> {
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
                    DESCRIPTION:Networld+Interop Conference and Exhibit\n\
                    Atlanta World Congress \r\n Center\n\
                    Atlanta\\, Georgia\r\n\
                    END:VEVENT\r\n\
                    END:VCALENDAR\r\n";

    let event = |event: &mut EventWriter<'_, _>| {
        event.write(&Organizer::new("mailto:jsmith@example.com"))?;
        event.write(&DtStart::new("19960918T143000Z"))?;
        event.write(&DtEnd::new("19960920T220000Z"))?;
        event.write(&Status::confirmed())?;
        event.write(&Categories::new("CONFERENCE"))?;
        event.write(&Summary::new("Networld+Interop Conference"))?;
        event.write(&Description::new(escape_text(
            "Networld+Interop Conference and Exhibit\n\
            Atlanta World Congress Center\n\
            Atlanta, Georgia"
        )))
    };

    let mut calendar = CalendarWriter::new(
        Vec::with_capacity(expected.len()),
        "2.0",
        "-//xyz Corp//NONSGML PDA Calendar Version 1.0//EN"
    )?;

    calendar.write_event(
        "b68378cf-872d-44f1-9703-5e3725c56e71",
        "19960704T120000Z",
        event
    )?;

    let output = calendar.close()?;
    assert_eq!(String::from_utf8_lossy(&output), expected);

    Ok(())
}

#[test]
fn todo() -> std::io::Result<()> {
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

    let todo = |todo: &mut TodoWriter<'_, _>| {
        todo.write(&Organizer::new("mailto:unclesam@example.com"))?;
        let mut attendee = Attendee::new("mailto:jqpublic@example.com");
        attendee.add(PartStat::ACCEPTED);
        todo.write(&attendee)?;
        todo.write(&Due::new("19980415T000000"))?;
        todo.write(&Status::needs_action())?;
        todo.write(&Summary::new("Submit Income Taxes"))?;
        todo.write(&Sequence::new(2))?;

        todo.write_audio_alarm(&Trigger::new("19980403T120000Z"), |alarm| {
            let mut attach = Attach::new("http://example.com/pub/audio-files/ssbanner.aud");
            attach.add(FmtType::new("audio/basic"));
            alarm.write(&attach)?;
            alarm.write(&Repeat::new(4))?;
            alarm.write(&Duration::new("PT1H"))
        })
    };

    let mut calendar = CalendarWriter::new(
        Vec::with_capacity(expected.len()),
        "2.0",
        "-//ABC Corporation//NONSGML My Product//EN"
    )?;
    calendar.write_todo(
        "b68378cf-872d-44f1-9703-5e3725c56e71",
        "19980130T134500Z",
        todo
    )?;

    let output = calendar.close()?;
    assert_eq!(String::from_utf8_lossy(&output), expected);

    Ok(())
}

#[test]
fn journal() -> std::io::Result<()> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:-//ABC Corporation//NONSGML My Product//EN\r\n\
                    BEGIN:VJOURNAL\r\n\
                    UID:b4035e76-699e-4cb2-85f6-724d01f18284\r\n\
                    DTSTAMP:19970324T120000Z\r\n\
                    ORGANIZER:mailto:jsmith@example.com\r\n\
                    STATUS:DRAFT\r\n\
                    CLASS:PUBLIC\r\n\
                    CATEGORIES:Project Report,XYZ,Weekly Meeting\r\n\
                    DESCRIPTION:Project xyz Review Meeting Minutes\n\
                    Agenda\n\
                    1. Review of project \r\n version 1.0 requirements.\n\
                    2. Definition of project processes.\n\
                    3. Review of \r\n project schedule.\n\
                    Participants: John Smith\\, Jane Doe\\, Jim Dandy\n\
                    -It was d\r\n ecided that the requirements need to be signed off by product marketing.\n\
                    -P\r\n roject processes were accepted.\n\
                    -Project schedule needs to account for sche\r\n duled holidays and employee vacation time. Check with HR for specific dates\r\n .\n\
                    -New schedule will be distributed by Friday.\n\
                    -Next weeks meeting is cance\r\n lled. No meeting until 3/23.\r\n\
                    END:VJOURNAL\r\n\
                    END:VCALENDAR\r\n";

    let journal = |journal: &mut JournalWriter<'_, _>| {
        journal.write(&Organizer::new("mailto:jsmith@example.com"))?;
        journal.write(&Status::draft())?;
        journal.write(&Class::public())?;
        journal.write(&Categories::new("Project Report,XYZ,Weekly Meeting"))?;
        journal.write(&Description::new(escape_text("Project xyz Review Meeting Minutes\n\
            Agenda\n\
            1. Review of project version 1.0 requirements.\n\
            2. Definition of project processes.\n\
            3. Review of project schedule.\n\
            Participants: John Smith, Jane Doe, Jim Dandy\n\
            -It was decided that the requirements need to be signed off by product marketing.\n\
            -Project processes were accepted.\n\
            -Project schedule needs to account for scheduled holidays and employee vacation time. Check with HR for specific dates.\n\
            -New schedule will be distributed by Friday.\n\
            -Next weeks meeting is cancelled. No meeting until 3/23."
        )))
    };

    let mut calendar = CalendarWriter::new(
        Vec::with_capacity(expected.len()),
        "2.0",
        "-//ABC Corporation//NONSGML My Product//EN"
    )?;
    calendar.write_journal(
        "b4035e76-699e-4cb2-85f6-724d01f18284",
        "19970324T120000Z",
        journal
    )?;

    let output = calendar.close()?;
    assert_eq!(String::from_utf8_lossy(&output), expected);

    Ok(())
}

#[test]
fn freebusy() -> std::io::Result<()> {
    let expected = "BEGIN:VCALENDAR\r\n\
                    VERSION:2.0\r\n\
                    PRODID:-//RDU Software//NONSGML HandCal//EN\r\n\
                    BEGIN:VFREEBUSY\r\n\
                    UID:0b04bd52-c396-4251-a673-1cc0b96def93\r\n\
                    DTSTAMP:19970324T120000Z\r\n\
                    ORGANIZER:mailto:jsmith@example.com\r\n\
                    DTSTART:19980313T141711Z\r\n\
                    DTEND:19980410T141711Z\r\n\
                    FREEBUSY:19980314T233000Z/19980315T003000Z\r\n\
                    FREEBUSY:19980316T153000Z/19980316T163000Z\r\n\
                    FREEBUSY:19980318T030000Z/19980318T040000Z\r\n\
                    URL:http://www.example.com/calendar/busytime/jsmith.ifb\r\n\
                    END:VFREEBUSY\r\n\
                    END:VCALENDAR\r\n";

    let freebusy = |freebusy: &mut FreeBusyWriter<'_, _>| {
        freebusy.write(&Organizer::new("mailto:jsmith@example.com"))?;
        freebusy.write(&DtStart::new("19980313T141711Z"))?;
        freebusy.write(&DtEnd::new("19980410T141711Z"))?;
        freebusy.write(&FreeBusyTime::new("19980314T233000Z/19980315T003000Z"))?;
        freebusy.write(&FreeBusyTime::new("19980316T153000Z/19980316T163000Z"))?;
        freebusy.write(&FreeBusyTime::new("19980318T030000Z/19980318T040000Z"))?;
        freebusy.write(&URL::new(
            "http://www.example.com/calendar/busytime/jsmith.ifb"
        ))
    };

    let mut calendar = CalendarWriter::new(
        Vec::with_capacity(expected.len()),
        "2.0",
        "-//RDU Software//NONSGML HandCal//EN"
    )?;
    calendar.write_free_busy(
        "0b04bd52-c396-4251-a673-1cc0b96def93",
        "19970324T120000Z",
        freebusy
    )?;

    let output = calendar.close()?;
    assert_eq!(String::from_utf8_lossy(&output), expected);

    Ok(())
}