//! Algorithms for content lines.
use std::fmt;

// Content lines must be folded after around 75 bytes by inserting a carriage
// return and line feed followed by whitespace. This crate uses a space
// character as white space but it could also be a horizontal tab.
pub const LIMIT: usize = 75;
const LINE_BREAK: &str = "\r\n ";

pub fn fold<W: fmt::Write>(writer: &mut W, content: &str) -> fmt::Result {
    let len = content.len();
    let first_boundary = next_boundary(&content, LIMIT).unwrap_or(len);
    write!(writer, "{}", &content[0..first_boundary])?;
    let mut boundary = first_boundary;

    while boundary < len {
        write!(writer, "{}", LINE_BREAK)?;
        let next_boundary = next_boundary(&content, boundary + LIMIT).unwrap_or(len);
        write!(writer, "{}", &content[boundary..next_boundary])?;
        boundary = next_boundary;
    }
    Ok(())
}

// TODO: unfold algorithm

fn next_boundary(input: &str, index: usize) -> Option<usize> {
    if index >= input.len() {
        return None;
    }
    // TODO: When updating rustc, use Option::filter
    match input.as_bytes()[..=index]
        .iter()
        .rposition(|&i| i < 128 || i >= 192)
    {
        Some(0) | None => None,
        index => index
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
