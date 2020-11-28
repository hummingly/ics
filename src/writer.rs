#![allow(dead_code)]
use crate::contentline::{LineWrite, LineWriter};
use std::io::{Error, Write};

pub struct CalendarWriter<W: Write>(LineWriter<W>);

impl<W: Write> CalendarWriter<W> {
    pub fn new(writer: W, version: String, product_id: String) -> Result<CalendarWriter<W>, Error> {
        let mut line_writer = LineWriter::new(writer);
        line_writer.write_begin_unchecked("VCALENDAR")?;
        write!(line_writer, "VERSION:{}", version)?;
        line_writer.write_line_ending()?;
        write!(line_writer, "PRODID:{}", product_id)?;
        line_writer.write_line_ending()?;
        Ok(CalendarWriter(line_writer))
    }

    #[inline]
    fn write<C: LineWrite>(&mut self, content_line: &C) -> Result<(), Error> {
        content_line.write_content_line(&mut self.0)
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

pub struct EventWriter<'w, W: Write>(&'w mut LineWriter<W>);

impl<'w, W: Write> EventWriter<'w, W> {
    fn new(
        writer: &'w mut LineWriter<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<EventWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.write_line_ending()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.write_line_ending()?;
        Ok(EventWriter(writer))
    }

    #[inline]
    fn write<C: LineWrite>(&mut self, content_line: &C) -> Result<(), Error> {
        content_line.write_content_line(&mut self.0)
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

pub struct TodoWriter<'w, W: Write>(&'w mut LineWriter<W>);

impl<'w, W: Write> TodoWriter<'w, W> {
    fn new(
        writer: &'w mut LineWriter<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<TodoWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.write_line_ending()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.write_line_ending()?;
        Ok(TodoWriter(writer))
    }

    #[inline]
    fn write<C: LineWrite>(&mut self, content_line: &C) -> Result<(), Error> {
        content_line.write_content_line(&mut self.0)
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

pub struct JournalWriter<'w, W: Write>(&'w mut LineWriter<W>);

impl<'w, W: Write> JournalWriter<'w, W> {
    fn new(
        writer: &'w mut LineWriter<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<JournalWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.write_line_ending()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.write_line_ending()?;
        Ok(JournalWriter(writer))
    }

    #[inline]
    fn write<C: LineWrite>(&mut self, content_line: &C) -> Result<(), Error> {
        content_line.write_content_line(&mut self.0)
    }
}

pub struct FreeBusyWriter<'w, W: Write>(&'w mut LineWriter<W>);

impl<'w, W: Write> FreeBusyWriter<'w, W> {
    fn new(
        writer: &'w mut LineWriter<W>,
        uid: &str,
        dt_stamp: &str
    ) -> Result<FreeBusyWriter<'w, W>, Error> {
        writeln!(writer, "UID:{}", uid)?;
        writer.write_line_ending()?;
        writeln!(writer, "DTSTAMP:{}", dt_stamp)?;
        writer.write_line_ending()?;
        Ok(FreeBusyWriter(writer))
    }
}

pub struct AlarmWriter<'w, W: Write>(&'w mut LineWriter<W>);

impl<'w, W: Write> AlarmWriter<'w, W> {
    fn new(writer: &'w mut LineWriter<W>) -> Result<AlarmWriter<'w, W>, Error> {
        // TODO: Required properties
        Ok(AlarmWriter(writer))
    }

    #[inline]
    fn write<C: LineWrite>(&mut self, content_line: &C) -> Result<(), Error> {
        content_line.write_content_line(&mut self.0)
    }
}
