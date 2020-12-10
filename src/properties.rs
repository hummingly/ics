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
use crate::contentline::{ContentLine, PropertyWrite};
use crate::value::{Float, Integer};
use std::borrow::Cow;
use std::io;

property!(CalScale, "CALSCALE");
property!(Method, "METHOD");
property!(ProdID, "PRODID");
property!(Version, "VERSION");
property!(Attach, "ATTACH");
property!(Categories, "CATEGORIES");
property_with_constructor!(
    /// [Format definitions of classifications](https://tools.ietf.org/html/rfc5545#section-3.8.1.3)
    Class, "CLASS",
    // Default Value
    fn public() { "PUBLIC" };
    fn private() { "PRIVATE" };
    fn confidential() { "CONFIDENTIAL" }
);
property!(Comment, "COMMENT");
property!(Description, "DESCRIPTION");
// property!(Geo, "GEO");

/// `GEO` Property
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Geo<'a> {
    latitude: Float,
    longitude: Float,
    parameters: Vec<Parameter<'a>>
}

impl<'a> Geo<'a> {
    /// Creates a new `GEO` Property with the given value.
    pub const fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            parameters: Vec::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let parameter = parameter.into();
        match self.parameters.iter_mut().find(|p| p.key == parameter.key) {
            Some(p) => *p = parameter,
            None => self.parameters.push(parameter)
        }
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the [`parameters!`] macro.
    pub fn append(&mut self, parameters: &mut Parameters<'a>) {
        for parameter in parameters.drain(..) {
            self.add(parameter);
        }
    }
}

impl<'a> From<Geo<'a>> for Property<'a> {
    fn from(builder: Geo<'a>) -> Self {
        Property {
            key: Cow::Borrowed("GEO"),
            value: Cow::Owned(format!("{};{}", builder.latitude, builder.longitude)),
            parameters: builder.parameters
        }
    }
}

impl PropertyWrite for Geo<'_> {
    fn write<W: io::Write>(&self, line: &mut ContentLine<'_, W>) -> Result<(), io::Error> {
        line.write_name_unchecked("GEO");
        for parameter in &self.parameters {
            line.write_parameter(parameter)?;
        }
        line.write_fmt_value(format_args!("{};{}", self.latitude, self.longitude))
    }
}

property!(Location, "LOCATION");
property_integer!(PercentComplete, "PERCENT-COMPLETE");
property_integer!(Priority, "PRIORITY");
property!(Resources, "RESOURCES");
property_with_constructor!(
    /// [Format definitions of statuses](https://tools.ietf.org/html/rfc5545#section-3.8.1.11)
    Status, "STATUS",
    /// `Status` for a tentative event
    fn tentative() { "TENTATIVE" };
    /// `Status` for a definite event
    fn confirmed() { "CONFIRMED" };
    /// `Status` for a cancelled Event, To-Do or Journal
    fn cancelled() { "CANCELLED" };
    /// `Status` for a To-Do that needs action
    fn needs_action() { "NEEDS-ACTION" };
    /// `Status` for a completed To-Do
    fn completed() { "COMPLETED" };
    /// `Status` for an in-process To-Do
    fn in_process() { "IN-PROCESS" };
    /// `Status` for a draft Journal
    fn draft() { "DRAFT" };
    /// `Status` for a final Journal
    fn final_() { "FINAL" }
);
property!(Summary, "SUMMARY");
property!(Completed, "COMPLETED");
property!(DtEnd, "DTEND");
property!(Due, "DUE");
property!(DtStart, "DTSTART");
property!(Duration, "DURATION");
property!(FreeBusyTime, "FREEBUSY");
property_with_constructor!(
    /// [Format definitions of time transparency](https://tools.ietf.org/html/rfc5545#section-3.8.2.7)
    Transp, "TRANSP",
    // Default Value
    fn opaque() { "OPAQUE" };
    fn transparent() { "TRANSPARENT" }
);
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
property_with_constructor!(
    /// [Format definitions of alarm actions](https://tools.ietf.org/html/rfc5545#section-3.8.6.1)
    Action, "ACTION",
    fn audio() { "AUDIO" };
    fn display() { "DISPLAY" };
    fn email() { "EMAIL" }
);
property_integer!(Repeat, "REPEAT");
property!(Trigger, "TRIGGER");
property!(Created, "CREATED");
property!(DtStamp, "DTSTAMP");
property!(LastModified, "LAST-MODIFIED");
property_integer!(Sequence, "SEQUENCE");
property!(RequestStatus, "REQUEST-STATUS");

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
            parameters: Vec::new()
        }
    }
}
impl Default for Priority<'_> {
    fn default() -> Self {
        Self {
            value: 0,
            parameters: Vec::new()
        }
    }
}

impl Default for Repeat<'_> {
    fn default() -> Self {
        Self {
            value: 0,
            parameters: Vec::new()
        }
    }
}

impl Default for Sequence<'_> {
    fn default() -> Self {
        Self {
            value: 0,
            parameters: Vec::new()
        }
    }
}

#[cfg(feature = "rfc7986")]
pub use self::rfc7986::*;

#[cfg(feature = "rfc7986")]
mod rfc7986 {
    use crate::components::{Parameter, Parameters, Property};
    use crate::contentline::{ContentLine, PropertyWrite};
    use std::borrow::Cow;
    use std::io;
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
        parameters: Vec<Parameter<'a>>
    }

    impl<'a> Image<'a> {
        /// Creates a new `IMAGE` Property with the given value. The value type
        /// is `URI`.
        pub fn uri<S>(value: S) -> Self
        where
            S: Into<Cow<'a, str>>
        {
            Image {
                value: value.into(),
                parameters: parameters!("VALUE" => "URI")
            }
        }

        /// Creates a new `IMAGE` Property with the given value.
        /// The value type is `BINARY` which is why the `ENCODING` parameter
        /// with the value `BASE64` is also added.
        pub fn binary<S>(value: S) -> Self
        where
            S: Into<Cow<'a, str>>
        {
            Image {
                value: value.into(),
                parameters: parameters!("ENCODING" => "BASE64"; "VALUE" => "BINARY")
            }
        }

        /// Adds a parameter to the property.
        pub fn add<P>(&mut self, parameter: P)
        where
            P: Into<Parameter<'a>>
        {
            let parameter = parameter.into();
            match self.parameters.iter_mut().find(|p| p.key == parameter.key) {
                Some(p) => *p = parameter,
                None => self.parameters.push(parameter)
            }
        }

        /// Adds several parameters at once to the property. For creating
        /// several parameters at once, consult the documentation of
        /// the [`parameters!`] macro.
        pub fn append(&mut self, parameters: &mut Parameters<'a>) {
            for parameter in parameters.drain(..) {
                self.add(parameter);
            }
        }
    }

    impl_from_prop!(Image, "IMAGE");

    impl_property_write!(Image, "IMAGE");
}
