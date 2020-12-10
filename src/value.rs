#![allow(dead_code)]
use std::{
    borrow::Cow, convert::TryFrom, error::Error, fmt, io, marker::PhantomData, str::FromStr
};

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
pub struct Binary<'b>(Cow<'b, [u8]>);

impl<'b> From<&'b [u8]> for Binary<'b> {
    fn from(value: &'b [u8]) -> Self {
        Binary(Cow::Borrowed(value))
    }
}

impl From<Vec<u8>> for Binary<'_> {
    fn from(value: Vec<u8>) -> Self {
        Binary(Cow::Owned(value))
    }
}

impl TryFrom<&str> for Binary<'_> {
    type Error = ParseBinaryError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Binary::from_str(value)
    }
}

impl TryFrom<String> for Binary<'_> {
    type Error = ParseBinaryError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Binary::from_str(&value)
    }
}

impl FromStr for Binary<'_> {
    type Err = ParseBinaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match decode_base64(s) {
            Ok(bytes) => Ok(Binary(Cow::Owned(bytes))),
            Err(error) => Err(error)
        }
    }
}

impl fmt::Display for Binary<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        encode_base64(f, &self.0)
    }
}

/// Format bytes using the Base64 standard encoding.
fn encode_base64<W: fmt::Write>(output: &mut W, bytes: &[u8]) -> fmt::Result {
    // Mask for extracting 6 bits from a byte.
    const BIT_MASK: u8 = 0b0011_1111;

    const BASE_64: [char; 64] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
        'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1',
        '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'
    ];

    if bytes.is_empty() {
        return Ok(());
    }

    let mut chunks = bytes.chunks_exact(3);
    while let Some(chunk) = chunks.next() {
        // Since we cannot use unsafe and get chunks as arrays, we need to give the
        // compiler a hint.
        assert_eq!(chunk.len(), 3);
        output.write_char(BASE_64[usize::from(chunk[0] >> 2)])?;
        output.write_char(BASE_64[usize::from(chunk[0] << 4 & BIT_MASK | chunk[1] >> 4)])?;
        output.write_char(BASE_64[usize::from(chunk[1] << 2 & BIT_MASK | chunk[2] >> 6)])?;
        output.write_char(BASE_64[usize::from(chunk[2] & BIT_MASK)])?;
    }

    match chunks.remainder() {
        [first] => {
            output.write_char(BASE_64[usize::from(first >> 2)])?;
            output.write_char(BASE_64[usize::from(first << 4 & BIT_MASK)])?;
            output.write_str("==")
        }
        [first, second] => {
            output.write_char(BASE_64[usize::from(first >> 2)])?;
            output.write_char(BASE_64[usize::from(first << 4 & BIT_MASK | second >> 4)])?;
            output.write_char(BASE_64[usize::from(second << 2 & BIT_MASK)])?;
            output.write_char('=')
        }
        _ => Ok(())
    }
}

/// Decodes a Base64 encoded string into bytes.
fn decode_base64(input: &str) -> Result<Vec<u8>, ParseBinaryError> {
    fn decode_chunk(output: &mut Vec<u8>, chunk: &[u8]) -> Result<(), ParseBinaryError> {
        // Since we cannot use unsafe and get chunks as arrays, we need to give the
        // compiler a hint.
        assert_eq!(chunk.len(), 4);
        let first = to_base64_byte(chunk[0])?;
        let second = to_base64_byte(chunk[1])?;
        output.push(first << 2 | second >> 4);
        let third = to_base64_byte(chunk[2])?;
        output.push(second << 4 | third >> 2);
        let fourth = to_base64_byte(chunk[3])?;
        output.push(third << 6 | fourth);
        Ok(())
    }

    fn to_base64_byte(c: u8) -> Result<u8, ParseBinaryError> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => Err(ParseBinaryError::InvalidPadding),
            _ => Err(ParseBinaryError::InvalidEncoding)
        }
    }

    if input.is_empty() {
        return Ok(Vec::new());
    }

    if input.len() % 4 > 0 {
        return Err(ParseBinaryError::MissingBytes);
    }

    let input = input.as_bytes();
    let mut output = Vec::with_capacity(input.len() / 4 * 3);

    let mid = input.len() - 4;
    let (encoding, padding) = (&input[..mid], &input[mid..]);

    for chunk in encoding.chunks_exact(4) {
        decode_chunk(&mut output, chunk)?;
    }

    match padding {
        &[first, second, third, b'='] => {
            let first = to_base64_byte(first)?;
            let second = to_base64_byte(second)?;
            output.push(first << 2 | second >> 4);
            match to_base64_byte(third) {
                Ok(third) => {
                    output.push(second << 4 | third >> 2);
                    Ok(())
                }
                Err(ParseBinaryError::InvalidPadding) => Ok(()),
                Err(error) => Err(error)
            }
        }
        chunk => decode_chunk(&mut output, chunk)
    }?;
    Ok(output)
}

