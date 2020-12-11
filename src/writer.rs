use crate::contentline::{ContentLine, PropertyWrite, Writer};
use crate::properties::{
    Action, Description, DtStamp, DtStart, ProdID, Summary, Trigger, TzID, TzOffsetFrom,
    TzOffsetTo, Version, UID
};
use std::io::{Error, Write};

const VCALENDAR: &str = "VCALENDAR";
const VEVENT: &str = "VEVENT";
const VTODO: &str = "VTODO";
const VJOURNAL: &str = "VJOURNAL";
const VFREEBUSY: &str = "VFREEBUSY";
const VALARM: &str = "VALARM";
const VTIMEZONE: &str = "VTIMEZONE";
const STANDARD: &str = "STANDARD";
const DAYLIGHT: &str = "DAYLIGHT";

#[derive(Debug)]
pub struct ICalendar<W: Write>(Writer<W>);

impl<W: Write> ICalendar<W> {
    pub fn new(inner: W, version: Version, product_id: ProdID) -> Result<ICalendar<W>, Error> {
        let mut writer = Self(Writer::new(inner));
        writer.0.write_begin_unchecked(VCALENDAR)?;
        writer.write(&version)?;
        writer.write(&product_id)?;
        Ok(writer)
    }

    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_component<F>(&mut self, name: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Self) -> Result<(), Error>
    {
        self.0.write_begin(name)?;
        body(self)?;
        self.0.write_end(name)
    }

    pub fn write_event(&mut self, event: Event<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VEVENT)?;
        (event.0)(&mut EventWriter(&mut self.0))?;
        self.0.write_end_unchecked(VEVENT)
    }

    pub fn write_todo(&mut self, todo: ToDo<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VTODO)?;
        (todo.0)(&mut ToDoWriter(&mut self.0))?;
        self.0.write_end_unchecked(VTODO)
    }

    pub fn write_journal(&mut self, journal: Journal<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VJOURNAL)?;
        (journal.0)(&mut JournalWriter(&mut self.0))?;
        self.0.write_end_unchecked(VJOURNAL)
    }

    pub fn write_freebusy(&mut self, freebusy: FreeBusy<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VFREEBUSY)?;
        (freebusy.0)(&mut FreeBusyWriter(&mut self.0))?;
        self.0.write_end_unchecked(VFREEBUSY)
    }

    pub fn write_timezone(&mut self, timezone: TimeZone<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VTIMEZONE)?;
        (timezone.0)(&mut TimeZoneWriter(&mut self.0))?;
        self.0.write_end_unchecked(VTIMEZONE)
    }

    pub fn close(mut self) -> Result<W, Error> {
        self.0.write_end_unchecked(VCALENDAR)?;
        self.0.into_inner()
    }
}

pub struct Event<'e, W: Write>(Box<dyn FnOnce(&mut EventWriter<W>) -> Result<(), Error> + 'e>);

impl<'e, W: Write> Event<'e, W> {
    pub fn new<F>(uid: UID<'e>, dt_stamp: DtStamp<'e>, body: F) -> Self
    where
        F: FnOnce(&mut EventWriter<W>) -> Result<(), Error> + 'e
    {
        Self(Box::new(move |event| {
            event.write(&uid)?;
            event.write(&dt_stamp)?;
            body(event)
        }))
    }
}

pub struct ToDo<'t, W: Write>(Box<dyn FnOnce(&mut ToDoWriter<W>) -> Result<(), Error> + 't>);

impl<'t, W: Write> ToDo<'t, W> {
    pub fn new<F>(uid: UID<'t>, dt_stamp: DtStamp<'t>, body: F) -> Self
    where
        F: FnOnce(&mut ToDoWriter<W>) -> Result<(), Error> + 't
    {
        Self(Box::new(move |todo| {
            todo.write(&uid)?;
            todo.write(&dt_stamp)?;
            body(todo)
        }))
    }
}

pub struct Journal<'j, W: Write>(Box<dyn FnOnce(&mut JournalWriter<W>) -> Result<(), Error> + 'j>);

impl<'j, W: Write> Journal<'j, W> {
    pub fn new<F>(uid: UID<'j>, dt_stamp: DtStamp<'j>, body: F) -> Self
    where
        F: FnOnce(&mut JournalWriter<W>) -> Result<(), Error> + 'j
    {
        Self(Box::new(move |journal| {
            journal.write(&uid)?;
            journal.write(&dt_stamp)?;
            body(journal)
        }))
    }
}

pub struct FreeBusy<'f, W: Write>(
    Box<dyn FnOnce(&mut FreeBusyWriter<W>) -> Result<(), Error> + 'f>
);

impl<'f, W: Write> FreeBusy<'f, W> {
    pub fn new<F>(uid: UID<'f>, dt_stamp: DtStamp<'f>, body: F) -> Self
    where
        F: FnOnce(&mut FreeBusyWriter<W>) -> Result<(), Error> + 'f
    {
        Self(Box::new(move |freebusy| {
            freebusy.write(&uid)?;
            freebusy.write(&dt_stamp)?;
            body(freebusy)
        }))
    }
}

pub struct TimeZone<'t, W: Write>(
    Box<dyn FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't>
);

