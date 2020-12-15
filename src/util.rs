use std::io::{Error, Write};

/// Write bytes using the Base64 standard encoding.
pub fn write_base64<W: Write>(output: &mut W, bytes: &[u8]) -> Result<(), Error> {
    // Mask for extracting 6 bits from a byte.
    const BIT_MASK: u8 = 0b0011_1111;

    const BASE_64: [u8; 64] = [
        b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N', b'O',
        b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z', b'a', b'b', b'c', b'd',
        b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n', b'o', b'p', b'q', b'r', b's',
        b't', b'u', b'v', b'w', b'x', b'y', b'z', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
        b'8', b'9', b'+', b'/'
    ];

    if bytes.is_empty() {
        return Ok(());
    }

    let mut chunks = bytes.chunks_exact(3);
    while let Some(chunk) = chunks.next() {
        let first = BASE_64[usize::from(chunk[0] >> 2)];
        let second = BASE_64[usize::from(chunk[0] << 4 & BIT_MASK | chunk[1] >> 4)];
        let third = BASE_64[usize::from(chunk[1] << 2 & BIT_MASK | chunk[2] >> 6)];
        let fourth = BASE_64[usize::from(chunk[2] & BIT_MASK)];
        output.write_all(&[first, second, third, fourth])?;
    }

    match chunks.remainder() {
        [first] => {
            let first_char = BASE_64[usize::from(first >> 2)];
            let second = BASE_64[usize::from(first << 4 & BIT_MASK)];
            output.write_all(&[first_char, second, b'=', b'='])
        }
        [first, second] => {
            let first_char = BASE_64[usize::from(first >> 2)];
            let second_char = BASE_64[usize::from(first << 4 & BIT_MASK | second >> 4)];
            let third = BASE_64[usize::from(second << 2 & BIT_MASK)];
            output.write_all(&[first_char, second_char, third, b'='])
        }
        _ => Ok(())
    }
}

/// Escapes the comma, semicolon and backlash character and normalizes newlines
/// by replacing them with linefeed character.
pub fn write_escaped_text<W: Write>(writer: &mut W, text: &str) -> Result<(), Error> {
    let text = text.as_bytes();
    let mut last_end = 0;
    for (start, part) in EscapeByteIndices::new(text) {
        writer.write_all(&text[last_end..start])?;
        match part {
            b'\r' => {
                // Replace old macOS newline character with a line feed character otherwise
                // discard the carriage return character for Windows OS newlines.
                if text.get(start + 1) != Some(&b'\n') {
                    writer.write_all(b"\n")?;
                }
            }
            b => writer.write_all(&[b'\\', b])?
        }
        last_end = start + 1;
    }
    writer.write_all(&text[last_end..])
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
mod binary {
    use super::write_base64;
    use std::io::Error;

    fn write_binary(binary: &[u8]) -> Result<String, Error> {
        let mut buffer = Vec::with_capacity(binary.len() + binary.len() / 3);
        write_base64(&mut buffer, binary)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    // https://tools.ietf.org/html/rfc4648#section-10
    #[test]
    fn encode_rfc4648_test_sample() -> Result<(), Error> {
        assert_eq!(write_binary(b"")?, "");
        assert_eq!(write_binary(b"f")?, "Zg==");
        assert_eq!(write_binary(b"fo")?, "Zm8=");
        assert_eq!(write_binary(b"foo")?, "Zm9v");
        assert_eq!(write_binary(b"foob")?, "Zm9vYg==");
        assert_eq!(write_binary(b"fooba")?, "Zm9vYmE=");
        assert_eq!(write_binary(b"foobar")?, "Zm9vYmFy");
        Ok(())
    }

    #[test]
    fn encode_text() -> Result<(), Error> {
        let input = "Polyfon zwitschernd aßen Mäxchens Vögel Rüben, Joghurt und Quark";
        let expected = "UG9seWZvbiB6d2l0c2NoZXJuZCBhw59lbiBNw6R4Y2hlbnMgVsO2Z2VsIFLDvGJlbiwgSm9naHVydCB1bmQgUXVhcms=";
        let output = write_binary(input.as_bytes())?;
        assert_eq!(output, expected);
        Ok(())
    }
}

#[cfg(test)]
mod text {
    use super::write_escaped_text;
    use std::io::Error;

    fn write_text(text: &str) -> Result<String, Error> {
        let mut buffer = Vec::with_capacity(text.len());
        write_escaped_text(&mut buffer, text)?;
        Ok(String::from_utf8_lossy(&buffer).to_string())
    }

    #[test]
    fn escaped_chars_only() -> Result<(), Error> {
        let s = ",\r\n;\r:\\";
        let expected = "\\,\n\\;\n:\\\\";

        assert_eq!(expected, write_text(s)?);
        Ok(())
    }

    #[test]
    fn non_unix_newlines() -> Result<(), Error> {
        // To handle newlines, implementations have to check the next byte. However,
        // indexing must not panic.
        let s = "\r\n\rö\r";
        let expected = "\n\nö\n";

        assert_eq!(expected, write_text(s)?);
        Ok(())
    }

    #[test]
    fn no_escaped_chars() -> Result<(), Error> {
        let s = "This is a simple sentence.";

        assert_eq!(s, write_text(s)?);
        Ok(())
    }

    // test run with default features enabled but should be correct regardless
    #[test]
    fn long_sentence() -> Result<(), Error> {
        let s = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n";
        let expected = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.\n";

        assert_eq!(expected, write_text(s)?);
        Ok(())
    }
}