/// Parsing errors for standard Base64 encoded Binary.
#[derive(Debug, Clone)]
pub enum ParseBinaryError {
    /// Invalid characters for standard Base64 encoding.
    InvalidEncoding,
    // Padding bytes were set too early or incorrectly.
    InvalidPadding,
    /// Incorrect number of bytes were encoded.
    MissingBytes
}

impl ParseBinaryError {
    fn as_str(&self) -> &str {
        match self {
            ParseBinaryError::InvalidEncoding => {
                "Binary data is encoded with the standard Base64 encoding \
                 ( [a..z] | [A..Z] | + | / | = (padding) )."
            }
            ParseBinaryError::MissingBytes => "Incorrect number of bytes were encoded.",
            ParseBinaryError::InvalidPadding => "Padding bytes were set too early or incorrectly."
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
                if index >= bytes.len() || bytes[index] != b'\n' {
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
    use super::{write_escaped_bytes, write_escaped_text};

    fn write_text(text: &str) -> Result<String, std::fmt::Error> {
        let mut buffer = String::with_capacity(text.len());
        write_escaped_text(&mut buffer, text)?;
        Ok(buffer)
    }

    fn write_bytes(text: &str) -> Result<String, std::io::Error> {
        let mut buffer = Vec::with_capacity(text.len());
        write_escaped_bytes(&mut buffer, text.as_bytes())?;
        Ok(String::from_utf8(buffer).unwrap())
    }

    #[test]
    fn escaped_chars_only() {
        let s = ",\r\n;\r:\\";
        let expected = "\\,\n\\;\n:\\\\";

        assert_eq!(expected, write_text(s).unwrap());
        assert_eq!(expected, write_bytes(s).unwrap());
    }

    #[test]
    fn non_unix_newlines() {
        // To handle newlines, implementations have to check the next byte. However,
        // incorrect indexing on a multi-byte character like 'ö' will panic in the text
        // version.
        let s = "\r\n\rö\r";
        let expected = "\n\nö\n";

        // assert_eq!(expected, write_text(s).unwrap());
        assert_eq!(expected, write_bytes(s).unwrap());
    }

    #[test]
    fn no_escaped_chars() {
        let s = "This is a simple sentence.";

        assert_eq!(s, write_text(s).unwrap());
        assert_eq!(s, write_bytes(s).unwrap());
    }

    // test run with default features enabled but should be correct regardless
    #[test]
    fn long_sentence() {
        let s = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n";
        let expected = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.\n";

        assert_eq!(expected, write_text(s).unwrap());
        assert_eq!(expected, write_bytes(s).unwrap());
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

/// `STATUS` Property Values
///
/// [Format definitions of statuses](https://tools.ietf.org/html/rfc5545#section-3.8.1.11)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StatusValue {
    /// `TENTATIVE`
    ///
    /// Status for a tentative event
    Tentative,
    /// `CONFIRMED`
    ///
    /// Status for a definite event
    Confirmed,
    /// `CANCELLED`
    ///
    /// Status for a cancelled Event, To-Do or Journal
    Cancelled,
    /// `NEEDS-ACTION`
    ///
    /// Status for a To-Do that needs action
    NeedsAction,
    /// `COMPLETED`
    ///
    /// Status for a completed To-Do
    Completed,
    /// `IN-PROCESS`
    ///
    /// Status for an in-process To-Do
    InProcess,
    /// `DRAFT`
    ///
    /// Status for a draft Journal
    Draft,
    /// `FINAL`
    ///
    /// Status for a final Journal
    Final
}

impl StatusValue {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            StatusValue::Tentative => "TENTATIVE",
            StatusValue::Confirmed => "CONFIRMED",
            StatusValue::Cancelled => "CANCELLED",
            StatusValue::NeedsAction => "NEEDS-ACTION",
            StatusValue::Completed => "COMPLETED",
            StatusValue::InProcess => "IN-PROCESS",
            StatusValue::Draft => "DRAFT",
            StatusValue::Final => "FINAL"
        }
    }
}

/// `Transp` Property Values
///
/// [Format definitions of time transparency](https://tools.ietf.org/html/rfc5545#section-3.8.2.7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TranspValue {
    /// `OPAQUE`
    ///
    /// Blocks or opaque on busy time searches. Default value is OPAQUE.
    Opaque,
    /// `TRANSPARENT`
    ///
    /// Transparent on busy time searches.
    Transparent
}

impl TranspValue {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            TranspValue::Opaque => "OPAQUE",
            TranspValue::Transparent => "TRANSPARENT"
        }
    }
}

impl Default for TranspValue {
    fn default() -> Self {
        TranspValue::Opaque
    }
}
