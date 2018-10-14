pub(crate) fn fold_line(content: &str) -> String {
    let len = content.len();
    let limit = 75;

    if len <= limit {
        return content.to_owned();
    }

    let mut start = 0;
    let mut boundary = 0;
    let mut folded = String::with_capacity(len + (len / limit) * 3);

    while boundary < len {
        boundary += limit;
        while !content.is_char_boundary(boundary) {
            boundary -= 1;
        }

        if let Some(substring) = content.get(start..boundary) {
            folded.push_str(substring);
            if boundary < len {
                folded.push_str("\r\n ");
            }
            start = boundary;
        }
    }
    folded
}

#[allow(dead_code)]
pub fn escape_text(input: &str) -> String {
    // Windows newline character encoding replaced
    let s = input.replace("\r\n", "\n");
    let len = s.len() + s
        .chars()
        .filter(|&c| c == ',' || c == ';' || c == '\\' || c == '\n')
        .count();

    let mut output = String::with_capacity(len);

    for c in s.chars() {
        match c {
            ',' | ';' | '\\' => {
                output.push('\\');
                output.push(c);
            }
            // \r was in previous MacOS versions the newline characters
            '\n' | '\r' => output.push_str("\\n"),
            _ => output.push(c),
        }
    }
    output
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::escape_text;
    use super::fold_line;

    #[test]
    fn fold_line_short() {
        let line = "This is a short line";
        assert_eq!(line.clone(), fold_line(&line));
    }

    #[test]
    fn fold_line_exactly75() {
        let line = "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 ";
        assert_eq!(line.clone(), fold_line(&line));
    }

    #[test]
    fn fold_line_over75() {
        let line = "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515ö";
        let expected = String::from(
            "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515\r\n ö",
        );
        assert_eq!(expected, fold_line(&line));
    }

    #[test]
    fn fold_line_multibyte() {
        let line = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let expected = String::from(
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.",
        );
        assert_eq!(expected, fold_line(&line));
    }

    #[test]
    fn fold_line_multi_lines() {
        let line = "1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 ";

        let expected = String::from("1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 \r\n 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 \r\n 1111 2222 3333 4444 5555 6666 7777 8888 9999 1010 1111 1212 1313 1414 1515 ");

        assert_eq!(expected, fold_line(&line));
    }

    #[test]
    fn escape() {
        let s = String::from(",\r\n;:\\ \n \r\n");
        let expected = String::from("\\,\\n\\;:\\\\ \\n \\n");
        assert_eq!(expected, escape_text(&s));
    }
}
