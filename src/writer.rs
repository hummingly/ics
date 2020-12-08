#![allow(dead_code)]
use crate::contentline::{ContentLine, PropertyWrite, Writer};
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
    pub fn new(inner: W, version: String, product_id: String) -> Result<CalendarWriter<W>, Error> {
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

    pub fn write_free_busy<F>(&mut self, uid: &str, dt_stamp: &str, body: F) -> Result<(), Error>
    where
        F: FnOnce(&mut FreeBusyWriter<'_, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VFREEBUSY)?;
        let mut writer = FreeBusyWriter::new(&mut self.0, uid, dt_stamp)?;
        body(&mut writer)?;
        self.0.write_end_unchecked(VFREEBUSY)
    }

    pub fn close(mut self) -> Result<W, Error> {
        self.0.write_end_unchecked(VCALENDAR)?;
        self.0.into_inner()
    }
}

pub struct EventWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> EventWriter<'_, W> {
    fn new<'w>(
        writer: &'w mut Writer<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<EventWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(EventWriter(writer))
    }

    fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm<F>(&mut self, write_fn: F) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut AlarmWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::new(self.0)?;
        write_fn(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }
}

pub struct TodoWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> TodoWriter<'_, W> {
    fn new<'w>(
        writer: &'w mut Writer<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<TodoWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(TodoWriter(writer))
    }

    pub fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm<F>(&mut self, write_fn: F) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut AlarmWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked(VALARM)?;
        let mut alarm = AlarmWriter::new(self.0)?;
        write_fn(&mut alarm)?;
        self.0.write_end_unchecked(VALARM)
    }
}

pub struct JournalWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> JournalWriter<'_, W> {
    fn new<'w>(
        writer: &'w mut Writer<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<JournalWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(JournalWriter(writer))
    }

    pub fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct FreeBusyWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> FreeBusyWriter<'_, W> {
    fn new<'w>(
        writer: &'w mut Writer<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<FreeBusyWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.end_line()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.end_line()?;
        Ok(FreeBusyWriter(writer))
    }

    pub fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}

pub struct AlarmWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> AlarmWriter<'_, W> {
    fn new<'w>(writer: &'w mut Writer<W>) -> Result<AlarmWriter<'w, W>, Error> {
        // TODO: Required properties
        Ok(AlarmWriter(writer))
    }

    pub fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}
