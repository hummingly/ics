//! In the RFC5545 and RFC7986 specified properties except for IANA and
//! non-standard properties ("X"-prefix parameters).
//!
//! Properties are key-value pairs which can have optionally several
//! parameters. A property forms a content line which is line folded (CRLF +
//! whitespace) after 75 bytes automatically for you.
//!
//! Additionally, some of them also specify format definitions or defined
//! values. Those are associated functions or constructors.
//!
//! # Example
//! ```
//! use ics::components::Property;
//! use ics::properties::Class;
//!
//! // Using associated functions should be preferred over using the generic
//! // constructors whenever possible
//! let confidential = Class::confidential();
//!
//! assert_eq!(Class::new("CONFIDENTIAL"), confidential);
//! assert_eq!(Property::new("CLASS", "CONFIDENTIAL"), confidential.into());
//! ```
//! For more information on properties, please refer to the specification [RFC5545 3.7. Calendar Properties](https://tools.ietf.org/html/rfc5545#section-3.7) and [RFC7986 5. Properties](https://tools.ietf.org/html/rfc7986#section-5).
use crate::components::{Parameter, Parameters, Property};
use std::borrow::Cow;
use std::collections::BTreeMap;

property!(CalScale, "CALSCALE");
property!(Method, "METHOD");
property!(ProdID, "PRODID");
property!(Version, "VERSION");
property!(Attach, "ATTACH");
property!(Categories, "CATEGORIES");
property!(Class, "CLASS");
property!(Comment, "COMMENT");
property!(Description, "DESCRIPTION");
property!(Geo, "GEO");
property!(Location, "LOCATION");
property!(PercentComplete, "PERCENT-COMPLETE");
property!(Priority, "PRIORITY");
property!(Resources, "RESOURCES");
property!(Status, "STATUS");
property!(Summary, "SUMMARY");
property!(Completed, "COMPLETED");
property!(DtEnd, "DTEND");
property!(Due, "DUE");
property!(DtStart, "DTSTART");
property!(Duration, "DURATION");
property!(FreeBusyTime, "FREEBUSY");
property!(Transp, "TRANSP");
property!(TzID, "TZID");
property!(TzName, "TZNAME");
property!(TzOffsetFrom, "TZOFFSETFROM");
property!(TzOffsetTo, "TZOFFSETTO");
property!(TzURL, "TZURL");
property!(Attendee, "ATTENDEE");
property!(Contact, "CONTACT");
property!(Organizer, "ORGANIZER");
property!(RecurrenceID, "RECURRENCE-ID");
property!(RelatedTo, "RELATED-TO");
property!(URL, "URL");
property!(UID, "UID");
property!(ExDate, "EXDATE");
property!(RDate, "RDATE");
property!(RRule, "RRULE");
property!(Action, "ACTION");
property!(Repeat, "REPEAT");
property!(Trigger, "TRIGGER");
property!(Created, "CREATED");
property!(DtStamp, "DTSTAMP");
property!(LastModified, "LAST-MODIFIED");
property!(Sequence, "SEQUENCE");
property!(RequestStatus, "REQUEST-STATUS");

impl Class<'_> {
    /// Specifies the access classification as public for a component (default value).
    pub fn public() -> Self {
        Self::new("PUBLIC")
    }

    /// Specifies the access classification as private for a component.
    pub fn private() -> Self {
        Self::new("PRIVATE")
    }

    /// Specifies the access classification as confidential for a component.
    pub fn confidential() -> Self {
        Self::new("CONFIDENTIAL")
    }
}

