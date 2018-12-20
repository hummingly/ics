// Content lines must be folded after 75 bytes
pub const LIMIT: usize = 75;

pub fn fold(content: &mut String) {
    // drain the first 75 bytes or before if the boundary is not on a char boundary
    let mut boundary = next_boundary(content.as_bytes(), LIMIT);
    let input: String = content.drain(boundary..).collect();
    content.push_str("\r\n ");

    let len = input.len();
    boundary = 0;
    while boundary < len {
        let start = boundary;
        boundary += LIMIT;
        if boundary < len {
            boundary = next_boundary(input.as_bytes(), boundary);
            content.push_str(&input[start..boundary]);
            content.push_str("\r\n ");
        } else {
            content.push_str(&input[start..len]);
        }
    }
}

// Returns the next char boundary at or before index
fn next_boundary(bytes: &[u8], index: usize) -> usize {
    // 'The start and end of the string are considered to be boundaries.'
    if index == 0 {
        return index;
    } else if index >= bytes.len() {
        return bytes.len();
    }
    bytes[0..=index]
        .iter()
        .rposition(|&i| (i as i8) >= -0x40) // bit magic i < 128 || i >= 192
        .unwrap_or(0)
}

// Calculates the new text length after inserting a Line Break
pub fn size(len: usize) -> usize {
    if len % LIMIT == 0 {
        len + (len / LIMIT - 1) * 3
    } else {
        len + (len / LIMIT) * 3
    }
}

#[cfg(test)]
mod tests {
    use super::fold;
    use super::size;
    use super::LIMIT;

    // There are no test for short input because the function is only called once
    // when the length is longer than the LIMIT! The contentline method is
    // also tested partially to see if space is allocated properly.
    #[test]
    fn over_limit() {
        let mut line = String::from("Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.");
        let len = line.len();
        line.reserve_exact(size(len) - len);
        assert!(line.len() > LIMIT);
        fold(&mut line);

        let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";
        assert_eq!(line, expected);
        assert_eq!(size(len), expected.len());
        assert_eq!(line.capacity(), expected.len());
    }

    #[test]
    fn multibytes() {
        let mut line = String::from(
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎."
        );
        let len = line.len();
        line.reserve_exact(size(len) - len);
        assert!(line.len() > LIMIT);
        fold(&mut line);

        let expected =
            "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";
        assert_eq!(line, expected);
        assert_eq!(size(len), expected.len());
        assert_eq!(line.capacity(), expected.len());
    }

    #[test]
    fn multi_lines() {
        let mut line = String::from("The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ");
        let len = line.len();
        line.reserve_exact(size(len) - len);
        assert!(line.len() > LIMIT);
        fold(&mut line);

        let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown\r\n  fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";
        assert_eq!(line, expected);
        assert_eq!(size(len), expected.len());
        assert_eq!(line.capacity(), expected.len());
    }
}
