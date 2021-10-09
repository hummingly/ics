//! Algorithms for content lines.
use std::fmt;

// Content lines must be folded after around 75 bytes by inserting a carriage
// return and line feed followed by whitespace. This crate uses a space
// character as white space but it could also be a horizontal tab.
pub const LIMIT: usize = 75;
const LINE_BREAK: &str = "\r\n ";

pub fn fold<W: fmt::Write>(writer: &mut W, mut content: &str) -> fmt::Result {
    let mut boundary = next_boundary(content);
    writer.write_str(&content[..boundary])?;

    while boundary < content.len() {
        content = &content[boundary..];
        writer.write_str(LINE_BREAK)?;
        let next_boundary = next_boundary(content);
        writer.write_str(&content[..next_boundary])?;
        boundary = next_boundary;
    }
    Ok(())
}

// TODO: unfold algorithm

fn next_boundary(input: &str) -> usize {
    let input = input.as_bytes();
    if LIMIT >= input.len() {
        return input.len();
    }
    match input[..=LIMIT]
        .iter()
        .rposition(|&i| !(128..192).contains(&i))
    {
        Some(0) | None => input.len(),
        Some(index) => index
    }
}

// Calculates the new estimated text length after inserting line breaks
pub fn size(len: usize) -> usize {
    if len % LIMIT == 0 {
        len + (len / LIMIT - 1) * 3
    } else {
        len + (len / LIMIT) * 3
    }
}

#[cfg(test)]
mod tests {
    use super::{fold, size};

    #[test]
    fn no_linebreak() {
        let content = "No line break today.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();

        assert_eq!(line, content);
    }

    #[test]
    fn over_limit() {
        let content = "Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";

        assert_eq!(line, expected);
    }

    #[test]
    fn multibytes() {
        let content = "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";

        assert_eq!(line, expected);
    }

    #[test]
    fn multibytes_with_space() {
        let content =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老 虎.";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老 \r\n 虎.";

        assert_eq!(line, expected);
    }

    #[test]
    fn multi_lines() {
        let content = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";
        let mut line = String::with_capacity(size(content.len()));
        fold(&mut line, content).unwrap();
        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy cog. The quick brown fox jumps over the lazy hog. The quick brown\r\n  fox jumps over the lazy log. The quick brown fox jumps over the lazy dog. ";

        assert_eq!(line, expected);
    }
}
