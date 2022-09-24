//! Factory functions to create component functions.

use std::io::Error;

use crate::{
    properties::{
        Action, Description, DtStamp, DtStart, Summary, Trigger, TzID, TzOffsetFrom, TzOffsetTo,
        UID,
    },
    writer::{
        AlarmWriter, DaylightWriter, EventWriter, FreeBusyWriter, JournalWriter, StandardWriter,
        TimeZoneWriter, ToDoWriter,
    },
};

pub struct Event;

impl Event {
    pub fn build<'e>(
        uid: UID<'e>,
        dt_stamp: DtStamp<'e>,
        body: impl FnOnce(&mut EventWriter) -> Result<(), Error> + 'e,
    ) -> impl FnOnce(&mut EventWriter) -> Result<(), Error> + 'e {
        move |event| {
            event.write(&uid)?;
            event.write(&dt_stamp)?;
            body(event)
        }
    }
}

pub struct ToDo;

impl ToDo {
    pub fn build<'t>(
        uid: UID<'t>,
        dt_stamp: DtStamp<'t>,
        body: impl FnOnce(&mut ToDoWriter) -> Result<(), Error> + 't,
    ) -> impl FnOnce(&mut ToDoWriter) -> Result<(), Error> + 't {
        move |todo| {
            todo.write(&uid)?;
            todo.write(&dt_stamp)?;
            body(todo)
        }
    }
}

pub struct Journal;

impl Journal {
    pub fn build<'j>(
        uid: UID<'j>,
        dt_stamp: DtStamp<'j>,
        body: impl FnOnce(&mut JournalWriter) -> Result<(), Error> + 'j,
    ) -> impl FnOnce(&mut JournalWriter) -> Result<(), Error> + 'j {
        move |journal| {
            journal.write(&uid)?;
            journal.write(&dt_stamp)?;
            body(journal)
        }
    }
}

pub struct FreeBusy;

impl FreeBusy {
    pub fn build<'f>(
        uid: UID<'f>,
        dt_stamp: DtStamp<'f>,
        body: impl FnOnce(&mut FreeBusyWriter) -> Result<(), Error> + 'f,
    ) -> impl FnOnce(&mut FreeBusyWriter) -> Result<(), Error> + 'f {
        move |freebusy| {
            freebusy.write(&uid)?;
            freebusy.write(&dt_stamp)?;
            body(freebusy)
        }
    }
}

pub struct TimeZone;

impl TimeZone {
    pub fn standard<'t>(
        tzid: TzID<'t>,
        definition: impl FnOnce(&mut StandardWriter) -> Result<(), Error> + 't,
        body: impl FnOnce(&mut TimeZoneWriter) -> Result<(), Error> + 't,
    ) -> impl FnOnce(&mut TimeZoneWriter) -> Result<(), Error> + 't {
        move |timezone| {
            timezone.write(&tzid)?;
            timezone.write_standard(definition)?;
            body(timezone)
        }
    }

    pub fn daylight<'t>(
        tzid: TzID<'t>,
        definition: impl FnOnce(&mut DaylightWriter) -> Result<(), Error> + 't,
        body: impl FnOnce(&mut TimeZoneWriter) -> Result<(), Error> + 't,
    ) -> impl FnOnce(&mut TimeZoneWriter) -> Result<(), Error> + 't {
        move |timezone| {
            timezone.write(&tzid)?;
            timezone.write_daylight(definition)?;
            body(timezone)
        }
    }
}

pub struct Alarm;

impl Alarm {
    pub fn build<'a>(
        action: Action<'a>,
        trigger: Trigger<'a>,
        body: impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a,
    ) -> impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&action)?;
            alarm.write(&trigger)?;
            body(alarm)
        }
    }

    pub fn audio<'a>(
        trigger: Trigger<'a>,
        body: impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a,
    ) -> impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&Action::AUDIO)?;
            alarm.write(&trigger)?;
            body(alarm)
        }
    }

    pub fn display<'a>(
        trigger: Trigger<'a>,
        description: Description<'a>,
        body: impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a,
    ) -> impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&Action::DISPLAY)?;
            alarm.write(&trigger)?;
            alarm.write(&description)?;
            body(alarm)
        }
    }

    pub fn email<'a>(
        trigger: Trigger<'a>,
        description: Description<'a>,
        summary: Summary<'a>,
        body: impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a,
    ) -> impl FnOnce(&mut AlarmWriter) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&Action::EMAIL)?;
            alarm.write(&trigger)?;
            alarm.write(&description)?;
            alarm.write(&summary)?;
            body(alarm)
        }
    }
}

pub struct Standard;

impl Standard {
    pub fn build<'s>(
        dt_start: DtStart<'s>,
        tz_offset_from: TzOffsetFrom<'s>,
        tz_offset_to: TzOffsetTo<'s>,
        body: impl FnOnce(&mut StandardWriter) -> Result<(), Error> + 's,
    ) -> impl FnOnce(&mut StandardWriter) -> Result<(), Error> + 's {
        move |standard| {
            standard.write(&dt_start)?;
            standard.write(&tz_offset_from)?;
            standard.write(&tz_offset_to)?;
            body(standard)
        }
    }
}

pub struct Daylight;

impl Daylight {
    pub fn build<'d>(
        dt_start: DtStart<'d>,
        tz_offset_from: TzOffsetFrom<'d>,
        tz_offset_to: TzOffsetTo<'d>,
        body: impl FnOnce(&mut DaylightWriter) -> Result<(), Error> + 'd,
    ) -> impl FnOnce(&mut DaylightWriter) -> Result<(), Error> + 'd {
        move |daylight| {
            daylight.write(&dt_start)?;
            daylight.write(&tz_offset_from)?;
            daylight.write(&tz_offset_to)?;
            body(daylight)
        }
    }
}
