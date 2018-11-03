//! In the RFC5545 and RFC7986 specified parameters except for IANA and
//! non-standard parameters ("X"-prefix parameters).
//!
//! Parameters are key-value pairs which can specify a property in detail.
//!
//! For more information on the parameters, please refer to the specification [RFC5545 3.2. Property Parameters](https://tools.ietf.org/html/rfc5545#section-3.2).
use components::Parameter;
use std::borrow::Cow;

parameter_builder!(AltRep, "ALTREP");
parameter_builder!(CN, "CN");
parameter_builder!(CUType, "CUTYPE");
parameter_builder!(DelegatedFrom, "DELEGATED-FROM");
parameter_builder!(DelegatedTo, "DELEGATED-TO");
parameter_builder!(Dir, "DIR");
parameter_builder!(FmtType, "FMTTYPE");
parameter_builder!(FBType, "FBTYPE");
parameter_builder!(Language, "LANGUAGE");
parameter_builder!(Member, "MEMBER");
parameter_builder!(PartStat, "PARTSTAT");
parameter_builder!(RelType, "RELTYPE");
parameter_builder!(Role, "ROLE");
parameter_builder!(SentBy, "SENT-BY");
parameter_builder!(TzIDParam, "TZID");
parameter_builder!(Value, "VALUE");
#[cfg(feature = "rfc7986")]
parameter_builder!(Display, "DISPLAY");
#[cfg(feature = "rfc7986")]
parameter_builder!(Email, "EMAIL");
#[cfg(feature = "rfc7986")]
parameter_builder!(Feature, "FEATURE");
#[cfg(feature = "rfc7986")]
parameter_builder!(Label, "LABEL");

impl_default_parameter!(AltRep);
impl_default_parameter!(CN);
impl_default_parameter!(CUType, "INDIVIDUAL");
impl_default_parameter!(DelegatedFrom);
impl_default_parameter!(DelegatedTo);
impl_default_parameter!(Dir);
impl_default_parameter!(FmtType);
impl_default_parameter!(FBType, "BUSY");
impl_default_parameter!(Language);
impl_default_parameter!(Member);
impl_default_parameter!(PartStat, "NEEDS-ACTION");
impl_default_parameter!(RelType, "PARENT");
impl_default_parameter!(Role, "REQ-PARTICIPANT");
impl_default_parameter!(SentBy);
impl_default_parameter!(TzIDParam);
impl_default_parameter!(Value);
#[cfg(feature = "rfc7986")]
impl_default_parameter!(Display, "BADGE");
#[cfg(feature = "rfc7986")]
impl_default_parameter!(Email);
#[cfg(feature = "rfc7986")]
impl_default_parameter!(Feature);
#[cfg(feature = "rfc7986")]
impl_default_parameter!(Label);

/// ENCODING Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Encoding {
    /// Text Encoding defined in RFC2045
    Bit8,
    /// Binary Encoding Format defined in RFC4648
    Base64
}

impl Encoding {
    fn into_value<'a>(self) -> Cow<'a, str> {
        match self {
            Encoding::Bit8 => Cow::Borrowed("8BIT"),
            Encoding::Base64 => Cow::Borrowed("BASE64")
        }
    }
}

impl<'a> From<Encoding> for Parameter<'a> {
    fn from(builder: Encoding) -> Self {
        Parameter {
            key: "ENCODING".into(),
            value: builder.into_value()
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Bit8
    }
}

/// RANGE Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Range {
    /// "THISANDFUTURE" (default value)
    ThisAndFuture
}

impl Range {
    fn into_value<'a>(self) -> Cow<'a, str> {
        Cow::Borrowed("THISANDFUTURE")
    }
}

impl<'a> From<Range> for Parameter<'a> {
    fn from(builder: Range) -> Self {
        Parameter {
            key: "RANGE".into(),
            value: builder.into_value()
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::ThisAndFuture
    }
}

/// RELATED Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Related {
    /// Trigger off of start
    Start,
    /// Trigger off of end
    End
}

impl Related {
    fn into_value<'a>(self) -> Cow<'a, str> {
        match self {
            Related::Start => Cow::Borrowed("START"),
            Related::End => Cow::Borrowed("END")
        }
    }
}

impl<'a> From<Related> for Parameter<'a> {
    fn from(builder: Related) -> Self {
        Parameter {
            key: "RELATED".into(),
            value: builder.into_value()
        }
    }
}

impl Default for Related {
    fn default() -> Self {
        Related::Start
    }
}

/// RSVP Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RSVP {
    /// "TRUE"
    True,
    /// "FALSE" (default value)
    False
}

impl RSVP {
    fn into_value<'a>(self) -> Cow<'a, str> {
        match self {
            RSVP::True => Cow::Borrowed("TRUE"),
            RSVP::False => Cow::Borrowed("FALSE")
        }
    }
}

impl<'a> From<RSVP> for Parameter<'a> {
    fn from(builder: RSVP) -> Self {
        Parameter {
            key: "RSVP".into(),
            value: builder.into_value()
        }
    }
}

impl Default for RSVP {
    fn default() -> Self {
        RSVP::False
    }
}
