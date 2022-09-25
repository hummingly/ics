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
use crate::components::Parameter;
use std::borrow::Cow;

parameter!(AltRep, "ALTREP");
parameter!(CN, "CN");
parameter!(CUType, "CUTYPE");
parameter!(DelegatedFrom, "DELEGATED-FROM");
parameter!(DelegatedTo, "DELEGATED-TO");
parameter!(Dir, "DIR");
parameter!(FmtType, "FMTTYPE");
parameter!(FBType, "FBTYPE");
parameter!(Language, "LANGUAGE");
parameter!(Member, "MEMBER");
parameter!(PartStat, "PARTSTAT");
parameter!(RelType, "RELTYPE");
parameter!(Role, "ROLE");
parameter!(SentBy, "SENT-BY");
parameter!(TzIDParam, "TZID");
parameter!(Value, "VALUE");

impl CUType<'_> {
    /// Identifies an individual (default value).
    pub const INDIVIDUAL: Self = Self {
        value: Cow::Borrowed("INDIVIDUAL"),
    };

    /// Identifies the calendar user as a group of individuals.
    pub const GROUP: Self = Self {
        value: Cow::Borrowed("GROUP"),
    };

    /// Identifies the calendar user as a physical resource.
    pub const RESOURCE: Self = Self {
        value: Cow::Borrowed("RESOURCE"),
    };

    /// Identifies the calendar user as a room resource
    pub const ROOM: Self = Self {
        value: Cow::Borrowed("ROOM"),
    };

    /// Identifies the calendar user as an unknown calendar user type.
    pub const UNKNOWN: Self = Self {
        value: Cow::Borrowed("UNKNOWN"),
    };
}

impl FBType<'_> {
    /// The time interval is free for scheduling.
    pub const FREE: Self = Self {
        value: Cow::Borrowed("FREE"),
    };

    /// The time interval is busy because one or more events have been scheduled for that interval (default value).
    pub const BUSY: Self = Self {
        value: Cow::Borrowed("BUSY"),
    };

    /// The time interval is busy and the interval cannot be scheduled.
    pub const BUSY_UNAVAILABLE: Self = Self {
        value: Cow::Borrowed("BUSY-UNAVAILABLE"),
    };

    /// The time interval is busy because one or more events have been tentatively scheduled for that interval.
    pub const BUSY_TENTATIVE: Self = Self {
        value: Cow::Borrowed("BUSY-TENTATIVE"),
    };
}

impl PartStat<'_> {
    /// Participation status for an Event, To-Do or Journal that needs action (default Value).
    pub const NEEDS_ACTION: Self = Self {
        value: Cow::Borrowed("NEEDS-ACTION"),
    };

    /// Participation status for an accepted Event, To-Do or Journal.
    pub const ACCEPTED: Self = Self {
        value: Cow::Borrowed("ACCEPTED"),
    };

    /// Participation status for a declined Event, To-Do or Journal.
    pub const DECLINED: Self = Self {
        value: Cow::Borrowed("DECLINED"),
    };

    /// Participation status for a tentatively accepted Event or To-Do.
    pub const TENTATIVE: Self = Self {
        value: Cow::Borrowed("TENTATIVE"),
    };

    /// Participation status for a delegated Event or To-Do.
    pub const DELEGATED: Self = Self {
        value: Cow::Borrowed("DELEGATED"),
    };

    /// Participation status for a completed To-Do.
    pub const COMPLETED: Self = Self {
        value: Cow::Borrowed("COMPLETED"),
    };

    /// Participation status for an in-process To-Do.
    pub const IN_PROCESS: Self = Self {
        value: Cow::Borrowed("IN-PROCESS"),
    };
}

impl RelType<'_> {
    /// Specifies a parent relationship (default value).
    pub const PARENT: Self = Self {
        value: Cow::Borrowed("PARENT"),
    };

    /// Specifies a child relationship.
    pub const CHILD: Self = Self {
        value: Cow::Borrowed("CHILD"),
    };

    /// Specifies a sibling relationship.
    pub const SIBLING: Self = Self {
        value: Cow::Borrowed("SIBLING"),
    };

    /// See [`RelType::SIBLING`].
    #[deprecated(note = "use RelType::SIBLING instead")]
    pub const SILBLING: Self = Self::SIBLING;
}

impl Role<'_> {
    /// Indicates chair of the calendar entity.
    pub const CHAIR: Self = Self {
        value: Cow::Borrowed("CHAIR"),
    };

    /// Indicates a participant whose participation is required (default value).
    pub const REQ_PARTICIPANT: Self = Self {
        value: Cow::Borrowed("REQ-PARTICIPANT"),
    };

    /// Indicates a participant whose participation is optional.
    pub const OPT_PARTICIPANT: Self = Self {
        value: Cow::Borrowed("OPT-PARTICIPANT"),
    };

    /// Indicates a participant who is copied for information purposes only.
    pub const NON_PARTICIPANT: Self = Self {
        value: Cow::Borrowed("NON-PARTICIPANT"),
    };
}

