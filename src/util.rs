use std::borrow::Cow;

/// Escapes comma, semicolon and backlash character with a backlash.
///
/// This method is only necessary for properties with the value type "TEXT".
///
/// #Example
/// ```
/// use ics::escape_text;
///
/// let line = "Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.";
/// let expected = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.";
/// assert_eq!(expected, escape_text(line));
pub fn escape_text<'a, S>(input: S) -> Cow<'a, str>
where
    S: Into<Cow<'a, str>>
{
    if cfg!(feature = "fast_text") {
        #[cfg(feature = "fast_text")]
        return escape_value_regex(input.into());
    }
    escape_value(input.into())
}

fn escape_value(mut input: Cow<str>) -> Cow<str> {
    if input.contains("\r\n") {
        input = input.replace("\r\n", "\n").into();
    }

    let escaped_chars = |c| c == ',' || c == ';' || c == '\\';
    if let Some(index) = input.find(|c| c == '\r' || escaped_chars(c)) {
        let extra_bytes = input.chars().filter(|&c| escaped_chars(c)).count();
        let mut output = String::with_capacity(input.len() + extra_bytes);
        output.push_str(&input[0..index]);

        for c in input[index..].chars() {
            match c {
                ',' => output.push_str("\\,"),
                ';' => output.push_str("\\;"),
                '\\' => output.push_str("\\\\"),
                // \r was in old MacOS versions the newline characters
                '\r' => output.push_str("\n"),
                _ => output.push(c)
            }
        }
        Cow::Owned(output)
    } else {
        input
    }
}

// https://lise-henry.github.io/articles/optimising_strings.html
#[cfg(feature = "fast_text")]
fn escape_value_regex(input: Cow<str>) -> Cow<str> {
    use regex::Regex;

    lazy_static! {
        static ref REGEX: Regex = Regex::new("[,;\\\\]|\r\n|\r").unwrap();
    }

    if REGEX.is_match(&input) {
        let escaped_chars = |c| c == ',' || c == ';' || c == '\\';
        let extra_bytes = input.chars().filter(|&c| escaped_chars(c)).count();
        let mut output = String::with_capacity(input.len() + extra_bytes);

        let mut last_match = 0;
        let matches = REGEX.find_iter(&input);
        for m in matches {
            output.push_str(&input[last_match..m.start()]);
            match &input[m.start()..m.end()] {
                "," => output.push_str("\\,"),
                ";" => output.push_str("\\;"),
                "\\" => output.push_str("\\\\"),
                "\r\n" => output.push_str("\n"),
                // \r was in old MacOS versions the newline characters
                "\r" => output.push_str("\n"),
                _ => unreachable!()
            }
            last_match = m.end();
        }
        output.push_str(&input[last_match..]);
        output.shrink_to_fit();
        Cow::Owned(output)
    } else {
        input
    }
}

#[cfg(test)]
mod escape_text_tests {
    use super::escape_text;
    use super::escape_value;
    #[cfg(feature = "fast_text")]
    use super::escape_value_regex;

    #[test]
    fn escaped_chars() {
        let s = ",\r\n;:\\ \n \r\n";
        let expected = "\\,\n\\;:\\\\ \n \n";
        assert_eq!(expected, escape_value(s.into()));
        #[cfg(feature = "fast_text")]
        assert_eq!(expected, escape_value_regex(s.into()));
    }

    #[test]
    fn no_escaped_chars() {
        let s = "This is a simple sentence.";
        let expected = s.clone();
        assert_eq!(expected, escape_value(s.into()));
        #[cfg(feature = "fast_text")]
        assert_eq!(expected, escape_value_regex(s.into()));
    }

    // test run with default features enabled but should be correct regardless
    #[test]
    fn escape_property() {
        use components::Property;

        let expected_value = "Hello\\, World! Today is a beautiful day to test: Escape Methods.\n Characters like \\; or \\\\ must be escaped.\n";
        let property = Property::new(
            "COMMENT",
            escape_text("Hello, World! Today is a beautiful day to test: Escape Methods.\n Characters like ; or \\ must be escaped.\r\n")
        );
        assert_eq!(expected_value, property.value);
    }
}
