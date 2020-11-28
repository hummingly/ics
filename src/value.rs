#![allow(dead_code)]
use std::{borrow::Cow, convert::TryFrom, error::Error, fmt, io, marker::PhantomData};

pub type Integer = i32;

pub type Float = f32;

#[derive(Debug, Copy, Clone)]
pub struct Boolean(pub bool);

impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.0 { "TRUE" } else { "FALSE" })
    }
}

#[derive(Debug, Clone)]
pub struct Binary<'b>(Cow<'b, str>);

fn is_base64_encoded(binary: &[u8]) -> Result<(), ParseBinaryError> {
    if binary.is_empty() {
        return Ok(());
    }

    if binary.len() % 4 > 0 {
        return Err(ParseBinaryError::MissingBytes);
    }

    fn is_base64(&b: &u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'+' || b == b'/'
    }

    // At most there can be only two '=' characters at the end. This is safe to call
    // because binary.len is greater than 0 and a multiple of 4.
    let mid = binary.len() - 2;
    let (encoding, padding) = (&binary[..mid], &binary[mid..]);
    if encoding.iter().all(is_base64) {
        if match padding {
            [b'=', b'='] => true,
            [b, b'='] => is_base64(b),
            [b1, b2] => is_base64(b1) && is_base64(b2),
            _ => false
        } {
            Ok(())
        } else {
            Err(ParseBinaryError::InvalidEncoding)
        }
    } else {
        Err(ParseBinaryError::InvalidEncoding)
    }
}

impl<'b> TryFrom<&'b str> for Binary<'b> {
    type Error = ParseBinaryError;

    fn try_from(value: &'b str) -> Result<Self, Self::Error> {
        is_base64_encoded(value.as_bytes()).map(|_| Binary(Cow::Borrowed(value)))
    }
}

impl TryFrom<String> for Binary<'_> {
    type Error = ParseBinaryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        is_base64_encoded(value.as_bytes()).map(|_| Binary(Cow::Owned(value)))
    }
}

impl<'b> TryFrom<&'b [u8]> for Binary<'b> {
    type Error = ParseBinaryError;

    fn try_from(value: &'b [u8]) -> Result<Self, Self::Error> {
        match is_base64_encoded(value) {
            // unsafe std::str::from_utf8_unchecked if this turns out as bottle neck
            Ok(()) => match std::str::from_utf8(value) {
                Ok(value) => Ok(Binary(Cow::Borrowed(value))),
                // base64 encoded bytes are all ascii
                _ => unreachable!()
            },
            _ => Err(ParseBinaryError::InvalidEncoding)
        }
    }
}

impl TryFrom<Vec<u8>> for Binary<'_> {
    type Error = ParseBinaryError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match is_base64_encoded(&value) {
            // unsafe String::from_utf8_unchecked if this turns out as bottle neck
            Ok(()) => match String::from_utf8(value) {
                Ok(value) => Ok(Binary(Cow::Owned(value))),
                // base64 encoded bytes are all ascii
                _ => unreachable!()
            },
            _ => Err(ParseBinaryError::InvalidEncoding)
        }
    }
}

impl fmt::Display for Binary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Parsing errors for standard Base64 encoded Binary.
#[derive(Debug, Clone)]
pub enum ParseBinaryError {
    /// Invalid characters for standard Base64 encoding.
    InvalidEncoding,
    /// Padding is incorrect or not all bytes were properly encoded.
    MissingBytes
}

impl ParseBinaryError {
    fn as_str(&self) -> &str {
        match self {
            ParseBinaryError::InvalidEncoding => {
                "Binary data is encoded with the standard Base64 encoding \
                 ( [a..z] | [A..Z] | + | / | = (padding) )."
            }
            ParseBinaryError::MissingBytes => "Incorrect number of bytes or missing padding."
        }
    }
}

impl fmt::Display for ParseBinaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Error for ParseBinaryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

// TODO: Validation?
pub type Uri<'u> = Cow<'u, str>;

// TODO: Validation?
pub type CalAdress<'a> = Cow<'a, str>;

#[derive(Debug, Clone)]
pub struct Text<'t>(pub Cow<'t, str>);

impl<'t> From<&'t str> for Text<'t> {
    fn from(text: &'t str) -> Self {
        Text(Cow::Borrowed(text))
    }
}

impl From<String> for Text<'_> {
    fn from(text: String) -> Self {
        Text(Cow::Owned(text))
    }
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_escaped_text(f, &self.0)
    }
}

pub fn write_escaped_text<W>(writer: &mut W, text: &str) -> Result<(), fmt::Error>
where
    W: fmt::Write
{
    let mut last_end = 0;
    for (start, part) in EscapeByteIndices::new(text.as_bytes()) {
        writer.write_str(&text[last_end..start])?;
        match part {
            b'\r' => {
                // Replace old macOS newline character with a line feed character otherwise
                // discard carriage return character for Windows OS.
                // WARNING: Do not implement this with slicing instead of str::get. Indexing
                // outside a char boundary will panic!
                if text.get(start + 1..start + 2) != Some("\n") {
                    writer.write_str("\n")?;
                }
            }
            b => writer.write_fmt(format_args!("\\{}", char::from(b)))?
        }
        last_end = start + 1;
    }
    writer.write_str(&text[last_end..])
}