impl Value<'_> {
    /// Explicitly specifies the BINARY value type format for a property value.
    pub const BINARY: Self = Self {
        value: Cow::Borrowed("BINARY"),
    };

    /// Explicitly specifies the BOOLEAN value type format for a property value.
    pub const BOOLEAN: Self = Self {
        value: Cow::Borrowed("BOOLEAN"),
    };

    /// Explicitly specifies the CAL-ADDRESS value type format for a property value.
    pub const CAL_ADDRESS: Self = Self {
        value: Cow::Borrowed("CAL-ADDRESS"),
    };

    /// Explicitly specifies the DATE value type format for a property value.
    pub const DATE: Self = Self {
        value: Cow::Borrowed("DATE"),
    };

    /// Explicitly specifies the DATE-TIME value type format for a property value.
    pub const DATE_TIME: Self = Self {
        value: Cow::Borrowed("DATE-TIME"),
    };

    /// Explicitly specifies the DURATION value type format for a property value.
    pub const DURATION: Self = Self {
        value: Cow::Borrowed("DURATION"),
    };

    /// Explicitly specifies the FLOAT value type format for a property value.
    pub const FLOAT: Self = Self {
        value: Cow::Borrowed("FLOAT"),
    };

    /// Explicitly specifies the INTEGER value type format for a property value.
    pub const INTEGER: Self = Self {
        value: Cow::Borrowed("INTEGER"),
    };

    /// Explicitly specifies the PERIOD value type format for a property value.
    pub const PERIOD: Self = Self {
        value: Cow::Borrowed("PERIOD"),
    };

    /// Explicitly specifies the RECUR value type format for a property value.
    pub const RECUR: Self = Self {
        value: Cow::Borrowed("RECUR"),
    };

    /// Explicitly specifies the TEXT value type format for a property value.
    pub const TEXT: Self = Self {
        value: Cow::Borrowed("TEXT"),
    };

    /// Explicitly specifies the TIME value type format for a property value.
    pub const TIME: Self = Self {
        value: Cow::Borrowed("TIME"),
    };

    /// Explicitly specifies the URI value type format for a property value.
    pub const URI: Self = Self {
        value: Cow::Borrowed("URI"),
    };

    /// Explicitly specifies the UTC-OFFSET value type format for a property value.
    pub const UTC_OFFSET: Self = Self {
        value: Cow::Borrowed("UTC-OFFSET"),
    };
}

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
    Base64,
}

impl<'a> From<Encoding> for Parameter<'a> {
    fn from(builder: Encoding) -> Self {
        Parameter {
            key: "ENCODING".into(),
            value: match builder {
                Encoding::Byte => Cow::Borrowed("8BIT"),
                Encoding::Base64 => Cow::Borrowed("BASE64"),
            },
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
    ThisAndFuture,
}

impl<'a> From<Range> for Parameter<'a> {
    fn from(builder: Range) -> Self {
        Parameter {
            key: "RANGE".into(),
            value: match builder {
                Range::ThisAndFuture => Cow::Borrowed("THISANDFUTURE"),
            },
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
    End,
}

impl<'a> From<Related> for Parameter<'a> {
    fn from(builder: Related) -> Self {
        Parameter {
            key: "RELATED".into(),
            value: match builder {
                Related::Start => Cow::Borrowed("START"),
                Related::End => Cow::Borrowed("END"),
            },
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
    False,
}

impl<'a> From<RSVP> for Parameter<'a> {
    fn from(builder: RSVP) -> Self {
        Parameter {
            key: "RSVP".into(),
            value: match builder {
                RSVP::True => Cow::Borrowed("TRUE"),
                RSVP::False => Cow::Borrowed("FALSE"),
            },
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
    use crate::components::Parameter;
    use std::borrow::Cow;

    parameter!(Display, "DISPLAY");
    parameter!(Email, "EMAIL");
    parameter!(Feature, "FEATURE");
    parameter!(Label, "LABEL");

    impl Display<'_> {
        /// Displays an image inline with the title of the event (default value).
        pub const BADGE: Self = Self {
            value: Cow::Borrowed("BADGE"),
        };

        /// Displays a full image replacement for the event itself.
        pub const GRAPHIC: Self = Self {
            value: Cow::Borrowed("GRAPHIC"),
        };

        /// Displays an image that is used to enhance the event.
        pub const FULLSIZE: Self = Self {
            value: Cow::Borrowed("FULLSIZE"),
        };

        /// Displays a smaller variant of "FULLSIZE" to be used when space for the image is constrained.
        pub const THUMBNAIL: Self = Self {
            value: Cow::Borrowed("THUMBNAIL"),
        };
    }

    impl Feature<'_> {
        /// Specifies a conference or broacast system with audio capability.
        pub const AUDIO: Self = Self {
            value: Cow::Borrowed("AUDIO"),
        };

        /// Specifies a conference or broacast system with chat or instant messaging.
        pub const CHAT: Self = Self {
            value: Cow::Borrowed("CHAT"),
        };

        /// Specifies a conference or broacast system with blog or atom feed.
        pub const FEED: Self = Self {
            value: Cow::Borrowed("FEED"),
        };

        /// Specifies a conference or broacast system with moderator dial-in code.
        pub const MODERATOR: Self = Self {
            value: Cow::Borrowed("MODERATOR"),
        };

        /// Specifies a conference or broacast system with phone conference.
        pub const PHONE: Self = Self {
            value: Cow::Borrowed("PHONE"),
        };

        /// Specifies a conference or broacast system with screen sharing.
        pub const SCREEN: Self = Self {
            value: Cow::Borrowed("SCREEN"),
        };

        /// Specifies a conference or broacast system with video capability.
        pub const VIDEO: Self = Self {
            value: Cow::Borrowed("VIDEO"),
        };
    }

    impl<'a> Default for Display<'a> {
        fn default() -> Self {
            Self::BADGE
        }
    }
}
