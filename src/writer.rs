#![allow(dead_code)]
use crate::contentline::{ContentLine, PropertyWrite, Writer};
use std::io::{Error, Write};

pub struct CalendarWriter<W: Write>(Writer<W>);

impl<W: Write> CalendarWriter<W> {
    pub fn new(writer: W, version: String, product_id: String) -> Result<CalendarWriter<W>, Error> {
        let mut line_writer = Writer::new(writer);
        line_writer.write_begin_unchecked("VCALENDAR")?;
        write!(line_writer, "VERSION:{}", version)?;
        line_writer.end_line()?;
        write!(line_writer, "PRODID:{}", product_id)?;
        line_writer.end_line()?;
        Ok(CalendarWriter(line_writer))
    }

    fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_event<F>(&mut self, uid: &str, dt_stamp: &str, write_fn: F) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut EventWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked("VEVENT")?;
        let mut writer = EventWriter::new(&mut self.0, uid, dt_stamp)?;
        write_fn(&mut writer)?;
        self.0.write_end_unchecked("VEVENT")
    }

    pub fn write_todo<F>(&mut self, uid: &str, dt_stamp: &str, write_fn: F) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut TodoWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked("VTODO")?;
        let mut writer = TodoWriter::new(&mut self.0, uid, dt_stamp)?;
        write_fn(&mut writer)?;
        self.0.write_end_unchecked("VTODO")
    }

    pub fn write_journal<F>(&mut self, uid: &str, dt_stamp: &str, write_fn: F) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut JournalWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked("VJOURNAL")?;
        let mut writer = JournalWriter::new(&mut self.0, uid, dt_stamp)?;
        write_fn(&mut writer)?;
        self.0.write_end_unchecked("VJOURNAL")
    }

    pub fn write_free_busy<F>(
        &mut self,
        uid: &str,
        dt_stamp: &str,
        write_fn: F
    ) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut FreeBusyWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked("VFREEBUSY")?;
        let mut writer = FreeBusyWriter::new(&mut self.0, uid, dt_stamp)?;
        write_fn(&mut writer)?;
        self.0.write_end_unchecked("VFREEBUSY")
    }

    pub fn finish(mut self) -> Result<W, Error> {
        self.0.write_end_unchecked("VCALENDAR")?;
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
        self.0.write_begin_unchecked("VALARM")?;
        let mut alarm = AlarmWriter::new(self.0)?;
        write_fn(&mut alarm)?;
        self.0.write_end_unchecked("VALARM")
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

    fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }

    pub fn write_alarm<F>(&mut self, write_fn: F) -> Result<(), Error>
    where
        F: for<'f> FnOnce(&mut AlarmWriter<'f, W>) -> Result<(), Error>
    {
        self.0.write_begin_unchecked("VALARM")?;
        let mut alarm = AlarmWriter::new(self.0)?;
        write_fn(&mut alarm)?;
        self.0.write_end_unchecked("VALARM")
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

    fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
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
}

pub struct AlarmWriter<'w, W: Write>(&'w mut Writer<W>);

impl<W: Write> AlarmWriter<'_, W> {
    fn new<'w>(writer: &'w mut Writer<W>) -> Result<AlarmWriter<'w, W>, Error> {
        // TODO: Required properties
        Ok(AlarmWriter(writer))
    }

    fn write<P: PropertyWrite>(&mut self, property: &P) -> Result<(), Error> {
        let mut line = ContentLine::new(&mut self.0);
        property.write(&mut line)?;
        line.end_line()
    }
}
