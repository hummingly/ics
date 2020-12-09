#![allow(dead_code)]
use crate::contentline::{ContentLine, PropertyWrite, Writer};
use crate::properties::{
    Action, Description, DtStart, Summary, Trigger, TzID, TzOffsetFrom, TzOffsetTo
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

pub struct CalendarWriter<W: Write>(Writer<W>);

impl<W: Write> CalendarWriter<W> {
    pub fn new(inner: W, version: &str, product_id: &str) -> Result<CalendarWriter<W>, Error> {
        let mut writer = Writer::new(inner);
        writer.write_begin_unchecked(VCALENDAR)?;
        write!(writer, "VERSION:{}", version)?;
        writer.end_line()?;
        write!(writer, "PRODID:{}", product_id)?;
        writer.end_line()?;
        Ok(CalendarWriter(writer))
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

    pub fn write_event<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut EventWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VEVENT)?;
        let mut writer = EventWriter::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VEVENT)
    }

    pub fn write_todo<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut TodoWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VTODO)?;
        let mut writer = TodoWriter::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VTODO)
    }

    pub fn write_journal<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut JournalWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VJOURNAL)?;
        let mut writer = JournalWriter::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VJOURNAL)
    }

    pub fn write_freebusy<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut FreeBusyWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VFREEBUSY)?;
        let mut writer = FreeBusyWriter::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VFREEBUSY)
    }

    pub fn write_timezone<F>(&mut self, tzid: &TzID, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut TimeZoneWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VTIMEZONE)?;
        let mut writer = TimeZoneWriter::new(&mut self.0, tzid)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VTIMEZONE)
    }

    pub fn close(mut self) -> Result<W, Error> {
        self.0.write_end_unchecked(VCALENDAR)?;
        self.0.into_inner()
    }
}

pub struct EventWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> EventWriter<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> EventWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm<F>(
        &mut self,
        action: &Action<'_>,
        trigger: &Trigger<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::new(self.0, action, trigger)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_audio_alarm<F>(&mut self, trigger: &Trigger<'_>, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::audio(self.0, trigger)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_display_alarm<F>(
        &mut self,
        trigger: &Trigger<'_>,
        description: &Description<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::display(self.0, trigger, description)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_email_alarm<F>(
        &mut self,
        trigger: &Trigger<'_>,
        description: &Description<'_>,
        summary: &Summary<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::email(self.0, trigger, description, summary)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }
}

pub struct TodoWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> TodoWriter<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> TodoWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm<F>(
        &mut self,
        action: &Action<'_>,
        trigger: &Trigger<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::new(self.0, action, trigger)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_audio_alarm<F>(&mut self, trigger: &Trigger<'_>, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::audio(self.0, trigger)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_display_alarm<F>(
        &mut self,
        trigger: &Trigger<'_>,
        description: &Description<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::display(self.0, trigger, description)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_email_alarm<F>(
        &mut self,
        trigger: &Trigger<'_>,
        description: &Description<'_>,
        summary: &Summary<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut AlarmWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::email(self.0, trigger, description, summary)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }
}

pub struct JournalWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> JournalWriter<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> JournalWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct FreeBusyWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> FreeBusyWriter<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> FreeBusyWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct TimeZoneWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> TimeZoneWriter<'w, W> {
    fn new(writer: &'w mut Writer<W>, tzid: &TzID) -> Result<Self, Error> {
        let mut timezone = Self(writer);
        timezone.write(tzid)?;
        Ok(timezone)
    }
}

impl<W: Write> TimeZoneWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_standard<F>(
        &mut self,
        dtstart: &DtStart<'_>,
        tz_offset_from: &TzOffsetFrom<'_>,
        tz_offset_to: &TzOffsetTo<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut StandardWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(STANDARD)?;
        let mut standard = StandardWriter::new(self.0, dtstart, tz_offset_from, tz_offset_to)?;
        body(&mut standard)?;
        self.0.write_end_unchecked(STANDARD)
    }

    pub fn write_daylight<F>(
        &mut self,
        dtstart: &DtStart<'_>,
        tz_offset_from: &TzOffsetFrom<'_>,
        tz_offset_to: &TzOffsetTo<'_>,
        body: F
    ) -> Result<(), Error>
    where
        F: FnOnce(&mut DaylightWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(DAYLIGHT)?;
        let mut daylight = DaylightWriter::new(self.0, dtstart, tz_offset_from, tz_offset_to)?;
        body(&mut daylight)?;
        self.0.write_end_unchecked(DAYLIGHT)
    }
}

pub struct AlarmWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> AlarmWriter<'w, W> {
    fn new(
        writer: &'w mut Writer<W>,
        action: &Action<'_>,
        trigger: &Trigger<'_>
    ) -> Result<Self, Error> {
        let mut alarm = Self(writer);
        alarm.write(action)?;
        alarm.write(trigger)?;
        Ok(alarm)
    }

    fn audio(writer: &'w mut Writer<W>, trigger: &Trigger<'_>) -> Result<Self, Error> {
        let mut alarm = Self(writer);
        alarm.write(&Action::audio())?;
        alarm.write(trigger)?;
        Ok(alarm)
    }

    fn display(
        writer: &'w mut Writer<W>,
        trigger: &Trigger<'_>,
        description: &Description<'_>
    ) -> Result<Self, Error> {
        let mut alarm = Self(writer);
        alarm.write(&Action::display())?;
        alarm.write(trigger)?;
        alarm.write(description)?;
        Ok(alarm)
    }

    fn email(
        writer: &'w mut Writer<W>,
        trigger: &Trigger<'_>,
        description: &Description<'_>,
        summary: &Summary<'_>
    ) -> Result<Self, Error> {
        let mut alarm = Self(writer);
        alarm.write(&Action::email())?;
        alarm.write(trigger)?;
        alarm.write(description)?;
        alarm.write(summary)?;
        Ok(alarm)
    }
}

impl<W: Write> AlarmWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct StandardWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> StandardWriter<'w, W> {
    fn new(
        writer: &'w mut Writer<W>,
        dtstart: &DtStart<'_>,
        tz_offset_from: &TzOffsetFrom<'_>,
        tz_offset_to: &TzOffsetTo<'_>
    ) -> Result<Self, Error> {
        let mut standard = Self(writer);
        standard.write(dtstart)?;
        standard.write(tz_offset_from)?;
        standard.write(tz_offset_to)?;
        Ok(standard)
    }
}

impl<W: Write> StandardWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct DaylightWriter<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> DaylightWriter<'w, W> {
    fn new(
        writer: &'w mut Writer<W>,
        dtstart: &DtStart<'_>,
        tz_offset_from: &TzOffsetFrom<'_>,
        tz_offset_to: &TzOffsetTo<'_>
    ) -> Result<Self, Error> {
        let mut daylight = Self(writer);
        daylight.write(dtstart)?;
        daylight.write(tz_offset_from)?;
        daylight.write(tz_offset_to)?;
        Ok(daylight)
    }
}

impl<W: Write> DaylightWriter<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}
