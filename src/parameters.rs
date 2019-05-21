//! In the RFC5545 and RFC7986 specified parameters except for IANA and
//! non-standard parameters ("X"-prefix parameters).
//!
//! Parameters are key-value pairs which can specify a property in detail. Some
//! of them also specify format definitions or defined values. Those are either
//! defined as enums or associated constants on their respective parameter.
//!
//! # Example
//! ```
//! use ics::components::Parameter;
//! use ics::parameters::CUType;
//!
//! // Using associated constants or enums should be preferred over using the
//! // generic constructors whenever possible
//! let individual = CUType::INDIVIDUAL;
//!
//! assert_eq!(CUType::new("INDIVIDUAL"), individual);
//! assert_eq!(Parameter::new("CUTYPE", "INDIVIDUAL"), individual.into());
//! ```
//! For more information on parameters, please refer to the specification [RFC5545 3.2. Property Parameters](https://tools.ietf.org/html/rfc5545#section-3.2) and [RFC7986 6. Property Parameters](https://tools.ietf.org/html/rfc7986#section-6).
use components::Parameter;
use std::borrow::Cow;

parameter!(AltRep, "ALTREP");
parameter!(CN, "CN");
parameter_with_const!(
    /// [Format definitions of calender user types.](https://tools.ietf.org/html/rfc5545#section-3.2.3)
    CUType, "CUTYPE",
    /// Default Value
    INDIVIDUAL, "INDIVIDUAL";
    GROUP, "GROUP";
    RESOURCE, "RESOURCE";
    ROOM, "ROOM";
    UNKNOWN, "UNKNOWN"
);
parameter!(DelegatedFrom, "DELEGATED-FROM");
parameter!(DelegatedTo, "DELEGATED-TO");
parameter!(Dir, "DIR");
parameter!(FmtType, "FMTTYPE");
parameter_with_const!(
    /// [Format definitions of free/busy time types](https://tools.ietf.org/html/rfc5545#section-3.2.9)
    FBType, "FBTYPE",
    FREE, "FREE";
    /// Default Value
    BUSY, "BUSY";
    BUSY_UNAVAILABLE, "BUSY-UNAVAILABLE";
    BUSY_TENTATIVE, "BUSY-TENTATIVE"
);
parameter!(Language, "LANGUAGE");
parameter!(Member, "MEMBER");
parameter_with_const!(
    /// [Format definitions of participation statuses of calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.12)
    PartStat, "PARTSTAT",
    /// `PartStat` for an Event, To-Do or Journal that needs action (Default Value)
    NEEDS_ACTION, "NEEDS-ACTION";
    /// `PartStat` for an accepted Event, To-Do or Journal
    ACCEPTED, "ACCEPTED";
    /// `PartStat` for a declined Event, To-Do or Journal
    DECLINED, "DECLINED";
    /// `PartStat` for a tentatively accepted Event or To-Do
    TENTATIVE, "TENTATIVE";
    /// `PartStat` for a delegated Event or To-Do
    DELEGATED, "DELEGATED";
    /// `PartStat` for a completed To-Do
    COMPLETED, "COMPLETED";
    /// `PartStat` for an in-process To-Do
    IN_PROCESS, "IN-PROCESS"
);
parameter_with_const!(
    /// [Format definitions of hierarchical relationship types associated with the calendar component](https://tools.ietf.org/html/rfc5545#section-3.2.15)
    RelType, "RELTYPE",
    /// Default Value
    PARENT, "PARENT";
    CHILD, "CHILD";
    SILBLING, "SILBLING"
);
parameter_with_const!(
    /// [Format definitions of participation roles for calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.16)
    Role, "ROLE",
    CHAIR, "CHAIR";
    /// Default Value
    REQ_PARTICIPANT, "REQ-PARTICIPANT";
    OPT_PARTICIPANT, "OPT-PARTICIPANT";
    NON_PARTICIPANT, "NON-PARTICIPANT"
);
parameter!(SentBy, "SENT-BY");
parameter!(TzIDParam, "TZID");
parameter_with_const!(
    /// [Format definitions of value type format for a property value](https://tools.ietf.org/html/rfc5545#section-3.2.20)
    Value, "VALUE",
    BINARY, "BINARY";
    BOOLEAN, "BOOLEAN";
    CAL_ADDRESS, "CAL-ADDRESS";
    DATE, "DATE";
    DATE_TIME, "DATE-TIME";
    DURATION, "DURATION";
    FLOAT, "FLOAT";
    INTEGER, "INTEGER";
    PERIOD, "PERIOD";
    RECUR, "RECUR";
    TEXT, "TEXT";
    TIME, "TIME";
    URI, "URI";
    UTC_OFFSET, "UTC-OFFSET"
);

impl<'a> Default for CUType<'a> {
    fn default() -> Self {
        Self::INDIVIDUAL
    }
}

impl<'a> Default for FBType<'a> {
    fn default() -> Self {
        Self::BUSY
    }
}

impl<'a> Default for PartStat<'a> {
    fn default() -> Self {
        PartStat::NEEDS_ACTION
    }
}

impl<'a> Default for RelType<'a> {
    fn default() -> Self {
        Self::PARENT
    }
}

impl<'a> Default for Role<'a> {
    fn default() -> Self {
        Self::REQ_PARTICIPANT
    }
}

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
    /// "THISANDFUTURE" (Default Value)
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
    /// "FALSE" (Default Value)
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

#[cfg(feature = "rfc7986")]
pub use self::rfc7986::*;

#[cfg(feature = "rfc7986")]
mod rfc7986 {
    use components::Parameter;
    use std::borrow::Cow;
    parameter_with_const!(
        /// [Format definitions of displaying images](https://tools.ietf.org/html/rfc7986#section-6.1)
        Display, "DISPLAY",
        /// Default Value
        BADGE, "BADGE";
        GRAPHIC, "GRAPHIC";
        FULLSIZE, "FULLSIZE";
        THUMBNAIL, "THUMBNAIL"
    );
    parameter!(Email, "EMAIL");
    parameter_with_const!(
        /// [Format definitions of features of of a conference or broadcast system](https://tools.ietf.org/html/rfc7986#section-6.3)
        Feature, "FEATURE",
        AUDIO, "AUDIO";
        CHAT, "CHAT";
        FEED, "FEED";
        MODERATOR, "MODERATOR";
        PHONE, "PHONE";
        SCREEN, "SCREEN";
        VIDEO, "VIDEO"
    );
    parameter!(Label, "LABEL");

    impl<'a> Default for Display<'a> {
        fn default() -> Self {
            Self::BADGE
        }
    }
}
