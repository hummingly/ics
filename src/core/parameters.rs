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
    const INDIVIDUAL = "INDIVIDUAL";
    const GROUP = "GROUP";
    const RESOURCE = "RESOURCE";
    const ROOM = "ROOM";
    const UNKNOWN = "UNKNOWN"
);
parameter!(DelegatedFrom, "DELEGATED-FROM");
parameter!(DelegatedTo, "DELEGATED-TO");
parameter!(Dir, "DIR");
parameter!(FmtType, "FMTTYPE");
parameter_with_const!(
    /// [Format definitions of free/busy time types](https://tools.ietf.org/html/rfc5545#section-3.2.9)
    FBType, "FBTYPE",
    const FREE = "FREE";
    /// Default Value
    const BUSY = "BUSY";
    const BUSY_UNAVAILABLE = "BUSY-UNAVAILABLE";
    const BUSY_TENTATIVE = "BUSY-TENTATIVE"
);
parameter!(Language, "LANGUAGE");
parameter!(Member, "MEMBER");
parameter_with_const!(
    /// [Format definitions of participation statuses of calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.12)
    PartStat, "PARTSTAT",
    /// `PartStat` for an Event, To-Do or Journal that needs action (Default Value)
    const NEEDS_ACTION = "NEEDS-ACTION";
    /// `PartStat` for an accepted Event, To-Do or Journal
    const ACCEPTED = "ACCEPTED";
    /// `PartStat` for a declined Event, To-Do or Journal
    const DECLINED = "DECLINED";
    /// `PartStat` for a tentatively accepted Event or To-Do
    const TENTATIVE = "TENTATIVE";
    /// `PartStat` for a delegated Event or To-Do
    const DELEGATED = "DELEGATED";
    /// `PartStat` for a completed To-Do
    const COMPLETED = "COMPLETED";
    /// `PartStat` for an in-process To-Do
    const IN_PROCESS = "IN-PROCESS"
);
parameter_with_const!(
    /// [Format definitions of hierarchical relationship types associated with the calendar component](https://tools.ietf.org/html/rfc5545#section-3.2.15)
    RelType, "RELTYPE",
    /// Default Value
    const PARENT = "PARENT";
    const CHILD = "CHILD";
    const SILBLING = "SILBLING"
);
parameter_with_const!(
    /// [Format definitions of participation roles for calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.16)
    Role, "ROLE",
    const CHAIR = "CHAIR";
    /// Default Value
    const REQ_PARTICIPANT = "REQ-PARTICIPANT";
    const OPT_PARTICIPANT = "OPT-PARTICIPANT";
    const NON_PARTICIPANT = "NON-PARTICIPANT"
);
parameter!(SentBy, "SENT-BY");
parameter!(TzIDParam, "TZID");
parameter_with_const!(
    /// [Format definitions of value type format for a property value](https://tools.ietf.org/html/rfc5545#section-3.2.20)
    Value, "VALUE",
    const BINARY = "BINARY";
    const BOOLEAN = "BOOLEAN";
    const CAL_ADDRESS = "CAL-ADDRESS";
    const DATE = "DATE";
    const DATE_TIME = "DATE-TIME";
    const DURATION = "DURATION";
    const FLOAT = "FLOAT";
    const INTEGER = "INTEGER";
    const PERIOD = "PERIOD";
    const RECUR = "RECUR";
    const TEXT = "TEXT";
    const TIME = "TIME";
    const URI = "URI";
    const UTC_OFFSET = "UTC-OFFSET"
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
    Byte,
    /// Binary Encoding Format defined in RFC4648
    Base64
}

impl Encoding {
    fn into_value<'a>(self) -> Cow<'a, str> {
        match self {
            Encoding::Byte => Cow::Borrowed("8BIT"),
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
        Encoding::Byte
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
        const BADGE = "BADGE";
        const GRAPHIC = "GRAPHIC";
        const FULLSIZE = "FULLSIZE";
        const THUMBNAIL = "THUMBNAIL"
    );
    parameter!(Email, "EMAIL");
    parameter_with_const!(
        /// [Format definitions of features of of a conference or broadcast system](https://tools.ietf.org/html/rfc7986#section-6.3)
        Feature, "FEATURE",
        const AUDIO = "AUDIO";
        const CHAT = "CHAT";
        const FEED = "FEED";
        const MODERATOR = "MODERATOR";
        const PHONE = "PHONE";
        const SCREEN = "SCREEN";
        const VIDEO = "VIDEO"
    );
    parameter!(Label, "LABEL");

    impl<'a> Default for Display<'a> {
        fn default() -> Self {
            Self::BADGE
        }
    }
}
