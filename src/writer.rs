pub use crate::contentline::{ContentLineWriter, PropertyWrite};
use crate::properties::{
    Action, Description, DtStamp, DtStart, ProdID, Summary, Trigger, TzID, TzOffsetFrom,
    TzOffsetTo, Version, UID
};
use std::io::{Error, Write};

pub const VCALENDAR: &str = "VCALENDAR";
pub const VEVENT: &str = "VEVENT";
pub const VTODO: &str = "VTODO";
pub const VJOURNAL: &str = "VJOURNAL";
pub const VFREEBUSY: &str = "VFREEBUSY";
pub const VALARM: &str = "VALARM";
pub const VTIMEZONE: &str = "VTIMEZONE";
pub const STANDARD: &str = "STANDARD";
pub const DAYLIGHT: &str = "DAYLIGHT";

#[derive(Debug)]
pub struct ICalendar<W: Write>(ContentLineWriter<W>);

impl<W: Write> ICalendar<W> {
    pub fn new(inner: W, version: Version, product_id: ProdID) -> Result<ICalendar<W>, Error> {
        let mut writer = ContentLineWriter::new(inner);
        writer.write_begin_unchecked(VCALENDAR)?;
        writer.write_property(&version)?;
        writer.write_property(&product_id)?;
        Ok(Self(writer))
    }

    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_component(
        &mut self,
        name: &str,
        body: impl FnOnce(&mut Self) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin(name)?;
        body(self)?;
        self.0.write_end(name)
    }

    pub fn write_event(
        &mut self,
        event: impl FnOnce(&mut EventWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VEVENT)?;
        (event)(&mut EventWriter(&mut self.0))?;
        self.0.write_end_unchecked(VEVENT)
    }

    pub fn write_todo(
        &mut self,
        todo: impl FnOnce(&mut ToDoWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VTODO)?;
        (todo)(&mut ToDoWriter(&mut self.0))?;
        self.0.write_end_unchecked(VTODO)
    }

    pub fn write_journal(
        &mut self,
        journal: impl FnOnce(&mut JournalWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VJOURNAL)?;
        (journal)(&mut JournalWriter(&mut self.0))?;
        self.0.write_end_unchecked(VJOURNAL)
    }

    pub fn write_freebusy(
        &mut self,
        freebusy: impl FnOnce(&mut FreeBusyWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VFREEBUSY)?;
        (freebusy)(&mut FreeBusyWriter(&mut self.0))?;
        self.0.write_end_unchecked(VFREEBUSY)
    }

    pub fn write_timezone(
        &mut self,
        timezone: impl FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VTIMEZONE)?;
        (timezone)(&mut TimeZoneWriter(&mut self.0))?;
        self.0.write_end_unchecked(VTIMEZONE)
    }

    pub fn close(mut self) -> Result<W, Error> {
        self.0.write_end_unchecked(VCALENDAR)?;
        self.0.into_inner()
    }
}

pub struct Event;

impl Event {
    pub fn build<'e, W: Write>(
        uid: UID<'e>,
        dt_stamp: DtStamp<'e>,
        body: impl FnOnce(&mut EventWriter<W>) -> Result<(), Error> + 'e
    ) -> impl FnOnce(&mut EventWriter<W>) -> Result<(), Error> + 'e {
        move |event| {
            event.write(&uid)?;
            event.write(&dt_stamp)?;
            body(event)
        }
    }
}

pub struct ToDo;

impl ToDo {
    pub fn build<'t, W: Write>(
        uid: UID<'t>,
        dt_stamp: DtStamp<'t>,
        body: impl FnOnce(&mut ToDoWriter<W>) -> Result<(), Error> + 't
    ) -> impl FnOnce(&mut ToDoWriter<W>) -> Result<(), Error> + 't {
        move |todo| {
            todo.write(&uid)?;
            todo.write(&dt_stamp)?;
            body(todo)
        }
    }
}

pub struct Journal;

impl Journal {
    pub fn build<'j, W: Write>(
        uid: UID<'j>,
        dt_stamp: DtStamp<'j>,
        body: impl FnOnce(&mut JournalWriter<W>) -> Result<(), Error> + 'j
    ) -> impl FnOnce(&mut JournalWriter<W>) -> Result<(), Error> + 'j {
        move |journal| {
            journal.write(&uid)?;
            journal.write(&dt_stamp)?;
            body(journal)
        }
    }
}

pub struct FreeBusy;

impl FreeBusy {
    pub fn build<'f, W: Write>(
        uid: UID<'f>,
        dt_stamp: DtStamp<'f>,
        body: impl FnOnce(&mut FreeBusyWriter<W>) -> Result<(), Error> + 'f
    ) -> impl FnOnce(&mut FreeBusyWriter<W>) -> Result<(), Error> + 'f {
        move |freebusy| {
            freebusy.write(&uid)?;
            freebusy.write(&dt_stamp)?;
            body(freebusy)
        }
    }
}

