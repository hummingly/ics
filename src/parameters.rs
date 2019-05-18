//! In the RFC5545 and RFC7986 specified parameters except for IANA and
//! non-standard parameters ("X"-prefix parameters).
//!
//! Parameters are key-value pairs which can specify a property in detail.
//!
//! For more information on parameters, please refer to the specification [RFC5545 3.2. Property Parameters](https://tools.ietf.org/html/rfc5545#section-3.2) and [RFC7986 6. Property Parameters](https://tools.ietf.org/html/rfc7986#section-6).
use components::Parameter;
use std::borrow::Cow;

parameter_builder!(AltRep, "ALTREP");
parameter_builder!(CN, "CN");
parameter_builder!(CUType, "CUTYPE", "INDIVIDUAL");
parameter_builder!(DelegatedFrom, "DELEGATED-FROM");
parameter_builder!(DelegatedTo, "DELEGATED-TO");
parameter_builder!(Dir, "DIR");
parameter_builder!(FmtType, "FMTTYPE");
parameter_builder!(FBType, "FBTYPE", "BUSY");
parameter_builder!(Language, "LANGUAGE");
parameter_builder!(Member, "MEMBER");
parameter_builder!(PartStat, "PARTSTAT", "NEEDS-ACTION");
parameter_builder!(RelType, "RELTYPE", "PARENT");
parameter_builder!(Role, "ROLE", "REQ-PARTICIPANT");
parameter_builder!(SentBy, "SENT-BY");
parameter_builder!(TzIDParam, "TZID");
parameter_builder!(Value, "VALUE");

def_param_consts!(
    /// [Format definitions of calender user types](https://tools.ietf.org/html/rfc5545#section-3.2.3)"
    CUType,
    INDIVIDUAL, "INDIVIDUAL";
    GROUP, "GROUP";
    RESOURCE, "RESOURCE";
    ROOM, "ROOM";
    UNKNOWN, "UNKNOWN"
);

def_param_consts!(
    /// Format definitions of free/busy time types](https://tools.ietf.org/html/rfc5545#section-3.2.9)
    FBType,
    FREE, "FREE";
    BUSY, "BUSY";
    BUSY_UNAVAILABLE, "BUSY-UNAVAILABLE";
    BUSY_TENTATIVE, "BUSY-TENTATIVE"
);

impl<'a> PartStat<'a> {
    const NEEDS_ACTION: Self = Self {
        value: Cow::Borrowed("NEEDS-ACTION")
    };

    const ACCEPTED: Self = Self {
        value: Cow::Borrowed("ACCEPTED")
    };

    const DECLINED: Self = Self {
        value: Cow::Borrowed("DECLINED")
    };

    const TENTATIVE: Self = Self {
        value: Cow::Borrowed("TENTATIVE")
    };

    const DELEGATED: Self = Self {
        value: Cow::Borrowed("DELEGATED")
    };

    const COMPLETED: Self = Self {
        value: Cow::Borrowed("COMPLETED")
    };

    const IN_PROCESS: Self = Self {
        value: Cow::Borrowed("IN-PROCESS")
    };
}

/// [Format definitions of participation statuses of calendar users for a VEVENT](https://tools.ietf.org/html/rfc5545#section-3.2.12)
pub struct PartStatEvent;

impl<'a> PartStatEvent {
    /// Returns a `PartStat` for an Event that needs action
    pub const NEEDS_ACTION: PartStat<'a> = PartStat::NEEDS_ACTION;

    /// Returns a `PartStat` for an accepted Event
    pub const ACCEPTED: PartStat<'a> = PartStat::ACCEPTED;

    /// Returns a `PartStat` for a declined Event
    pub const DECLINED: PartStat<'a> = PartStat::DECLINED;

    /// Returns a `PartStat` for a tentatively accepted Event
    pub const TENTATIVE: PartStat<'a> = PartStat::TENTATIVE;

    /// Returns a `PartStat` for a delegated Event
    pub const DELEGATED: PartStat<'a> = PartStat::DELEGATED;
}

