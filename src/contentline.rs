#![allow(dead_code)]
use crate::value::write_escaped_bytes;
use std::io::{Error, Write};

pub const LINE_MAX_LEN: usize = 75;
const CAPACITY: usize = LINE_MAX_LEN * 2;
const LINE_SPLIT: &[u8; 3] = b"\r\n ";

pub trait LineWrite {
    fn write_content_line<W: Write>(&self, line_writer: &mut LineWriter<W>) -> Result<(), Error>;
}

pub struct LineWriter<W: Write> {
    buffer: Box<[u8; CAPACITY]>,
    len: usize,
    writer: W
}

impl<W: Write> LineWriter<W> {
    pub(crate) fn new(writer: W) -> LineWriter<W> {
        LineWriter {
            buffer: Box::new([0; CAPACITY]),
            len: 0,
            writer
        }
    }

    pub fn write_name(&mut self, name: &str) -> Result<(), Error> {
        write!(self, "{}", name)
    }

    pub fn write_name_unchecked(&mut self, name: &str) {
        self.extend_from_slice(name.as_bytes());
    }

    pub fn write_parameter(&mut self, key: &str, value: &str) -> Result<(), Error> {
        write!(self, ";{}={}", key, value)
    }

    pub fn write_parameter_unchecked(&mut self, key: &str, value: &str) {
        self.buffer[self.len] = b';';
        self.len += 1;
        self.extend_from_slice(key.as_bytes());
        self.buffer[self.len] = b'=';
        self.len += 1;
        self.extend_from_slice(value.as_bytes());
    }

    #[inline]
    pub fn write_value<V>(&mut self, value: V) -> Result<(), Error>
    where
        V: std::fmt::Display
    {
        write!(self, ":{}", value)
    }

    pub fn write_escaped_text(&mut self, text: &str) -> Result<(), Error> {
        write_escaped_bytes(self, text.as_bytes())
    }

    pub fn write_line_ending(&mut self) -> Result<(), Error> {
        self.flush_line()?;
        self.writer.write_all(b"\r\n")
    }

    pub(crate) fn write_begin(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= LINE_MAX_LEN - "BEGIN:".len() {
            self.write_begin_unchecked(component)
        } else {
            writeln!(self, "BEGIN:{}", component)?;
            self.write_line_ending()
        }
    }

    /// Write BEGIN limiter without folding
    ///
    /// Components part of the specification have names that are shorter than
    /// `LIMIT - "BEGIN:".len()`. This is why checking for line breaks in a
    /// single line is redundant.
    pub(crate) fn write_begin_unchecked(&mut self, component: &str) -> Result<(), Error> {
        debug_assert!(component.len() <= LINE_MAX_LEN - "BEGIN:".len());
        writeln!(self.writer, "BEGIN:{}\r", component)
    }

    pub(crate) fn write_end(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= LINE_MAX_LEN - "END:".len() {
            self.write_end_unchecked(component)
        } else {
            writeln!(self, "END:{}", component)?;
            self.write_line_ending()
        }
    }

    /// Write END limiter without folding
    ///
    /// Components part of the specification have names that are shorter than
    /// `LIMIT - "END:".len()`. This is why checking for line breaks in a
    /// single line is redundant.
    pub(crate) fn write_end_unchecked(&mut self, component: &str) -> Result<(), Error> {
        debug_assert!(component.len() <= LINE_MAX_LEN - "END:".len());
        writeln!(self.writer, "END:{}\r", component)
    }

    pub(crate) fn into_inner(mut self) -> Result<W, Error> {
        self.flush_line()?;
        Ok(self.writer)
    }

    fn flush_line(&mut self) -> Result<(), Error> {
        if self.len > 0 {
            match lazy_fold(&mut self.writer, &self.buffer[..self.len]) {
                Ok(0) => Ok(()),
                Ok(n) => self.writer.write_all(&self.buffer[self.len - n..self.len]),
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
            let mut end = CAPACITY - self.len;
            loop {
                self.buffer[self.len..CAPACITY].copy_from_slice(&buf[..end]);
                match lazy_fold(&mut self.writer, self.buffer.as_ref()) {
                    Ok(n) => {
                        // SAFETY: The n value can never be bigger than CAPACITY because the input
                        // self.buffer is CAPACITY bytes long!
                        self.buffer.copy_within(CAPACITY - n..CAPACITY, 0);
                        self.len = n;
                        buf = &buf[end..];
                        end = CAPACITY - self.len;
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
/// bytes.
fn lazy_fold<W: Write>(writer: &mut W, mut content: &[u8]) -> Result<usize, Error> {
    let mut boundary = next_boundary(&content).unwrap_or(content.len());
    writer.write_all(&content[..boundary])?;

    while boundary < content.len() {
        content = &content[boundary..];
        writer.write_all(LINE_SPLIT)?;
        match next_boundary(&content) {
            Some(next_boundary) => {
                writer.write_all(&content[..next_boundary])?;
                boundary = next_boundary;
            }
            None => return Ok(content.len())
        }
    }
    Ok(0)
}

fn next_boundary(input: &[u8]) -> Option<usize> {
    if input.len() <= LINE_MAX_LEN {
        return None;
    }

    fn is_char_boundary(&b: &u8) -> bool {
        // In std::is_char_boundary bit magic is used in the form of (b as i8) >= -0x40
        // but this is more understandable for me.
        b < 128 || b >= 192
    }

    match input[..=LINE_MAX_LEN].iter().rposition(is_char_boundary) {
        Some(0) | None => None,
        boundary => boundary
    }
}

#[cfg(test)]
mod tests {
    use super::LineWriter;
    use std::io::Write;

    fn write(content: &[u8]) -> Result<String, std::io::Error> {
        let mut writer = LineWriter::new(Vec::with_capacity(content.len()));
        writer.write_all(content)?;
        writer.flush()?;
        Ok(String::from_utf8_lossy(&writer.writer).to_string())
    }

    #[test]
    fn no_linebreak() {
        let content = "No line break today.";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, content);
    }

    #[test]
    fn over_limit() {
        let content = "Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.";
        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn multibytes() {
        let content = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, expected);
    }

    #[test]
    fn multi_lines() {
        let content = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown\r\n  fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let output = write(content.as_bytes()).unwrap();

        assert_eq!(output, expected);
    }
}
