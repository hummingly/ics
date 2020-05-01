use super::error::ParseBinaryError;
use std::borrow::Cow;
// use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use values::encoding::{decode_base64, encode_base64, escape_text};

// INFO: https://tools.ietf.org/html/rfc2045#section-2.8
/// ICalendar Binary
///
/// Bytes are encoded with standard Base64 encoding.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Binary<'b>(Cow<'b, [u8]>);

impl<'b> Binary<'b> {
    /// Returns binary data as slice of bytes.
    pub fn get(&self) -> &[u8] {
        &self.0
    }
}

impl<'b> From<&'b [u8]> for Binary<'b> {
    fn from(value: &'b [u8]) -> Self {
        Binary(Cow::Borrowed(value))
    }
}

impl<'b> From<Vec<u8>> for Binary<'b> {
    fn from(value: Vec<u8>) -> Self {
        Binary(Cow::Owned(value))
    }
}

impl<'b> fmt::Display for Binary<'b> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        encode_base64(f, &self.0)
    }
}

impl<'b> FromStr for Binary<'b> {
    type Err = ParseBinaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Binary::from([].as_ref()));
        }

        // 24 bit groups are always encoded as 4 characters. Since shorter byte
        // sequences are always padded, we can assume the input has been set to a
        // multiple of 24.
        if s.len() % 4 > 0 {
            return Err(ParseBinaryError::MissingBytes);
        }

        let mut iter = s[..s.len() - 2]
            .bytes()
            .skip_while(|b| b.is_ascii_alphanumeric() || b == &b'+' || b == &b'/');

        match (iter.next(), iter.next(), iter.next()) {
            (None, _, _) | (Some(b'='), None, _) | (Some(b'='), Some(b'='), None) => {
                let mut output = Vec::with_capacity(s.len() - s.len() / 3);
                decode_base64(&mut output, s);
                Ok(Binary::from(output))
            }
            _ => Err(ParseBinaryError::InvalidEncoding)
        }
    }
}

// pub type CalAdress = Uri;

/// ICalendar Text
///
/// Text characters like comma, semicolon and backlash are automatically escaped
/// by prepending a backlash before the escaped chracters.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Text<'t>(Cow<'t, str>);

impl<'t> Text<'t> {
    /// Returns the text content.
    pub fn get(&self) -> &str {
        &self.0
    }
}

impl<'t> From<&'t str> for Text<'t> {
    fn from(value: &'t str) -> Self {
        Text(Cow::Borrowed(value))
    }
}

impl<'t> From<String> for Text<'t> {
    fn from(value: String) -> Self {
        Text(Cow::Owned(value))
    }
}

impl<'t> fmt::Display for Text<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        escape_text(f, &self.0)
    }
}

// TODO: Check for correct encoding
// impl<'t> FromStr for Text<'t> {
//     // TODO: Replace with Infallible
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Text::encode(s.to_owned()))
//     }
// }

// pub struct Uri;

// impl Uri {
//     pub fn builder() -> UriBuilder {
//         UriBuilder::new()
//     }

//     pub fn mailto() -> MailtoBuilder {
//         MailtoBuilder::new()
//     }
// }

// pub struct UriBuilder {}

// impl UriBuilder {
//     fn new() -> Self {
//         unimplemented!()
//     }

//     pub fn scheme(&mut self, scheme: &str) -> &mut Self {
//         unimplemented!()
//     }

//     pub fn authority(&mut self, authority: &str) -> &mut Self {
//         unimplemented!()
//     }

//     pub fn path(&mut self, path: &str) -> &mut Self {
//         unimplemented!()
//     }

//     pub fn query(&mut self, query: &[(&str, &str)]) -> &mut Self {
//         unimplemented!()
//     }

//     pub fn fragment(&mut self, fragment: &str) -> &mut Self {
//         unimplemented!()
//     }

//     pub fn build(&mut self) -> Result<Uri, ()> {
//         unimplemented!()
//     }
// }

// pub struct MailtoBuilder {
//     address: String,
//     headers: HashMap<String, String>
// }

// impl MailtoBuilder {
//     fn new() -> Self {
//         MailtoBuilder {
//             address: String::new(),
//             headers: HashMap::new()
//         }
//     }

//     pub fn address(&mut self, address: &str) -> &mut Self {
//         self.address = String::from(address);
//         self
//     }

//     pub fn addresses(&mut self, addresses: &[&str]) -> &mut Self {
//         self.address = addresses.join(",");
//         self
//     }

//     pub fn to(&mut self, value: &str) -> &mut Self {
//         self.header("to", value)
//     }

//     pub fn cc(&mut self, value: &str) -> &mut Self {
//         self.header("cc", value)
//     }

//     pub fn bcc(&mut self, value: &str) -> &mut Self {
//         self.header("bbc", value)
//     }

//     pub fn subject(&mut self, value: &str) -> &mut Self {
//         self.header("subject", value)
//     }

//     pub fn body(&mut self, value: &str) -> &mut Self {
//         self.header("body", value)
//     }

//     pub fn header(&mut self, name: &str, value: &str) -> &mut Self {
//         self.headers.insert(name.to_string(), value.to_string());
//         self
//     }

//     pub fn headers(&mut self, headers: &[(&str, &str)]) -> &mut Self {
//         for (name, value) in headers.into_iter() {
//             self.headers.insert(name.to_lowercase(), value.to_string());
//         }
//         self
//     }

//     pub fn build(&mut self) -> Result<Uri, ()> {
//         unimplemented!()
//     }
// }
