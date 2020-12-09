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

pub struct ICalendarWriter<W: Write>(Writer<W>);

impl<W: Write> ICalendarWriter<W> {
    pub fn new(inner: W, version: &str, product_id: &str) -> Result<ICalendarWriter<W>, Error> {
        let mut writer = Writer::new(inner);
        writer.write_begin_unchecked(VCALENDAR)?;
        write!(writer, "VERSION:{}", version)?;
        writer.end_line()?;
        write!(writer, "PRODID:{}", product_id)?;
        writer.end_line()?;
        Ok(ICalendarWriter(writer))
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
        F: FnOnce(&mut Event<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VEVENT)?;
        let mut writer = Event::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VEVENT)
    }

    pub fn write_todo<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut ToDo<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VTODO)?;
        let mut writer = ToDo::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VTODO)
    }

    pub fn write_journal<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Journal<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VJOURNAL)?;
        let mut writer = Journal::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VJOURNAL)
    }

    pub fn write_freebusy<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut FreeBusy<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VFREEBUSY)?;
        let mut writer = FreeBusy::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VFREEBUSY)
    }

    pub fn write_timezone<F>(&mut self, tzid: &TzID, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut TimeZone<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VTIMEZONE)?;
        let mut writer = TimeZone::new(&mut self.0, tzid)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VTIMEZONE)
    }

    pub fn close(mut self) -> Result<W, Error> {
        self.0.write_end_unchecked(VCALENDAR)?;
        self.0.into_inner()
    }
}

pub struct Event<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> Event<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> Event<'_, W> {
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
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::new(self.0, action, trigger)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_audio_alarm<F>(&mut self, trigger: &Trigger<'_>, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::audio(self.0, trigger)?;
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
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::display(self.0, trigger, description)?;
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
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::email(self.0, trigger, description, summary)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }
}

pub struct ToDo<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> ToDo<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> ToDo<'_, W> {
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
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::new(self.0, action, trigger)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }

    pub fn write_audio_alarm<F>(&mut self, trigger: &Trigger<'_>, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::audio(self.0, trigger)?;
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
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::display(self.0, trigger, description)?;
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
        F: FnOnce(&mut Alarm<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = Alarm::email(self.0, trigger, description, summary)?;
        body(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }
}

pub struct Journal<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> Journal<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> Journal<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct FreeBusy<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> FreeBusy<'w, W> {
    fn new(writer: &'w mut Writer<W>, uid: &str, dt_stamp: &str) -> Result<Self, Error> {
        write!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        write!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(Self(writer))
    }
}

impl<W: Write> FreeBusy<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct TimeZone<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> TimeZone<'w, W> {
    fn new(writer: &'w mut Writer<W>, tzid: &TzID) -> Result<Self, Error> {
        let mut timezone = Self(writer);
        timezone.write(tzid)?;
        Ok(timezone)
    }
}

impl<W: Write> TimeZone<'_, W> {
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
        F: FnOnce(&mut Standard<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(STANDARD)?;
        let mut standard = Standard::new(self.0, dtstart, tz_offset_from, tz_offset_to)?;
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
        F: FnOnce(&mut Daylight<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(DAYLIGHT)?;
        let mut daylight = Daylight::new(self.0, dtstart, tz_offset_from, tz_offset_to)?;
        body(&mut daylight)?;
        self.0.write_end_unchecked(DAYLIGHT)
    }
}

pub struct Alarm<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> Alarm<'w, W> {
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

impl<W: Write> Alarm<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct Standard<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> Standard<'w, W> {
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

impl<W: Write> Standard<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct Daylight<'w, W: Write>(&'w mut Writer<W>);

impl<'w, W: Write> Daylight<'w, W> {
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

impl<W: Write> Daylight<'_, W> {
    pub fn write<P>(&mut self, property: &P) -> Result<(), Error>
    where
        P: PropertyWrite
    {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}
