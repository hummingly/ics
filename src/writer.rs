#![allow(dead_code)]
use std::{
    fmt,
    io::{Error, Write}
};

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
        Ok(self.0.writer)
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

const CONTENT_LINE_MAX_LEN: usize = 75;
const CAPACITY: usize = CONTENT_LINE_MAX_LEN * 2;
const LINE_SPLIT: &[u8; 3] = b"\r\n ";

pub struct LineWriter<W: Write> {
    buffer: Box<[u8; CAPACITY]>,
    len: usize,
    writer: W
}

impl<W: Write> LineWriter<W> {
    fn new(writer: W) -> LineWriter<W> {
        LineWriter {
            buffer: Box::new([0; CAPACITY]),
            len: 0,
            writer
        }
    }

    #[inline]
    pub fn write_name<N>(&mut self, name: N) -> Result<(), Error>
    where
        N: fmt::Display
    {
        write!(self, "{}", name)
    }

    #[inline]
    pub fn write_parameter<K, V>(&mut self, key: K, value: V) -> Result<(), Error>
    where
        K: fmt::Display,
        V: fmt::Display
    {
        write!(self, ";{}={}", key, value)
    }

    #[inline]
    pub fn write_value<V>(&mut self, value: V) -> Result<(), Error>
    where
        V: fmt::Display
    {
        write!(self, ":{}", value)
    }

    pub fn write_line_ending(&mut self) -> Result<(), Error> {
        self.flush_line()?;
        self.writer.write_all(b"\r\n")
    }

    /// Write BEGIN limiter without folding
    ///
    /// Components part of the specification have names that are shorter than
    /// `LIMIT - "BEGIN:".len()`. This is why checking for line breaks in a
    /// single line is redundant.
    fn write_begin_unchecked(&mut self, component: &str) -> Result<(), Error> {
        debug_assert!(component.len() <= CONTENT_LINE_MAX_LEN - "BEGIN:".len());
        writeln!(self.writer, "BEGIN:{}\r", component)
    }

    /// Write END limiter without folding
    ///
    /// Components part of the specification have names that are shorter than
    /// `LIMIT - "END:".len()`. This is why checking for line breaks in a
    /// single line is redundant.
    fn write_end_unchecked(&mut self, component: &str) -> Result<(), Error> {
        debug_assert!(component.len() <= CONTENT_LINE_MAX_LEN - "END:".len());
        writeln!(self.writer, "END:{}\r", component)
    }

    fn write_begin(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= CONTENT_LINE_MAX_LEN - "BEGIN:".len() {
            self.write_begin_unchecked(component)
        } else {
            writeln!(self, "BEGIN:{}", component)?;
            self.write_line_ending()
        }
    }

    fn write_end(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= CONTENT_LINE_MAX_LEN - "END:".len() {
            self.write_end_unchecked(component)
        } else {
            writeln!(self, "END:{}", component)?;
            self.write_line_ending()
        }
    }

    fn flush_line(&mut self) -> Result<(), Error> {
        if self.len > 0 {
            match lazy_fold(&mut self.writer, &self.buffer[..self.len]) {
                Ok(0) => Ok(()),
                Ok(rest) => self
                    .writer
                    .write_all(&self.buffer[self.len - rest..self.len]),
                Err(error) => Err(error)
            }?;
            self.len = 0;
        }
        Ok(())
    }

    fn extend_from_slice(&mut self, bytes: &[u8]) {
        let end = self.len + bytes.len();
        self.buffer[self.len..end].copy_from_slice(bytes);
        self.len = end;
    }
}

impl<W: Write> Write for LineWriter<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        match self.write_all(buf) {
            Ok(_) => Ok(buf.len()),
            Err(error) => Err(error)
        }
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.flush_line()?;
        self.writer.flush()
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        if buf.is_empty() {
            return Ok(());
        }

        if self.len + buf.len() < CAPACITY {
            self.extend_from_slice(buf);
        } else {
            loop {
                let end = CAPACITY - self.len;
                self.buffer[self.len..CAPACITY].copy_from_slice(&buf[..end]);
                match lazy_fold(&mut self.writer, self.buffer.as_ref()) {
                    Ok(rest) => {
                        self.buffer.copy_within(CAPACITY - rest..CAPACITY, 0);
                        self.len = rest;
                        buf = &buf[end..];
                        if buf.len() < end {
                            self.extend_from_slice(buf);
                            break;
                        }
                    }
                    Err(err) => {
                        self.len = CAPACITY;
                        return Err(err);
                    }
                }
            }
        }
        Ok(())
    }
}

