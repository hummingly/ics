use std::borrow::Cow;

/// Escapes comma, semicolon, backslash and newline character by prepending a
/// backslash. Newlines are normalized to a line feed character.
///
/// This method is only necessary for properties with the value type "TEXT".
///
/// # Example
/// ```
/// use ics::escape_text;
///
/// let line = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.";
/// let expected = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\\n Characters like \\; or \\\\ must be escaped.";
/// assert_eq!(expected, escape_text(line));
pub fn escape_text<'a, S>(input: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>
{
    let input = input.into();
    let mut escaped_chars_count = 0;
    let mut has_carriage_return_char = false;

    for b in input.bytes() {
        if b == b',' || b == b';' || b == b'\\' || b == b'\n' {
            escaped_chars_count += 1;
        } else if b == b'\r' {
            has_carriage_return_char = true;
        }
    }

    if has_carriage_return_char || escaped_chars_count > 0 {
        let escaped_chars = |c| c == ',' || c == ';' || c == '\\' || c == '\r' || c == '\n';
        let mut output = String::with_capacity(input.len() + escaped_chars_count);
        let mut last_end = 0;
        for (start, part) in input.match_indices(escaped_chars) {
            output.push_str(&input[last_end..start]);
            match part {
                // \r was in old MacOS versions the newline character
                "\r" => {
                    if input.get(start + 1..start + 2) != Some("\n") {
                        output.push_str("\\n");
                    }
                }
                // Newlines needs to be escaped to the literal `\n`
                "\n" => {
                    output.push_str("\\n");
                }
                c => {
                    output.push('\\');
                    output.push_str(c);
                }
            }
            last_end = start + part.len();
        }
        output.push_str(&input[last_end..input.len()]);
        Cow::Owned(output)
    } else {
        input
    }
}

#[cfg(test)]
mod escape_text_tests {
    use super::escape_text;

    #[test]
    fn escaped_chars() {
        let s = ",\r\n;:\\ \r\n\rö\r";
        let expected = "\\,\\n\\;:\\\\ \\n\\nö\\n";
        assert_eq!(expected, escape_text(s));
    }

    #[test]
    fn no_escaped_chars() {
        let s = "This is a simple sentence.";
        let expected = s.clone();
        assert_eq!(expected, escape_text(s));
    }

    // test run with default features enabled but should be correct regardless
    #[test]
    fn escape_property() {
        use components::Property;

        let expected_value = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\\n Characters like \\; or \\\\ must be escaped.\\n";
        let property = Property::new(
            "COMMENT",
            escape_text("Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n")
        );
        assert_eq!(expected_value, property.value);
    }
}