/// [Format definitions of participation statuses of calendar users for a VTODO](https://tools.ietf.org/html/rfc5545#section-3.2.12)
pub struct PartStatToDo;

impl<'a> PartStatToDo {
    /// Returns a `PartStat` for a To-Do that needs action
    pub const NEEDS_ACTION: PartStat<'a> = PartStat::NEEDS_ACTION;

    /// Returns a `PartStat` for an accepted To-Do
    pub const ACCEPTED: PartStat<'a> = PartStat::ACCEPTED;

    /// Returns a `PartStat` for a declined To-Do
    pub const DECLINED: PartStat<'a> = PartStat::DECLINED;

    /// Returns a `PartStat` for a tentatively accepted To-Do
    pub const TENTATIVE: PartStat<'a> = PartStat::TENTATIVE;

    /// Returns a `PartStat` for a delegated To-Do
    pub const DELEGATED: PartStat<'a> = PartStat::DELEGATED;

    /// Returns a `PartStat` for a completed To-Do
    pub const COMPLETED: PartStat<'a> = PartStat::COMPLETED;

    /// Returns a `PartStat` for an in-process To-Do
    pub const IN_PROCESS: PartStat<'a> = PartStat::IN_PROCESS;
}

/// [Format definitions of participation statuses of calendar users for a VJOURNAL](https://tools.ietf.org/html/rfc5545#section-3.2.12)
pub struct PartStatJournal;

impl<'a> PartStatJournal {
    /// Returns a `PartStat` for a Journal that needs action
    pub const NEEDS_ACTION: PartStat<'a> = PartStat::NEEDS_ACTION;

    /// Returns a `PartStat` for an accepted Journal
    pub const ACCEPTED: PartStat<'a> = PartStat::ACCEPTED;

    /// Returns a `PartStat` for a declined Journal
    pub const DECLINED: PartStat<'a> = PartStat::DECLINED;
}

def_param_consts!(
    /// Format definitions of hierarchical relationship types associated with the calendar component](https://tools.ietf.org/html/rfc5545#section-3.2.15)
    RelType,
    PARENT, "PARENT";
    CHILD, "CHILD";
    SILBLING, "SILBLING"
);

def_param_consts!(
    /// Format definitions of participation roles for calendar users](https://tools.ietf.org/html/rfc5545#section-3.2.16)
    Role,
    CHAIR, "CHAIR";
    REQ_PARTICIPANT, "REQ-PARTICIPANT";
    OPT_PARTICIPANT, "OPT-PARTICIPANT";
    NON_PARTICIPANT, "NON-PARTICIPANT"
);

def_param_consts!(
    /// Format definitions of value type format for a property value](https://tools.ietf.org/html/rfc5545#section-3.2.20)
    Value,
    BINARY, "BINARY";
    BOOLEAN, "BOOLEAN";
    CAL_ADDRESS, "CAL-ADDRESS";
    DATE, "DATE";
    DATE_TIME, "DATE_TIME";
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

#[cfg(feature = "rfc7986")]
pub use self::rfc7986::*;

#[cfg(feature = "rfc7986")]
mod rfc7986 {
    use components::Parameter;
    use std::borrow::Cow;
    parameter_builder!(Display, "DISPLAY", "BADGE");
    parameter_builder!(Email, "EMAIL");
    parameter_builder!(Feature, "FEATURE");
    parameter_builder!(Label, "LABEL");

    def_param_consts!(
        /// Format definitions of displaying images](https://tools.ietf.org/html/rfc7986#section-6.1)
        Display,
        BADGE, "BADGE";
        GRAPHIC, "GRAPHIC";
        FULLSIZE, "FULLSIZE";
        THUMBNAIL, "THUMBNAIL"
    );

    def_param_consts!(
        /// Format definitions of features of of a conference or broadcast system](https://tools.ietf.org/html/rfc7986#section-6.3)
        Feature,
        AUDIO, "AUDIO";
        CHAT, "CHAT";
        FEED, "FEED";
        MODERATOR, "MODERATOR";
        PHONE, "PHONE";
        SCREEN, "SCREEN";
        VIDEO, "VIDEO"
    );
}