pub struct TimeZone;

impl TimeZone {
    pub fn standard<'t, W: Write + 't>(
        tzid: TzID<'t>,
        definition: impl FnOnce(&mut StandardWriter<W>) -> Result<(), Error> + 't,
        body: impl FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't
    ) -> impl FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't {
        move |timezone| {
            timezone.write(&tzid)?;
            timezone.write_standard(definition)?;
            body(timezone)
        }
    }

    pub fn daylight<'t, W: Write + 't>(
        tzid: TzID<'t>,
        definition: impl FnOnce(&mut DaylightWriter<W>) -> Result<(), Error> + 't,
        body: impl FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't
    ) -> impl FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't {
        move |timezone| {
            timezone.write(&tzid)?;
            timezone.write_daylight(definition)?;
            body(timezone)
        }
    }
}

pub struct Alarm;

impl Alarm {
    pub fn build<'a, W: Write>(
        action: Action<'a>,
        trigger: Trigger<'a>,
        body: impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    ) -> impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&action)?;
            alarm.write(&trigger)?;
            body(alarm)
        }
    }

    pub fn audio<'a, W: Write>(
        trigger: Trigger<'a>,
        body: impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    ) -> impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&Action::audio())?;
            alarm.write(&trigger)?;
            body(alarm)
        }
    }

    pub fn display<'a, W: Write>(
        trigger: Trigger<'a>,
        description: Description<'a>,
        body: impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    ) -> impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&Action::display())?;
            alarm.write(&trigger)?;
            alarm.write(&description)?;
            body(alarm)
        }
    }

    pub fn email<'a, W: Write>(
        trigger: Trigger<'a>,
        description: Description<'a>,
        summary: Summary<'a>,
        body: impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    ) -> impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a {
        move |alarm| {
            alarm.write(&Action::email())?;
            alarm.write(&trigger)?;
            alarm.write(&description)?;
            alarm.write(&summary)?;
            body(alarm)
        }
    }
}

pub struct Standard;

impl Standard {
    pub fn build<'s, W: Write>(
        dt_start: DtStart<'s>,
        tz_offset_from: TzOffsetFrom<'s>,
        tz_offset_to: TzOffsetTo<'s>,
        body: impl FnOnce(&mut StandardWriter<W>) -> Result<(), Error> + 's
    ) -> impl FnOnce(&mut StandardWriter<W>) -> Result<(), Error> + 's {
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
    pub fn build<'d, W: Write>(
        dt_start: DtStart<'d>,
        tz_offset_from: TzOffsetFrom<'d>,
        tz_offset_to: TzOffsetTo<'d>,
        body: impl FnOnce(&mut DaylightWriter<W>) -> Result<(), Error> + 'd
    ) -> impl FnOnce(&mut DaylightWriter<W>) -> Result<(), Error> + 'd {
        move |daylight| {
            daylight.write(&dt_start)?;
            daylight.write(&tz_offset_from)?;
            daylight.write(&tz_offset_to)?;
            body(daylight)
        }
    }
}

#[derive(Debug)]
pub struct EventWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> EventWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_alarm(
        &mut self,
        alarm: impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VALARM)?;
        (alarm)(&mut AlarmWriter(self.0))?;
        self.0.write_end_unchecked(VALARM)
    }
}

#[derive(Debug)]
pub struct ToDoWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> ToDoWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_alarm(
        &mut self,
        alarm: impl FnOnce(&mut AlarmWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VALARM)?;
        (alarm)(&mut AlarmWriter(self.0))?;
        self.0.write_end_unchecked(VALARM)
    }
}

#[derive(Debug)]
pub struct JournalWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> JournalWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct FreeBusyWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> FreeBusyWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct TimeZoneWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> TimeZoneWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_standard(
        &mut self,
        definition: impl FnOnce(&mut StandardWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(STANDARD)?;
        (definition)(&mut StandardWriter(self.0))?;
        self.0.write_end_unchecked(STANDARD)
    }

    pub fn write_daylight(
        &mut self,
        definition: impl FnOnce(&mut DaylightWriter<W>) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(DAYLIGHT)?;
        (definition)(&mut DaylightWriter(self.0))?;
        self.0.write_end_unchecked(DAYLIGHT)
    }
}

#[derive(Debug)]
pub struct AlarmWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> AlarmWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct StandardWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> StandardWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct DaylightWriter<'w, W: Write>(&'w mut ContentLineWriter<W>);

impl<W: Write> DaylightWriter<'_, W> {
    pub fn write(&mut self, property: &impl PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}
