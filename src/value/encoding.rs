use std::borrow::Cow;

// Mask for extracting 6 bits from a byte.
const BIT_MASK: u8 = 0b0011_1111;

const BASE_64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/'
];

pub fn encode_base64(binary: &[u8]) -> String {
    if binary.is_empty() {
        return String::new();
    }

    let mut output = String::with_capacity(binary.len() + binary.len() / 3);

    // TODO: Replace with chunks_exact when updating rustc
    let mut bytes = binary.chunks(3);
    let len = bytes.len() - 1;

    for _ in 0..len {
        let values = bytes.next().unwrap();
        output.extend(encode_chunk(values).iter())
    }

    if let Some(remainder) = bytes.next() {
        match remainder {
            chunk @ &[_, _, _] => output.extend(encode_chunk(chunk).iter()),
            &[first, second] => {
                output.push(BASE_64[usize::from(first >> 2)]);
                output.push(BASE_64[usize::from(first << 4 & 0b0011_1111 | second >> 4)]);
                output.push(BASE_64[usize::from(second << 2 & 0b0011_1111)]);
                output.push('=');
            }
            &[first] => {
                output.push(BASE_64[usize::from(first >> 2)]);
                output.push(BASE_64[usize::from(first & 0b0000_0011 << 4)]);
                output.push_str("==");
            }
            _ => unreachable!()
        }
    };
    output
}

fn encode_chunk(chunk: &[u8]) -> [char; 4] {
    let first = usize::from(chunk[0] >> 2);
    let second = usize::from(chunk[0] << 4 & BIT_MASK | chunk[1] >> 4);
    let third = usize::from(chunk[1] << 2 & BIT_MASK | chunk[2] >> 6);
    let fourth = usize::from(chunk[2] & BIT_MASK);
    [
        BASE_64[first],
        BASE_64[second],
        BASE_64[third],
        BASE_64[fourth]
    ]
}

#[cfg(test)]
mod binary {
    use super::encode_base64;

    #[test]
    fn text() {
        let input = "Polyfon zwitschernd aßen Mäxchens Vögel Rüben, Joghurt und Quark".as_bytes();
        let expected = "UG9seWZvbiB6d2l0c2NoZXJuZCBhw59lbiBNw6R4Y2hlbnMgVsO2Z2VsIFLDvGJlbiwgSm9naHVydCB1bmQgUXVhcms=";
        assert_eq!(encode_base64(input), expected);
    }
}

/// Escapes comma, semicolon and backlash character by prepending a backlash.
///
/// This method is only necessary for properties with the value type "TEXT".
///
/// # Example
/// ```
/// use ics::escape_text;
///
/// let line = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.";
/// let expected = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.";
/// assert_eq!(expected, escape_text(line));
pub fn escape_text<'t>(text: Cow<'t, str>) -> Cow<'t, str> {
    let matches = |c| c == '\r' || is_escaped_char(&c);
    if text.contains(matches) {
        let text = text.replace("\r\n", "\n");
        let size = text.len() + text.chars().filter(is_escaped_char).count();
        let mut output = String::with_capacity(size);
        let mut last_end = 0;
        for (start, part) in text.match_indices(matches) {
            output.push_str(&text[last_end..start]);
            match part {
                "," => output.push_str("\\,"),
                ";" => output.push_str("\\;"),
                "\\" => output.push_str("\\\\"),
                // \r was in old MacOS versions the newline character
                "\r" => output.push_str("\n"),
                _ => unreachable!()
            }
            last_end = start + part.len();
        }
        output.push_str(&text[last_end..text.len()]);
        return Cow::Owned(output);
    }
    text
}

fn is_escaped_char(c: &char) -> bool {
    c == &',' || c == &';' || c == &'\\'
}

#[cfg(test)]
mod text {
    use super::escape_text;

    #[test]
    fn escaped_chars() {
        let s = ",\r\n;:\\ \n \r\n";
        let expected = "\\,\n\\;:\\\\ \n \n";
        assert_eq!(expected, escape_text(s.into()));
    }

    #[test]
    fn no_escaped_chars() {
        let s = "This is a simple sentence.";
        let expected = s.clone();
        assert_eq!(expected, escape_text(s.into()));
    }

    // test run with default features enabled but should be correct regardless
    #[test]
    fn escape_property() {
        use components::Property;

        let s = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n";
        let expected_value = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.\n";
        let property = Property::new("COMMENT", escape_text(s.into()));
        assert_eq!(expected_value, property.value);
    }
}
