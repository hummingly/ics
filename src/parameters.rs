//! In the RFC5545 and RFC7986 specified parameters except for IANA and
//! non-standard parameters ("X"-prefix parameters).
//!
//! Parameters are key-value pairs which can specify a property in detail. Some
//! of them also specify format definitions or defined values. Those are either
//! defined as enums or associated constants on their respective parameter.
//!
//! # Example
//! ```
//! use ics::parameters::{CUType, Parameter};
//!
//! // Using associated constants or enums should be preferred over using the
//! // generic constructors whenever possible
//! let individual = CUType::INDIVIDUAL;
//!
//! assert_eq!(CUType::new("INDIVIDUAL"), individual);
//! assert_eq!(Parameter::new("CUTYPE", "INDIVIDUAL"), individual.into());
//! ```
//! For more information on parameters, please refer to the specification [RFC5545 3.2. Property Parameters](https://tools.ietf.org/html/rfc5545#section-3.2) and [RFC7986 6. Property Parameters](https://tools.ietf.org/html/rfc7986#section-6).
use std::borrow::Cow;
use std::fmt;

/// A `Parameter` is a key-value that can be added to a property to specify it
/// more.
///
/// This can be used to create a new calendar parameter by either creating a
/// wrapper type or just use it as it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Parameter<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) value: Cow<'a, str>
}

impl<'a> Parameter<'a> {
    /// Creates a new property with the given key and value.
    pub fn new(name: impl Into<Cow<'a, str>>, value: impl Into<Cow<'a, str>>) -> Self {
        Parameter {
            name: name.into(),
            value: value.into()
        }
    }
}

impl fmt::Display for Parameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.name, self.value)
    }
}

/// `Parameters` is a collection of `Parameter`s. It can be created with the
/// `parameters!` macro.
pub type Parameters<'p> = Vec<Parameter<'p>>;

parameter!(AltRep, "ALTREP");
parameter!(CN, "CN");
parameter!(
    /// [Format definitions of calender user types.](https://tools.ietf.org/html/rfc5545#section-3.2.3)
    CUType, "CUTYPE";
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
parameter!(
    /// [Format definitions of free/busy time types](https://tools.ietf.org/html/rfc5545#section-3.2.9)
    FBType, "FBTYPE";
    const FREE = "FREE";
    /// Default Value
    const BUSY = "BUSY";
    const BUSY_UNAVAILABLE = "BUSY-UNAVAILABLE";
    const BUSY_TENTATIVE = "BUSY-TENTATIVE"
);
parameter!(Language, "LANGUAGE");
parameter!(Member, "MEMBER");
parameter!(
    /// [Format definitions of participation statuses of calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.12)
    PartStat, "PARTSTAT";
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
parameter!(
    /// [Format definitions of hierarchical relationship types associated with the calendar component](https://tools.ietf.org/html/rfc5545#section-3.2.15)
    RelType, "RELTYPE";
    /// Default Value
    const PARENT = "PARENT";
    const CHILD = "CHILD";
    const SILBLING = "SILBLING"
);
parameter!(
    /// [Format definitions of participation roles for calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.16)
    Role, "ROLE";
    const CHAIR = "CHAIR";
    /// Default Value
    const REQ_PARTICIPANT = "REQ-PARTICIPANT";
    const OPT_PARTICIPANT = "OPT-PARTICIPANT";
    const NON_PARTICIPANT = "NON-PARTICIPANT"
);
parameter!(SentBy, "SENT-BY");
parameter!(TzIDParam, "TZID");
parameter!(
    /// [Format definitions of value type format for a property value](https://tools.ietf.org/html/rfc5545#section-3.2.20)
    Value, "VALUE";
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

impl Default for CUType<'_> {
    fn default() -> Self {
        Self::INDIVIDUAL
    }
}

impl Default for FBType<'_> {
    fn default() -> Self {
        Self::BUSY
    }
}

impl Default for PartStat<'_> {
    fn default() -> Self {
        PartStat::NEEDS_ACTION
    }
}

impl Default for RelType<'_> {
    fn default() -> Self {
        Self::PARENT
    }
}

impl Default for Role<'_> {
    fn default() -> Self {
        Self::REQ_PARTICIPANT
    }
}

/// `ENCODING` Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Encoding {
    /// `8BIT` encoding defined in RFC2045 (Text)
    Byte,
    /// `BASE64` encoding Format defined in RFC4648 (Binary)
    Base64
}

impl From<Encoding> for Parameter<'_> {
    fn from(builder: Encoding) -> Self {
        Parameter {
            name: Cow::Borrowed("ENCODING"),
            value: Cow::Borrowed(match builder {
                Encoding::Byte => "8BIT",
                Encoding::Base64 => "BASE64"
            })
        }
    }
}

impl Default for Encoding {
    fn default() -> Self {
        Encoding::Byte
    }
}

/// `RANGE` Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Range {
    /// `THISANDFUTURE` (Default Value)
    ThisAndFuture
}

impl From<Range> for Parameter<'_> {
    fn from(_builder: Range) -> Self {
        Parameter {
            name: Cow::Borrowed("RANGE"),
            value: Cow::Borrowed("THISANDFUTURE")
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::ThisAndFuture
    }
}

/// `RELATED` Parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Related {
    /// Trigger off of `START`
    Start,
    /// Trigger off of `END`
    End
}

impl From<Related> for Parameter<'_> {
    fn from(builder: Related) -> Self {
        Parameter {
            name: Cow::Borrowed("RELATED"),
            value: Cow::Borrowed(match builder {
                Related::Start => "START",
                Related::End => "END"
            })
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
    /// `TRUE`
    True,
    /// `FALSE` (Default Value)
    False
}

impl From<RSVP> for Parameter<'_> {
    fn from(builder: RSVP) -> Self {
        Parameter {
            name: Cow::Borrowed("RSVP"),
            value: Cow::Borrowed(match builder {
                RSVP::True => "TRUE",
                RSVP::False => "FALSE"
            })
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
    use super::Parameter;
    use std::borrow::Cow;
    parameter!(
        /// [Format definitions of displaying images](https://tools.ietf.org/html/rfc7986#section-6.1)
        Display, "DISPLAY";
        /// Default Value
        const BADGE = "BADGE";
        const GRAPHIC = "GRAPHIC";
        const FULLSIZE = "FULLSIZE";
        const THUMBNAIL = "THUMBNAIL"
    );
    parameter!(Email, "EMAIL");
    parameter!(
        /// [Format definitions of features of of a conference or broadcast system](https://tools.ietf.org/html/rfc7986#section-6.3)
        Feature, "FEATURE";
        const AUDIO = "AUDIO";
        const CHAT = "CHAT";
        const FEED = "FEED";
        const MODERATOR = "MODERATOR";
        const PHONE = "PHONE";
        const SCREEN = "SCREEN";
        const VIDEO = "VIDEO"
    );
    parameter!(Label, "LABEL");

    impl Default for Display<'_> {
        fn default() -> Self {
            Self::BADGE
        }
    }
}
