use crate::util::{write_base64, write_escaped_text};
use std::fmt;
use std::io::{Error, Write};

const LINE_MAX_LEN: usize = 75;
const CAPACITY: usize = LINE_MAX_LEN * 2;

pub trait PropertyWrite {
    fn write(&self, line: &mut LineWriter<'_>) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct LineWriter<'w>(Writer<'w>);

impl<'w> LineWriter<'w> {
    pub(crate) fn new(inner: &'w mut dyn Write) -> Self {
        Self(Writer::new(inner))
    }
}

impl LineWriter<'_> {
    pub(crate) fn write_name_unchecked(&mut self, name: &str) {
        let end = name.len();
        self.0.buffer[..end].copy_from_slice(name.as_bytes());
        self.0.len = end;
    }

    #[inline]
    pub(crate) fn write_property(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        property.write(self)?;
        self.write_line_break()
    }

    pub(crate) fn write_begin(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= LINE_MAX_LEN - "BEGIN:".len() {
            self.write_begin_unchecked(component)
        } else {
            write!(self.0, "BEGIN:{}", component)?;
            self.write_line_break()
        }
    }

    pub(crate) fn write_end(&mut self, component: &str) -> Result<(), Error> {
        if component.len() <= LINE_MAX_LEN - "END:".len() {
            self.write_begin_unchecked(component)
        } else {
            write!(self.0, "END:{}", component)?;
            self.write_line_break()
        }
    }

    pub(crate) fn write_begin_unchecked(&mut self, component: &str) -> Result<(), Error> {
        writeln!(self.0.inner, "BEGIN:{}\r", component)
    }

    pub(crate) fn write_end_unchecked(&mut self, component: &str) -> Result<(), Error> {
        writeln!(self.0.inner, "END:{}\r", component)
    }

    fn write_line_break(&mut self) -> Result<(), Error> {
        self.0.write_buffer()?;
        self.0.inner.write_all(b"\r\n")
    }
}

impl LineWriter<'_> {
    pub fn write_name(&mut self, name: &str) -> Result<(), Error> {
        self.0.write_all(name.as_bytes())
    }

    pub fn write_parameter(&mut self, key: &str, value: &str) -> Result<(), Error> {
        write!(self.0, ";{}={}", key, value)
    }

    pub fn write_value(&mut self, value: &dyn fmt::Display) -> Result<(), Error> {
        write!(self.0, ":{}", value)
    }

    pub fn write_fmt_value(&mut self, value: fmt::Arguments) -> Result<(), Error> {
        write!(self.0, ":{}", value)
    }

    pub fn write_boolean_value(&mut self, value: bool) -> Result<(), Error> {
        self.0.write_all(if value { b"TRUE" } else { b"FALSE" })
    }

    /// Writes binary data as BASE64 encoded string.
    pub fn write_binary_value(&mut self, binary: &[u8]) -> Result<(), Error> {
        self.0.write_all(b":")?;
        write_base64(&mut self.0, binary)
    }

    /// Escapes comma, semicolon and backslash, and normalizes newlines.
    pub fn write_text_value(&mut self, text: &str) -> Result<(), Error> {
        self.0.write_all(b":")?;
        write_escaped_text(&mut self.0, text)
    }
}

struct Writer<'w> {
    buffer: [u8; CAPACITY],
    len: usize,
    inner: &'w mut dyn Write
}

impl<'w> Writer<'w> {
    fn new(inner: &'w mut dyn Write) -> Self {
        Self {
            buffer: [0; CAPACITY],
            len: 0,
            inner
        }
    }
}

