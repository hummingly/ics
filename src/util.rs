use regex::Regex;
use std::borrow::Cow;

const LIMIT: usize = 75;

pub(crate) fn fold_line(content: String) -> String {
    let len = content.len();

    if len <= LIMIT {
        return content;
    }

    let mut boundary = 0;
    let mut folded = String::with_capacity(len + (len / LIMIT) * 3);

    while boundary < len {
        let start = boundary;
        boundary += LIMIT;
        if boundary > len {
            boundary = len;
        } else {
            while !content.is_char_boundary(boundary) {
                boundary -= 1;
            }
        }

        folded.push_str(&content[start..boundary]);
        if boundary < len {
            folded.push_str("\r\n ");
        }
    }
    folded
}

#[allow(dead_code)]
pub(crate) fn escape_text<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    // Windows newline character encoding replaced
    let input = input.into();

    if input.contains("\r\n") {
        let input = input.replace("\r\n", "\n");

        fn to_escape(c: char) -> bool {
            c == ',' || c == ';' || c == '\\'
        }

        if input.contains(to_escape) {
            let mut output = String::with_capacity(input.len() + (input.len() / 2));
            for c in input.chars() {
                match c {
                    ',' => output.push_str("\\,"),
                    ';' => output.push_str("\\;"),
                    '\\' => output.push_str("\\\\"),
                    // \r was in old MacOS versions the newline characters
                    '\r' => output.push_str("\n"),
                    _ => output.push(c),
                }
            }
            return Cow::Owned(output);
        }
    }
    input.into()
}

// https://lise-henry.github.io/articles/optimising_strings.html
pub(crate) fn escape_cow<'a, S: Into<Cow<'a, str>>>(input: S) -> Cow<'a, str> {
    lazy_static! {
        // TODO: optionally add /r for old mac systems
        static ref REGEX: Regex = Regex::new("[,;\\\\]|\r\n|\r").unwrap();
    }

    let input = input.into();
    let mut last_match = 0;

    if REGEX.is_match(&input) {
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
                _ => unreachable!(),
            }
            last_match = m.end();
        }
        output.push_str(&input[last_match..]);
        Cow::Owned(output)
    } else {
        input
    }
}

// TODO: better tests
#[cfg(test)]
mod line_folding_tests {
    use super::fold_line;

    #[test]
    fn fold_line_short() {
        let line = "This is a short line";
        assert_eq!(line.clone(), fold_line(line.to_owned()));
    }

    #[test]
    fn fold_line_exactly75() {
        let line = "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 ";
        assert_eq!(line.clone(), fold_line(line.to_owned()));
    }

    #[test]
    fn fold_line_over75() {
        let line = "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515ö";
        let expected = String::from(
            "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515\r\n ö",
        );
        assert_eq!(expected, fold_line(line.to_owned()));
    }

    #[test]
    fn fold_line_multibyte() {
        let line = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let expected = String::from(
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.",
        );
        assert_eq!(expected, fold_line(line.to_owned()));
    }

    #[test]
    fn fold_line_multi_lines() {
        let line = "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 ";

        let expected = String::from("1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 \r\n 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 \r\n 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 ");

        assert_eq!(expected, fold_line(line.to_owned()));
    }

}

#[cfg(test)]
mod escape_text_tests {
    use super::escape_cow;
    #[allow(unused_imports)]
    use super::escape_text;

    #[test]
    fn escape_string() {
        let s = String::from(",\r\n;:\\ \n \r\n");
        let expected = String::from("\\,\n\\;:\\\\ \n \n");
        assert_eq!(expected, escape_text(&s));
    }

    #[test]
    fn escape_regex_cow() {
        let s = String::from(",\r\n;:\\ \n \r\n");
        let expected = String::from("\\,\n\\;:\\\\ \n \n");
        assert_eq!(expected, escape_cow(s));
    }

    use components::Property;
    #[test]
    fn escape_property() {
        let expected_value = "Hello\\, World! Today is a beautiful day to test: Escape methods.\n Characters like \\; or \\\\ must be escaped.\n";
        let property = Property::new("COMMENT", "Hello, World! Today is a beautiful day to test: Escape methods.\n Characters like ; or \\ must be escaped.\r\n");
        assert_eq!(expected_value, property.value);
    }
}
