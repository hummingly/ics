// Content lines must be folded after 75 bytes
pub const LIMIT: usize = 75;

pub fn fold(content: &mut String) {
    if content.len() > LIMIT {
        // drain until the first char boundary closest to the limit
        if let Some(first_boundary) = next_boundary(&content, LIMIT) {
            let input = content.split_off(first_boundary);
            let len = input.len();

            let mut boundary = 0;
            while boundary < len {
                content.push_str("\r\n ");
                let index = boundary + LIMIT;
                boundary = if index < len {
                    let next_boundary = next_boundary(&input, index).unwrap_or(len);
                    content.push_str(&input[boundary..next_boundary]);
                    next_boundary
                } else {
                    content.push_str(&input[boundary..len]);
                    len
                };
            }
        }
    }
}

fn next_boundary(input: &str, index: usize) -> Option<usize> {
    input.as_bytes()[..=index]
        .iter()
        .rposition(|&i| i < 128 || i >= 192)
        .filter(|&x| x > 0)
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
        let mut line = String::from("No line break today.");
        let len = line.len();
        fold(&mut line);

        let expected = line.clone();
        assert_eq!(line, expected);
        assert_eq!(size(len), expected.len());
    }

    #[test]
    fn over_limit() {
        let mut line = String::from("Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.");
        let len = line.len();
        fold(&mut line);

        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";
        assert_eq!(line, expected);
        assert_eq!(size(len), expected.len());
    }

    #[test]
    fn multibytes() {
        let mut line = String::from(
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎."
        );
        fold(&mut line);

        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";
        assert_eq!(line, expected);
    }

    #[test]
    fn multi_lines() {
        let mut line = String::from("The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ");
        let len = line.len();
        fold(&mut line);

        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown\r\n  fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";
        assert_eq!(line, expected);
        assert_eq!(size(len), expected.len());
    }
}
