//! In the RFC 5545 specified parameters except for IANA and non-standard
//! parameters ("X"-prefix parameters).
//!
//! Parameters are key-value pairs which can specify a property in detail.
//!
//! For more information on the parameters, please refer to the specification [RFC 5545 3.2. Property Parameters](https://tools.ietf.org/html/rfc5545#section-3.2).
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

/// ENCODING Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Encoding {
    /// Text Encoding defined in RFC 2045
    Bit8,
    /// Binary Encoding Format defined in RFC 4648
    Base64
}

impl<'a> From<Encoding> for Cow<'a, str> {
    fn from(builder: Encoding) -> Self {
        match builder {
            Encoding::Bit8 => Cow::Borrowed("8BIT"),
            Encoding::Base64 => Cow::Borrowed("BASE64")
        }
    }
}

impl<'a> From<Encoding> for Parameter<'a> {
    fn from(builder: Encoding) -> Self {
        Parameter {
            key: "ENCODING".into(),
            value: builder.into()
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

impl<'a> From<Range> for Cow<'a, str> {
    fn from(builder: Range) -> Self {
        match builder {
            Range::ThisAndFuture => Cow::Borrowed("THISANDFUTURE")
        }
    }
}

impl<'a> From<Range> for Parameter<'a> {
    fn from(builder: Range) -> Self {
        Parameter {
            key: "RANGE".into(),
            value: builder.into()
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

impl<'a> From<Related> for Cow<'a, str> {
    fn from(builder: Related) -> Self {
        match builder {
            Related::Start => Cow::Borrowed("START"),
            Related::End => Cow::Borrowed("END")
        }
    }
}

impl<'a> From<Related> for Parameter<'a> {
    fn from(builder: Related) -> Self {
        Parameter {
            key: "RELATED".into(),
            value: builder.into()
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

impl<'a> From<RSVP> for Cow<'a, str> {
    fn from(builder: RSVP) -> Self {
        match builder {
            RSVP::True => Cow::Borrowed("TRUE"),
            RSVP::False => Cow::Borrowed("FALSE")
        }
    }
}

impl<'a> From<RSVP> for Parameter<'a> {
    fn from(builder: RSVP) -> Self {
        Parameter {
            key: "RSVP".into(),
            value: builder.into()
        }
    }
}

impl Default for RSVP {
    fn default() -> Self {
        RSVP::False
    }
}