impl Writer<'_> {
    fn write_buffer(&mut self) -> Result<(), Error> {
        match self.fold_buffer() {
            Ok(0) => Ok(()),
            Ok(n) => self.inner.write_all(&self.buffer[self.len - n..self.len]),
            Err(error) => Err(error)
        }?;
        self.len = 0;
        Ok(())
    }

    /// Folds and writes exactly LIMIT * N bytes and returns number of not
    /// written bytes.
    fn fold_buffer(&mut self) -> Result<usize, Error> {
        /// Delimeter for content lines (CR LF SPACE)
        const LINE_BREAK: &[u8] = b"\r\n ";

        fn next_boundary(input: &[u8]) -> Option<usize> {
            if input.len() <= LINE_MAX_LEN {
                return None;
            }

            // In str::is_char_boundary bit magic is used in the form of (b as i8) >= -0x40
            // but this is more understandable for me.
            fn is_char_boundary(b: u8) -> bool {
                !(128..192).contains(&b)
            }

            for boundary in (1..=LINE_MAX_LEN).rev() {
                if is_char_boundary(input[boundary]) {
                    return Some(boundary);
                }
            }
            None
        }

        let mut content = &self.buffer[..self.len];
        let mut boundary = next_boundary(content).unwrap_or(content.len());
        self.inner.write_all(&content[..boundary])?;

        while boundary < content.len() {
            content = &content[boundary..];
            self.inner.write_all(LINE_BREAK)?;
            match next_boundary(content) {
                Some(next_boundary) => {
                    self.inner.write_all(&content[..next_boundary])?;
                    boundary = next_boundary;
                }
                None => return Ok(content.len())
            }
        }
        Ok(0)
    }

    fn extend_buffer(&mut self, buffer: &[u8]) {
        let end = self.len + buffer.len();
        self.buffer[self.len..end].copy_from_slice(buffer);
        self.len = end;
    }
}

impl Write for Writer<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Error> {
        self.write_buffer()?;
        self.inner.flush()
    }

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        if self.len + buf.len() < CAPACITY {
            self.extend_buffer(buf);
            return Ok(());
        }

        let mut end = CAPACITY - self.len;
        loop {
            self.buffer[self.len..].copy_from_slice(&buf[..end]);
            self.len = CAPACITY;
            match self.fold_buffer() {
                Ok(n) => {
                    self.buffer.copy_within(CAPACITY - n.., 0);
                    // SAFETY: The value n can never be bigger than CAPACITY because the input
                    // self.buffer is CAPACITY bytes long!
                    assert!(n <= CAPACITY);
                    self.len = n;
                    buf = &buf[end..];
                    end = CAPACITY - self.len;
                    if buf.len() < end {
                        self.extend_buffer(buf);
                        break;
                    }
                }
                Err(err) => return Err(err)
            }
        }
        Ok(())
    }
}

impl fmt::Debug for Writer<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Writer")
            .field("buffer", &&self.buffer[..])
            .field("len", &self.len)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Writer;
    use std::io::{Error, Write};

    fn write(content: &[u8]) -> Result<String, Error> {
        let mut output = Vec::with_capacity(content.len());
        let mut writer = Writer::new(&mut output);
        writer.write_all(content)?;
        writer.flush()?;
        Ok(String::from_utf8_lossy(&output).to_string())
    }

    #[test]
    fn no_linebreak() -> Result<(), Error> {
        let content = "No line break today.";
        let output = write(content.as_bytes())?;

        assert_eq!(output, content);
        Ok(())
    }

    #[test]
    fn over_limit() -> Result<(), Error> {
        let content = "Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.";
        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";
        let output = write(content.as_bytes())?;

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn multibytes() -> Result<(), Error> {
        let content = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";
        let output = write(content.as_bytes())?;

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn multibytes_with_space() -> Result<(), Error> {
        let content =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老 虎.";
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老 \r\n 虎.";
        let output = write(content.as_bytes())?;

        assert_eq!(output, expected);
        Ok(())
    }

    #[test]
    fn multi_lines() -> Result<(), Error> {
        let content = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown\r\n  fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let output = write(content.as_bytes())?;

        assert_eq!(output, expected);
        Ok(())
    }
}
