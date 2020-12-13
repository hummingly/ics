use std::io::{Error, Write};

/// Write bytes using the Base64 standard encoding.
pub fn write_base64<W: Write>(output: &mut W, bytes: &[u8]) -> Result<(), Error> {
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
        let first = BASE_64[usize::from(chunk[0] >> 2)];
        let second = BASE_64[usize::from(chunk[0] << 4 & BIT_MASK | chunk[1] >> 4)];
        let third = BASE_64[usize::from(chunk[1] << 2 & BIT_MASK | chunk[2] >> 6)];
        let fourth = BASE_64[usize::from(chunk[2] & BIT_MASK)];
        write!(output, "{}{}{}{}", first, second, third, fourth)?;
    }

    match chunks.remainder() {
        [first] => {
            let first_char = BASE_64[usize::from(first >> 2)];
            let second = BASE_64[usize::from(first << 4 & BIT_MASK)];
            write!(output, "{}{}==", first_char, second)
        }
        [first, second] => {
            let first_char = BASE_64[usize::from(first >> 2)];
            let second_char = BASE_64[usize::from(first << 4 & BIT_MASK | second >> 4)];
            let third = BASE_64[usize::from(second << 2 & BIT_MASK)];
            write!(output, "{}{}{}=", first_char, second_char, third)
        }
        _ => Ok(())
    }
}

/// Escapes the comma, semicolon and backlash character and normalizes newlines
/// by replacing them with linefeed character.
pub fn write_escaped_text<W: Write>(writer: &mut W, text: &str) -> Result<(), Error> {
    let mut last_end = 0;
    for (start, part) in EscapeByteIndices::new(text.as_bytes()) {
        write!(writer, "{}", &text[last_end..start])?;
        match part {
            b'\r' => {
                // Replace old macOS newline character with a line feed character otherwise
                // discard carriage return character for Windows OS.
                // WARNING: Do not implement this with slicing instead of str::get. Indexing
                // outside a char boundary will panic!
                if text.get(start + 1..start + 2) != Some("\n") {
                    write!(writer, "{}", "\n")?;
                }
            }
            b => write!(writer, "\\{}", char::from(b))?
        }
        last_end = start + 1;
    }
    write!(writer, "{}", &text[last_end..])
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
        // incorrect indexing on a multi-byte character like 'ö' will panic in the text
        // version.
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