impl Status<'_> {
    /// Status for a tentative event.
    pub fn tentative() -> Self {
        Self::new("TENTATIVE")
    }

    /// Status for a definite event.
    pub fn confirmed() -> Self {
        Self::new("CONFIRMED")
    }

    /// Status for a cancelled Event, To-Do or Journal.
    pub fn cancelled() -> Self {
        Self::new("CANCELLED")
    }

    /// Status for a To-Do that needs action.
    pub fn needs_action() -> Self {
        Self::new("NEEDS-ACTION")
    }

    /// Status for a completed To-Do.
    pub fn completed() -> Self {
        Self::new("COMPLETED")
    }

    /// Status for an in-process To-Do.
    pub fn in_process() -> Self {
        Self::new("IN-PROCESS")
    }

    /// Status for a draft Journal.
    pub fn draft() -> Self {
        Self::new("DRAFT")
    }

    /// Status for a final Journal.
    pub fn final_() -> Self {
        Self::new("FINAL")
    }
}

impl Transp<'_> {
    /// Blocks or opaque on busy time searches (default value).
    pub fn opaque() -> Self {
        Self::new("OPAQUE")
    }

    /// Transparent on busy time searches.
    pub fn transparent() -> Self {
        Self::new("TRANSPARENT")
    }
}

impl Action<'_> {
    /// Specifies an audio action to be invoked when an alarm is triggered.
    pub fn audio() -> Self {
        Self::new("AUDIO")
    }
    /// Specifies a display action to be invoked when an alarm is triggered.
    pub fn display() -> Self {
        Self::new("DISPLAY")
    }
    /// Specifies an email action to be invoked when an alarm is triggered.
    pub fn email() -> Self {
        Self::new("EMAIL")
    }
}

impl Default for Class<'_> {
    fn default() -> Self {
        Self::public()
    }
}

impl Default for Transp<'_> {
    fn default() -> Self {
        Self::opaque()
    }
}

impl Default for CalScale<'_> {
    fn default() -> Self {
        Self {
            value: Cow::Borrowed("GREGORIAN"),
            parameters: BTreeMap::new(),
        }
    }
}

impl Default for Priority<'_> {
    fn default() -> Self {
        Self {
            value: Cow::Borrowed("0"),
            parameters: BTreeMap::new(),
        }
    }
}

impl Default for Repeat<'_> {
    fn default() -> Self {
        Self {
            value: Cow::Borrowed("0"),
            parameters: BTreeMap::new(),
        }
    }
}

impl Default for Sequence<'_> {
    fn default() -> Self {
        Self {
            value: Cow::Borrowed("0"),
            parameters: BTreeMap::new(),
        }
    }
}

#[cfg(feature = "rfc7986")]
pub use self::rfc7986::*;

#[cfg(feature = "rfc7986")]
mod rfc7986 {
    use crate::components::{Parameter, Parameters, Property};
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    property!(Name, "NAME");
    property_with_parameter!(RefreshInterval, "REFRESH-INTERVAL", "DURATION");
    property_with_parameter!(Source, "SOURCE", "URI");
    property!(Color, "COLOR");
    property_with_parameter!(Conference, "CONFERENCE", "URI");

    /// `IMAGE` Property
    ///
    /// Newer properties that have a different value type than `TEXT` have to
    /// include the `VALUE` parameter. This property already contains the
    /// `VALUE` parameter, do not add this parameter manually. Depending on
    /// the constructor the value can be either `URI` or `BINARY`.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Image<'a> {
        value: Cow<'a, str>,
        parameters: Parameters<'a>,
    }

    impl<'a> Image<'a> {
        /// Creates a new `IMAGE` Property with the given value. The value type
        /// is `URI`.
        pub fn uri<S>(value: S) -> Self
        where
            S: Into<Cow<'a, str>>,
        {
            Image {
                value: value.into(),
                parameters: parameters!("VALUE" => "URI"),
            }
        }

        /// Creates a new `IMAGE` Property with the given value.
        /// The value type is `BINARY` which is why the `ENCODING` parameter
        /// with the value `BASE64` is also added.
        pub fn binary<S>(value: S) -> Self
        where
            S: Into<Cow<'a, str>>,
        {
            Image {
                value: value.into(),
                parameters: parameters!("ENCODING" => "BASE64"; "VALUE" => "BINARY"),
            }
        }
    }

    impl_add_parameters!(Image);

    impl_from_prop!(Image, "IMAGE");
}
