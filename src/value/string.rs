use super::error::ParseBinaryError;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use value::encoding::{encode_base64, escape_text};

// INFO: https://tools.ietf.org/html/rfc2045#section-2.8
/// ICalendar Binary value type
///
/// Bytes encoded with standard Base64 encoding.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Binary(String);

impl Binary {
    /// Creates binary data from bytes.
    pub fn encode(data: &[u8]) -> Self {
        Binary(encode_base64(data))
    }

    // pub fn decode(&self) -> Vec<u8> {
    //     unimplemented!()
    // }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<&'a str> for Binary {
    fn from(value: &'a str) -> Self {
        Binary::encode(value.as_bytes())
    }
}

impl From<String> for Binary {
    fn from(value: String) -> Self {
        Binary::encode(value.as_bytes())
    }
}

impl FromStr for Binary {
    type Err = ParseBinaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn is_base64_byte(b: u8) -> bool {
            b.is_ascii_alphanumeric() || b == b'+' || b == b'/'
        }

        // One encoded character equals 6 bits and the total must be a multiple
        // of 8 to make up a byte sequence
        if s.len() * 6 % 8 > 0 {
            return Err(ParseBinaryError::MissingBytes);
        }

        if s[0..s.len() - 2].bytes().all(is_base64_byte) {
            let is_valid = match &s[s.len() - 2..].as_bytes() {
                [b'=', b'='] => true,
                [b, b'='] => is_base64_byte(*b),
                slice => slice.iter().all(|&x| is_base64_byte(x))
            };
            if is_valid {
                return Ok(Binary(s.to_owned()));
            }
        }
        Err(ParseBinaryError::InvalidEncoding)
    }
}

// pub type CalAdress = Uri;

// #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
// pub struct Text<'t>(Cow<'t, str>);

// impl<'t> Text<'t> {
//     fn encode<T>(text: T) -> Self
//     where
//         T: Into<Cow<'t, str>>
//     {
//         Text(escape_text(text.into()))
//     }
// }

// impl<'t> fmt::Display for Text<'t> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }

// impl<'t> From<String> for Text<'t> {
//     fn from(value: String) -> Self {
//         Text::encode(value)
//     }
// }

// impl<'t> From<&'t str> for Text<'t> {
//     fn from(value: &'t str) -> Self {
//         Text::encode(value)
//     }
// }

// impl<'t> FromStr for Text<'t> {
//     // TODO: Replace with Infallible
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Text::new(s.to_owned()))
//     }
// }

// TODO: Decoding
// impl<'t> From<Text<'t>> for String {
//     fn from(value: Text) -> Self {
//         value.0.into_owned()
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
