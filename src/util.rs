use regex::Regex;
use std::borrow::Cow;

pub(crate) const LINE_LIMIT: usize = 75;
pub(crate) fn fold_line(content_line: &mut String) {
    let input = content_line.clone();
    content_line.clear();

    let len = input.len();
    let mut boundary = 0;
    while boundary < len {
        let start = boundary;
        boundary += LINE_LIMIT;
        if boundary > len {
            boundary = len;
        } else {
            while !input.is_char_boundary(boundary) {
                boundary -= 1;
            }
        }

        content_line.push_str(&input[start..boundary]);
        if boundary < len {
            content_line.push_str("\r\n ");
        }
    }
}

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
    let input = input.into();

    if cfg!(feature = "fast_encoding") {
        escape_value_regex(input)
    } else {
        escape_value(input)
    }
}

fn escape_value(mut input: Cow<str>) -> Cow<str> {
    if input.contains("\r\n") {
        input = input.replace("\r\n", "\n").into();
    }

    fn escaped_char(c: char) -> bool {
        c == ',' || c == ';' || c == '\\'
    }

    if input.contains(escaped_char) {
        let size = input.len() + input.chars().filter(|&c| escaped_char(c)).count();
        let mut output = String::with_capacity(size);
        for c in input.chars() {
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
fn escape_value_regex(input: Cow<str>) -> Cow<str> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("[,;\\\\]|\r\n|\r").unwrap();
    }

    if REGEX.is_match(&input) {
        let mut last_match = 0;
        let matches = REGEX.find_iter(&input);
        let mut output = String::with_capacity(input.len() + (input.len() / 2));
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

pub(crate) fn content_line_len(len: usize) -> usize {
    if len % LINE_LIMIT == 0 {
        len + ((len / LINE_LIMIT - 1) * 3)
    } else {
        len + ((len / LINE_LIMIT) * 3)
    }
}

#[cfg(test)]
mod line_folding_tests {
    use super::fold_line;
    use super::LINE_LIMIT;

    #[test]
    fn no_folding_short_line() {
        let mut line = String::from("This is a short line");
        let expected = line.clone();
        assert!(line.len() < LINE_LIMIT);
        fold_line(&mut line);
        assert_eq!(line, expected);
    }

    #[test]
    fn no_folding_at_limit() {
        let mut line = String::from(
            "Content lines that have a fixed length of 75 bytes shouldn't be line folded"
        );
        let expected = line.clone();
        assert!(line.len() == LINE_LIMIT);
        fold_line(&mut line);
        assert_eq!(line, expected);
    }

    #[test]
    fn folding_over_limit() {
        let mut line = String::from("Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.");
        assert!(line.len() > LINE_LIMIT);
        fold_line(&mut line);

        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";
        assert_eq!(line, expected);
    }

    #[test]
    fn folding_with_multibytes() {
        let mut line = String::from(
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎."
        );
        assert!(line.len() > LINE_LIMIT);
        fold_line(&mut line);

        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";
        assert_eq!(line, expected);
    }

    #[test]
    fn folding_multi_lines() {
        let mut line = String::from("The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ");
        assert!(line.len() > LINE_LIMIT);
        fold_line(&mut line);

        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown\r\n  fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";
        assert_eq!(line, expected);
    }

}

#[cfg(test)]
mod escape_text_tests {
    use super::escape_text;
    use super::escape_value;
    use super::escape_value_regex;

    #[test]
    fn escaped_chars() {
        let s = ",\r\n;:\\ \n \r\n";
        let expected = "\\,\n\\;:\\\\ \n \n";
        assert_eq!(expected, escape_value(s.into()));
        assert_eq!(expected, escape_value_regex(s.into()));
    }

    #[test]
    fn no_escaped_chars() {
        let s = "This is a simple sentence.";
        let expected = s.clone();
        assert_eq!(expected, escape_value(s.into()));
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
