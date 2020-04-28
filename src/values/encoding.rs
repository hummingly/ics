use std::borrow::Cow;
use std::fmt;

// Mask for extracting 6 bits from a byte.
const BIT_MASK: u8 = 0b0011_1111;

const BASE_64: [char; 64] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9', '+', '/'
];

/// Encodes bytes into a string using the Base64 standard encoding.
pub(crate) fn encode_base64<W: fmt::Write>(output: &mut W, input: &[u8]) -> fmt::Result {
    if input.is_empty() {
        return Ok(());
    }
    // TODO: Replace with chunks_exact when updating rustc
    let mut bytes = input.chunks(3);
    let len = bytes.len() - 1;
    for _ in 0..len {
        encode_chunk(output, bytes.next().unwrap())?
    }

    if let Some(remainder) = bytes.next() {
        match remainder {
            &[first] => {
                output.write_char(BASE_64[usize::from(first >> 2)])?;
                output.write_char(BASE_64[usize::from(first << 4 & BIT_MASK)])?;
                output.write_str("==")?;
            }
            &[first, second] => {
                output.write_char(BASE_64[usize::from(first >> 2)])?;
                output.write_char(BASE_64[usize::from(first << 4 & BIT_MASK | second >> 4)])?;
                output.write_char(BASE_64[usize::from(second << 2 & BIT_MASK)])?;
                output.write_char('=')?;
            }
            chunk => encode_chunk(output, chunk)?
        }
    }
    Ok(())
}

fn encode_chunk<W: fmt::Write>(output: &mut W, chunk: &[u8]) -> fmt::Result {
    // Since we cannot use unsafe and get chunks as arrays, we need to give the
    // compiler a hint.
    assert_eq!(chunk.len(), 3);
    output.write_char(BASE_64[usize::from(chunk[0] >> 2)])?;
    output.write_char(BASE_64[usize::from(chunk[0] << 4 & BIT_MASK | chunk[1] >> 4)])?;
    output.write_char(BASE_64[usize::from(chunk[1] << 2 & BIT_MASK | chunk[2] >> 6)])?;
    output.write_char(BASE_64[usize::from(chunk[2] & BIT_MASK)])
}

/// Decodes a Base64 encoded string into bytes.
///
/// The function assumes that input is an ASCII string containing only
/// characters that are in the Base64 alphabet. Furthermore, the string must be
/// properly padded, otherwise data will be lost.
pub(crate) fn decode_base64(output: &mut Vec<u8>, input: &str) {
    if input.is_empty() {
        return;
    }
    let mut bytes = input.as_bytes().chunks(4);
    let len = bytes.len() - 1;
    for _ in 0..len {
        decode_chunk(output, bytes.next().unwrap())
    }
    if let Some(remainder) = bytes.next() {
        match remainder {
            &[first, second, b'=', b'='] => {
                output.push(to_binary(first) << 2 | to_binary(second) >> 4);
            }
            &[first, second, third, b'='] => {
                let second = to_binary(second);
                output.push(to_binary(first) << 2 | second >> 4);
                output.push(second << 4 | to_binary(third) >> 2);
            }
            chunk => decode_chunk(output, chunk)
        }
    }
}

fn decode_chunk(output: &mut Vec<u8>, chunk: &[u8]) {
    assert_eq!(chunk.len(), 4);
    let (second, third) = (to_binary(chunk[1]), to_binary(chunk[2]));
    output.push(to_binary(chunk[0]) << 2 | second >> 4);
    output.push(second << 4 | third >> 2);
    output.push(third << 6 | to_binary(chunk[3]));
}

fn to_binary(c: u8) -> u8 {
    match c {
        b'A'..=b'Z' => c - b'A',
        b'a'..=b'z' => c - b'a' + 26,
        b'0'..=b'9' => c - b'0' + 52,
        b'+' => 62,
        b'/' => 63,
        _ => unreachable!()
    }
}

/// Escapes comma, semicolon and backlash character by prepending a backlash.
///
/// This method is used for properties with the value type "TEXT".
pub(crate) fn escape_text<'t>(text: Cow<'t, str>) -> Cow<'t, str> {
    let matches = |c| c == '\r' || is_escaped_char(c);
    if text.contains(matches) {
        let text = text.replace("\r\n", "\n");
        let size = text.len() + text.chars().filter(|&c| is_escaped_char(c)).count();
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

fn is_escaped_char(c: char) -> bool {
    c == ',' || c == ';' || c == '\\'
}