impl<'t, W: Write + 't> TimeZone<'t, W> {
    pub fn standard<F>(tzid: TzID<'t>, definition: Standard<'t, W>, body: F) -> Self
    where
        F: FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't
    {
        Self(Box::new(move |timezone| {
            timezone.write(&tzid)?;
            timezone.write_standard(definition)?;
            body(timezone)
        }))
    }

    pub fn daylight<F>(tzid: TzID<'t>, definition: Daylight<'t, W>, body: F) -> Self
    where
        F: FnOnce(&mut TimeZoneWriter<W>) -> Result<(), Error> + 't
    {
        Self(Box::new(move |timezone| {
            timezone.write(&tzid)?;
            timezone.write_daylight(definition)?;
            body(timezone)
        }))
    }
}

pub struct Alarm<'a, W: Write>(Box<dyn FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a>);

impl<'a, W: Write> Alarm<'a, W> {
    pub fn new<F>(action: Action<'a>, trigger: Trigger<'a>, body: F) -> Self
    where
        F: FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    {
        Self(Box::new(move |alarm| {
            alarm.write(&action)?;
            alarm.write(&trigger)?;
            body(alarm)
        }))
    }

    pub fn audio<F>(trigger: Trigger<'a>, body: F) -> Self
    where
        F: FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    {
        Self(Box::new(move |alarm| {
            alarm.write(&Action::audio())?;
            alarm.write(&trigger)?;
            body(alarm)
        }))
    }

    pub fn display<F>(trigger: Trigger<'a>, description: Description<'a>, body: F) -> Self
    where
        F: FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    {
        Self(Box::new(move |alarm| {
            alarm.write(&Action::display())?;
            alarm.write(&trigger)?;
            alarm.write(&description)?;
            body(alarm)
        }))
    }

    pub fn email<F>(
        trigger: Trigger<'a>,
        description: Description<'a>,
        summary: Summary<'a>,
        body: F
    ) -> Self
    where
        F: FnOnce(&mut AlarmWriter<W>) -> Result<(), Error> + 'a
    {
        Self(Box::new(move |alarm| {
            alarm.write(&Action::email())?;
            alarm.write(&trigger)?;
            alarm.write(&description)?;
            alarm.write(&summary)?;
            body(alarm)
        }))
    }
}

pub struct Standard<'s, W: Write>(
    Box<dyn FnOnce(&mut StandardWriter<W>) -> Result<(), Error> + 's>
);

impl<'s, W: Write> Standard<'s, W> {
    pub fn new<F>(
        dt_start: DtStart<'s>,
        tz_offset_from: TzOffsetFrom<'s>,
        tz_offset_to: TzOffsetTo<'s>,
        body: F
    ) -> Self
    where
        F: FnOnce(&mut StandardWriter<W>) -> Result<(), Error> + 's
    {
        Self(Box::new(move |standard| {
            standard.write(&dt_start)?;
            standard.write(&tz_offset_from)?;
            standard.write(&tz_offset_to)?;
            body(standard)
        }))
    }
}

pub struct Daylight<'d, W: Write>(
    Box<dyn FnOnce(&mut DaylightWriter<W>) -> Result<(), Error> + 'd>
);

impl<'d, W: Write> Daylight<'d, W> {
    pub fn new<F>(
        dt_start: DtStart<'d>,
        tz_offset_from: TzOffsetFrom<'d>,
        tz_offset_to: TzOffsetTo<'d>,
        body: F
    ) -> Self
    where
        F: FnOnce(&mut DaylightWriter<W>) -> Result<(), Error> + 'd
    {
        Self(Box::new(move |daylight| {
            daylight.write(&dt_start)?;
            daylight.write(&tz_offset_from)?;
            daylight.write(&tz_offset_to)?;
            body(daylight)
        }))
    }
}

#[derive(Debug)]
pub struct EventWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> EventWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm(&mut self, alarm: Alarm<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VALARM)?;
        (alarm.0)(&mut AlarmWriter(self.0))?;
        self.0.write_end_unchecked(VALARM)
    }
}

#[derive(Debug)]
pub struct ToDoWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> ToDoWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm(&mut self, alarm: Alarm<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(VALARM)?;
        (alarm.0)(&mut AlarmWriter(self.0))?;
        self.0.write_end_unchecked(VALARM)
    }
}

#[derive(Debug)]
pub struct JournalWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> JournalWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

#[derive(Debug)]
pub struct FreeBusyWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> FreeBusyWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

#[derive(Debug)]
pub struct TimeZoneWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> TimeZoneWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_standard(&mut self, definition: Standard<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(STANDARD)?;
        (definition.0)(&mut StandardWriter(self.0))?;
        self.0.write_end_unchecked(STANDARD)
    }

    pub fn write_daylight(&mut self, definition: Daylight<W>) -> Result<(), Error> {
        self.0.write_begin_unchecked(DAYLIGHT)?;
        (definition.0)(&mut DaylightWriter(self.0))?;
        self.0.write_end_unchecked(DAYLIGHT)
    }
}

#[derive(Debug)]
pub struct AlarmWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> AlarmWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

#[derive(Debug)]
pub struct StandardWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> StandardWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

#[derive(Debug)]
pub struct DaylightWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> DaylightWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}