/// Folds and writes exactly LIMIT * N bytes and returns number of not written
/// bytes
fn lazy_fold<W: Write>(writer: &mut W, content: &[u8]) -> Result<usize, Error> {
    let len = content.len();
    let mut boundary = next_boundary(&content, CONTENT_LINE_MAX_LEN).unwrap_or(len);
    writer.write_all(&content[..boundary])?;

    while boundary < len {
        writer.write_all(LINE_SPLIT)?;
        boundary = match next_boundary(&content, boundary + CONTENT_LINE_MAX_LEN) {
            Some(next_boundary) => {
                writer.write_all(&content[boundary..next_boundary])?;
                next_boundary
            }
            None => return Ok(len - boundary)
        };
    }
    Ok(0)
}

fn next_boundary(input: &[u8], index: usize) -> Option<usize> {
    if index >= input.len() {
        return None;
    }
    match input[..=index].iter().rposition(|&i| i < 128 || i >= 192) {
        Some(0) | None => None,
        boundary => boundary
    }
}

pub trait LineWrite {
    fn write_content_line<W: Write>(&self, line_writer: &mut LineWriter<W>) -> Result<(), Error>;
}

#[cfg(test)]
mod test {
    use crate::escape_text;

    use super::{LineWrite, LineWriter};
    use std::{collections::BTreeMap, io::Write};

    struct MockupText {
        name: String,
        value: String,
        parameters: BTreeMap<String, String>
    }

    impl MockupText {
        fn new(name: &str, text: &str) -> Self {
            MockupText {
                name: String::from(name),
                value: String::from(text),
                parameters: BTreeMap::new()
            }
        }

        fn add_parameter(&mut self, key: &str, value: &str) {
            self.parameters
                .insert(String::from(key), String::from(value));
        }
    }

    impl LineWrite for MockupText {
        fn write_content_line<W: Write>(
            &self,
            line_writer: &mut LineWriter<W>
        ) -> Result<(), std::io::Error> {
            line_writer.write_name(&self.name)?;
            for (key, value) in &self.parameters {
                line_writer.write_parameter(key, value)?;
            }
            line_writer.write_value(&self.value)?;
            line_writer.write_line_ending()
        }
    }

    #[test]
    fn summary() {
        let buffer = Vec::new();
        let mut line_writer = LineWriter::new(buffer);
        let s = MockupText::new("SUMMARY", "Staff meeting minutes");
        s.write_content_line(&mut line_writer).unwrap();

        assert_eq!(
            String::from_utf8_lossy(&line_writer.writer),
            "SUMMARY:Staff meeting minutes\r\n"
        );
    }

    #[test]
    fn description() {
        let buffer = Vec::new();
        let mut line_writer = LineWriter::new(buffer);
        let value = escape_text("1. Staff meeting: Participants include Joe, Lisa, and Bob. Aurora project plans were reviewed. There is currently no budget reserves for this project. Lisa will escalate to management. Next meeting on Tuesday.\n\
        2. Telephone Conference: ABC Corp. sales representative called to discuss new printer. Promised to get us a demo by Friday.\n\
        3. Henry Miller (Handsoff Insurance): Car was totaled by tree. Is looking into a loaner car. 555-2323 (tel).");
        let s = MockupText::new("DESCRIPTION", &value);
        s.write_content_line(&mut line_writer).unwrap();

        assert_eq!(
            String::from_utf8_lossy(&line_writer.writer),
            "DESCRIPTION:1. Staff meeting: Participants include Joe\\, Lisa\\, and Bob. Au\r\n rora project plans were reviewed. There is currently no budget reserves for\r\n  this project. Lisa will escalate to management. Next meeting on Tuesday.\n\
            2\r\n . Telephone Conference: ABC Corp. sales representative called to discuss ne\r\n w printer. Promised to get us a demo by Friday.\n\
            3. Henry Miller (Handsoff I\r\n nsurance): Car was totaled by tree. Is looking into a loaner car. 555-2323 \r\n (tel).\r\n"
        );
    }
}