pub(crate) fn write_escaped_bytes<W>(writer: &mut W, bytes: &[u8]) -> Result<(), io::Error>
where
    W: io::Write
{
    let mut last_end = 0;
    for (start, part) in EscapeByteIndices::new(bytes) {
        writer.write_all(&bytes[last_end..start])?;
        match part {
            b'\r' => {
                // Replace old macOS newline character with a line feed character otherwise
                // discard carriage return character for Windows OS.
                let index = start + 1;
                if index <= bytes.len() && bytes[index] == b'\n' {
                    writer.write_all(b"\n")?;
                }
            }
            b => writer.write_all(&[b'\\', b])?
        }
        last_end = start + 1;
    }
    writer.write_all(&bytes[last_end..])
}

struct EscapeByteIndices<'m> {
    offset: usize,
    bytes: &'m [u8]
}

impl<'m> EscapeByteIndices<'m> {
    fn new(bytes: &'m [u8]) -> EscapeByteIndices<'m> {
        EscapeByteIndices { offset: 0, bytes }
    }
}

impl Iterator for EscapeByteIndices<'_> {
    type Item = (usize, u8);

    fn next(&mut self) -> Option<Self::Item> {
        fn is_escaped_byte(b: u8) -> bool {
            b == b',' || b == b';' || b == b'\\' || b == b'\r'
        }

        for &b in &self.bytes[self.offset..] {
            self.offset += 1;
            if is_escaped_byte(b) {
                return Some((self.offset - 1, b));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::Text;

    #[test]
    fn escaped_chars() {
        let s = ",\r\n;:\\ \r\n\rö";
        let expected = "\\,\n\\;:\\\\ \n\nö";
        assert_eq!(expected, Text::from(s).to_string());
    }

    #[test]
    fn no_escaped_chars() {
        let s = "This is a simple sentence.";
        assert_eq!(s, Text::from(s).to_string());
    }

    // test run with default features enabled but should be correct regardless
    #[test]
    fn escape_property() {
        use components::Property;

        let expected_value = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.\n";
        let property = Property::new(
            "COMMENT",
            Text::from("Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n").to_string()
        );
        assert_eq!(expected_value, property.value);
    }
}

pub struct Date {
    year: u16,
    month: u8,
    day: u8
}

/// Local/Floating Time Marker
pub enum Local {}
/// Utc Time Marker
pub enum Utc {}

/// ICalendar Time
pub struct Time<T = Local> {
    hour: u8,
    minute: u8,
    second: u8,
    _phantom: PhantomData<T>
}

pub struct DateTime<T = Local> {
    date: Date,
    time: Time<T>
}

pub struct UtcOffset {
    hour: i8,
    minute: u8,
    second: u8
}

enum DurationInner {
    Week(u32),
    Day(u32),
    Time {
        hour: u8,
        minute: u8,
        second: u8
    },
    DayTime {
        day: u32,
        hour: u8,
        minute: u8,
        second: u8
    }
}

pub enum Positive {}
pub enum Negative {}

pub struct Duration<T = Positive> {
    inner: DurationInner,
    _phantom: PhantomData<T>
}

impl<T> Duration<T> {
    fn new(duration: DurationInner) -> Self {
        Duration {
            inner: duration,
            _phantom: PhantomData
        }
    }

    fn _week(week: u32) -> Self {
        Duration::new(DurationInner::Week(week))
    }

    fn _day(day: u32) -> Self {
        Duration::new(DurationInner::Day(day))
    }

    fn _day_time(day: u32, hour: u8, minute: u8, second: u8) -> Self {
        Duration::new(DurationInner::DayTime {
            day,
            hour,
            minute,
            second
        })
    }

    fn _time(hour: u8, minute: u8, second: u8) -> Self {
        Duration::new(DurationInner::Time {
            hour,
            minute,
            second
        })
    }
}

impl Duration {
    pub fn week(week: u32) -> Duration {
        Self::_week(week)
    }

    pub fn day(day: u32) -> Duration {
        Self::_day(day)
    }

    pub fn day_time(day: u32, hour: u8, minute: u8, second: u8) -> Duration {
        Self::_day_time(day, hour, minute, second)
    }

    pub fn time(hour: u8, minute: u8, second: u8) -> Duration {
        Self::_time(hour, minute, second)
    }

    pub fn into_negative(self) -> Duration<Negative> {
        Duration::new(self.inner)
    }
}

impl Duration<Negative> {
    pub fn neg_week(week: u32) -> Duration<Negative> {
        Self::_week(week)
    }

    pub fn neg_day(day: u32) -> Duration<Negative> {
        Self::_day(day)
    }

    pub fn neg_day_time(day: u32, hour: u8, minute: u8, second: u8) -> Duration<Negative> {
        Self::_day_time(day, hour, minute, second)
    }

    pub fn neg_time(hour: u8, minute: u8, second: u8) -> Duration<Negative> {
        Self::_time(hour, minute, second)
    }

    pub fn into_positive(self) -> Duration<Positive> {
        Duration::new(self.inner)
    }
}

enum Period<T = Local> {
    /// The type bound on the type parameters is stricter than the specification
    /// demands. However, if start and end had different parameters, the end
    /// could be before the start when a time zone is added as a parameter
    /// to a property. In practice T will be Utc as only FreeBusy and RDate
    /// use a Period in UTC time.
    Explicit {
        start: DateTime<T>,
        end: DateTime<T>
    },
    Start {
        start: DateTime<T>,
        duration: Duration<Positive>
    }
}

// Recur
// List
