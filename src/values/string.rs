use super::error::ParseBinaryError;
use std::borrow::Cow;
// use std::collections::HashMap;
use std::fmt;
use std::iter::FromIterator;
use std::str::FromStr;

use values::encoding::{decode_base64, encode_base64, escape_text};

// INFO: https://tools.ietf.org/html/rfc2045#section-2.8
/// ICalendar Binary
///
/// Bytes are encoded with standard Base64 encoding.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Binary<'b>(Cow<'b, [u8]>);

impl<'b> Binary<'b> {
    /// Creates binary data by encoding bytes with standard Base64 encoding.
    pub fn new<B>(bytes: B) -> Self
    where
        B: Into<Cow<'b, [u8]>>
    {
        Binary(bytes.into())
    }

    /// Returns binary data as slice of bytes.
    pub fn get(&self) -> &[u8] {
        &self.0
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
            return Ok(Binary::new([].as_ref()));
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
                Ok(Binary::new(output))
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
pub struct Text<'t>(pub(crate) Cow<'t, str>);

impl<'t> Text<'t> {
    /// Creates new Text by prepending commas, semicolons and backlashes with a
    /// backlash.
    pub fn new<T>(text: T) -> Self
    where
        T: Into<Cow<'t, str>>
    {
        // TODO: Escape in toString
        Text(escape_text(text.into()))
    }

    /// TODO: This is an internal function! Each property that supports lists
    /// should instead have this on their API
    pub fn from_list<T>(list: Vec<T>) -> Self
    where
        T: Into<Cow<'t, str>>
    {
        if list.is_empty() {
            return Text(Cow::Borrowed(""));
        }

        let mut text = String::from_iter(
            list.into_iter()
                .map(|l| [escape_text(l.into()), Cow::Borrowed(",")].concat())
        );
        text.pop();
        Text(Cow::Owned(text))
    }
}

impl<'t> fmt::Display for Text<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
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

#[cfg(test)]
mod test {
    use super::Binary;

    // https://tools.ietf.org/html/rfc4648#section-10
    #[test]
    fn parse_valid_binary() {
        assert_eq!(Some(Binary::new(b"".as_ref())), "".parse().ok());
        assert_eq!(Some(Binary::new(b"f".as_ref())), "Zg==".parse().ok());
        assert_eq!(Some(Binary::new(b"fo".as_ref())), "Zm8=".parse().ok());
        assert_eq!(Some(Binary::new(b"foo".as_ref())), "Zm9v".parse().ok());
        assert_eq!(Some(Binary::new(b"foob".as_ref())), "Zm9vYg==".parse().ok());
        assert_eq!(
            Some(Binary::new(b"fooba".as_ref())),
            "Zm9vYmE=".parse().ok()
        );
        assert_eq!(
            Some(Binary::new(b"foobar".as_ref())),
            "Zm9vYmFy".parse().ok()
        );
    }

    #[test]
    fn parse_invalid_binary() {
        assert!("ABC".parse::<Binary>().is_err());
        assert!("Zö==".parse::<Binary>().is_err());
    }
}
