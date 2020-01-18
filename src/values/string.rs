use super::error::ParseBinaryError;
use std::borrow::Cow;
// use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use values::encoding::{encode_base64, escape_text};

// INFO: https://tools.ietf.org/html/rfc2045#section-2.8
/// ICalendar Binary
///
/// Bytes are encoded with standard Base64 encoding.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Binary(pub(crate) String);

impl Binary {
    /// Creates binary data by encoding bytes with standard Base64 encoding.
    pub fn new(bytes: &[u8]) -> Self {
        Binary(encode_base64(bytes))
    }
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// impl<'a> From<&'a str> for Binary {
//     fn from(value: &'a str) -> Self {
//         Binary::encode(value.as_bytes())
//     }
// }

// impl From<String> for Binary {
//     fn from(value: String) -> Self {
//         Binary::encode(value.as_bytes())
//     }
// }

impl FromStr for Binary {
    type Err = ParseBinaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Binary(String::new()));
        }

        // 24 bit groups are always encoded as 4 characters. Since shorter byte
        // sequences are always padded, we can assume the input has been set to a
        // multiple of 24.
        if s.len() % 4 > 0 {
            return Err(ParseBinaryError::MissingBytes);
        }

        let mut iter = s
            .bytes()
            .skip_while(|b| b.is_ascii_alphanumeric() || b == &b'+' || b == &b'/');

        match (iter.next(), iter.next(), iter.next()) {
            (None, _, _) | (Some(b'='), None, _) | (Some(b'='), Some(b'='), None) => {
                Ok(Binary(s.to_owned()))
            }
            _ => Err(ParseBinaryError::InvalidEncoding)
        }
    }
}

// pub type CalAdress = Uri;

/// ICalendar Text Value Type
///
/// Text characters like comma, semicolon and backlash are automatically escaped
/// by prepending a backlash before the escaped chracters.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Text<'t>(pub(crate) Cow<'t, str>);

impl<'t> Text<'t> {
    /// Creates new Text by prepending commas, semicolons and backlashes with a
    /// backlash.
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Cow<'t, str>>
    {
        Text(escape_text(text.into()))
    }

    /// TODO: This is an internal function! Each property that supports lists
    /// should instead have this on their API
    pub fn from_list<T>(list: Vec<T>) -> Self
    where
        T: Into<Cow<'t, str>>
    {
        let mut text = String::with_capacity(list.len());
        for t in list {
            text.push_str(&escape_text(t.into()));
            text.push(',');
        }
        let len = text.len();
        text.truncate(len);
        Text(Cow::Owned(text))
    }
}

impl<'t> fmt::Display for Text<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

// TODO: Check for correct encoding
// impl<'t> FromStr for Text<'t> {
//     // TODO: Replace with Infallible
//     type Err = ();
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(Text::encode(s.to_owned()))
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

#[cfg(test)]
mod test {
    use super::Binary;

    // https://tools.ietf.org/html/rfc4648#section-10
    #[test]
    fn parse_valid_binary() {
        assert_eq!(Some(Binary::new(b"")), "".parse().ok());
        assert_eq!(Some(Binary::new(b"f")), "Zg==".parse().ok());
        assert_eq!(Some(Binary::new(b"fo")), "Zm8=".parse().ok());
        assert_eq!(Some(Binary::new(b"foo")), "Zm9v".parse().ok());
        assert_eq!(Some(Binary::new(b"foob")), "Zm9vYg==".parse().ok());
        assert_eq!(Some(Binary::new(b"fooba")), "Zm9vYmE=".parse().ok());
        assert_eq!(Some(Binary::new(b"foobar")), "Zm9vYmFy".parse().ok());
    }

    #[test]
    fn parse_invalid_binary() {
        assert!("ABC".parse::<Binary>().is_err());
        assert!("Zö==".parse::<Binary>().is_err());
    }
}
