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
//! For more information on properties, please refer to the specification [RFC5545 3.7. Calendar Properties](https://tools.ietf.org/html/rfc5545#section-3.7) and [RFC7986 5. Properties](https://tools.ietf.org/html/rfc7986#section-5).
use crate::contentline::{LineWriter, PropertyWrite};
use crate::parameters::{Parameter, Parameters};
use crate::value::{Float, Integer, StatusValue, TranspValue};
use std::borrow::Cow;
use std::io::Error;

property_text!(CalScale, "CALSCALE");
property_text!(Method, "METHOD");
property_text!(ProdID, "PRODID");
// TODO: min max fmt
property!(Version, "VERSION");
property!(Attach, "ATTACH");
// TODO: property_text_list!
property!(Categories, "CATEGORIES");
property_text!(
    /// [Format definitions of classifications](https://tools.ietf.org/html/rfc5545#section-3.8.1.3)
    Class, "CLASS";
    // Default Value
    const PUBLIC = "PUBLIC";
    const PRIVATE = "PRIVATE";
    const CONFIDENTIAL = "CONFIDENTIAL"
);
property_text!(Comment, "COMMENT");
property_text!(Description, "DESCRIPTION");

/// `GEO` Property
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Geo<'a> {
    latitude: Float,
    longitude: Float,
    parameters: Vec<Parameter<'a>>
}

impl Geo<'_> {
    /// The associated specification name of the property in upper case.
    pub const NAME: &'static str = "GEO";

    /// Creates a new `GEO` Property with the given value.
    pub const fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            parameters: Vec::new()
        }
    }
}

impl<'a> Geo<'a> {
    /// Adds a parameter to the property.
    pub fn add(&mut self, parameter: impl Into<Parameter<'a>>) {
        self.parameters.push(parameter.into())
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the [`parameters!`] macro.
    pub fn append(&mut self, parameters: &mut Parameters<'a>) {
        self.parameters.append(parameters)
    }
}

impl PropertyWrite for Geo<'_> {
    fn write(&self, w: &mut LineWriter<'_>) -> Result<(), Error> {
        w.write_name_unchecked(Self::NAME);
        for parameter in &self.parameters {
            w.write_parameter(&parameter.name, &parameter.value)?;
        }
        w.write_fmt_value(format_args!("{};{}", self.latitude, self.longitude))
    }
}

property_text!(Location, "LOCATION");
property_integer!(PercentComplete, "PERCENT-COMPLETE");
property_integer!(Priority, "PRIORITY");
// TODO: property_text_list!
property!(Resources, "RESOURCES");

/// `STATUS` Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Status<'a> {
    value: StatusValue,
    parameters: Parameters<'a>
}

impl Status<'_> {
    /// The associated specification name of the property in upper case.
    pub const NAME: &'static str = "STATUS";

    const fn new(value: StatusValue) -> Self {
        Self {
            value,
            parameters: Vec::new()
        }
    }

    /// Status for a tentative event
    pub const TENTATIVE: Self = Self::new(StatusValue::Tentative);

    /// Status for a definite event
    pub const CONFIRMED: Self = Self::new(StatusValue::Confirmed);

    /// Status for a cancelled Event, To-Do or Journal
    pub const CANCELLED: Self = Self::new(StatusValue::Cancelled);

    /// Status for a To-Do that needs action
    pub const NEEDS_ACTION: Self = Self::new(StatusValue::NeedsAction);

    /// Status for a completed To-Do
    pub const COMPLETED: Self = Self::new(StatusValue::Completed);

    /// Status for an in-process To-Do
    pub const IN_PROCESS: Self = Self::new(StatusValue::InProcess);

    /// Status for a draft Journal
    pub const DRAFT: Self = Self::new(StatusValue::Draft);

    /// Status for a final Journal
    pub const FINAL: Self = Self::new(StatusValue::Final);
}

impl<'a> Status<'a> {
    /// Adds a parameter to the property.
    pub fn add(&mut self, parameter: impl Into<Parameter<'a>>) {
        self.parameters.push(parameter.into())
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the [`parameters!`] macro.
    pub fn append(&mut self, parameters: &mut Parameters<'a>) {
        self.parameters.append(parameters)
    }
}

impl PropertyWrite for Status<'_> {
    fn write(&self, w: &mut LineWriter<'_>) -> Result<(), Error> {
        w.write_name_unchecked(Self::NAME);
        for parameter in &self.parameters {
            w.write_parameter(&parameter.name, &parameter.value)?;
        }
        w.write_text_value(self.value.as_str())
    }
}

property_text!(Summary, "SUMMARY");
property!(Completed, "COMPLETED");
property!(DtEnd, "DTEND");
property!(Due, "DUE");
property!(DtStart, "DTSTART");
property!(Duration, "DURATION");
property!(FreeBusyTime, "FREEBUSY");

/// `TRANSP` Property
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Transp<'a> {
    value: TranspValue,
    parameters: Parameters<'a>
}

impl Transp<'_> {
    /// The associated specification name of the property in upper case.
    pub const NAME: &'static str = "TRANSP";

    const fn new(value: TranspValue) -> Self {
        Self {
            value,
            parameters: Vec::new()
        }
    }

    /// Blocks or opaque on busy time searches.
    pub const OPAQUE: Self = Self::new(TranspValue::Opaque);

    /// Transparent on busy time searches.
    pub const TRANSPARENT: Self = Self::new(TranspValue::Transparent);
}

impl<'a> Transp<'a> {
    /// Adds a parameter to the property.
    pub fn add(&mut self, parameter: impl Into<Parameter<'a>>) {
        self.parameters.push(parameter.into())
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the [`parameters!`] macro.
    pub fn append(&mut self, parameters: &mut Parameters<'a>) {
        self.parameters.append(parameters)
    }
}

impl PropertyWrite for Transp<'_> {
    fn write(&self, w: &mut LineWriter<'_>) -> Result<(), Error> {
        w.write_name_unchecked(Self::NAME);
        for parameter in &self.parameters {
            w.write_parameter(&parameter.name, &parameter.value)?;
        }
        w.write_text_value(self.value.as_str())
    }
}

property_text!(TzID, "TZID");
property_text!(TzName, "TZNAME");
property!(TzOffsetFrom, "TZOFFSETFROM");
property!(TzOffsetTo, "TZOFFSETTO");
property!(TzURL, "TZURL");
property!(Attendee, "ATTENDEE");
property_text!(Contact, "CONTACT");
property!(Organizer, "ORGANIZER");
property!(RecurrenceID, "RECURRENCE-ID");
property_text!(RelatedTo, "RELATED-TO");
property!(URL, "URL");
property_text!(UID, "UID");
property!(ExDate, "EXDATE");
property!(RDate, "RDATE");
property!(RRule, "RRULE");
property_text!(
    /// [Format definitions of alarm actions](https://tools.ietf.org/html/rfc5545#section-3.8.6.1)
    Action, "ACTION";
    const AUDIO = "AUDIO";
    const DISPLAY = "DISPLAY";
    const EMAIL = "EMAIL"
);
property_integer!(Repeat, "REPEAT");
property!(Trigger, "TRIGGER");
property!(Created, "CREATED");
property!(DtStamp, "DTSTAMP");
property!(LastModified, "LAST-MODIFIED");
property_integer!(Sequence, "SEQUENCE");
// TODO: statcode ";" statdesc [";" extdata]
property!(RequestStatus, "REQUEST-STATUS");

impl Default for Class<'_> {
    fn default() -> Self {
        Self::PUBLIC
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
    use crate::contentline::{LineWriter, PropertyWrite};
    use crate::parameters::{Parameter, Parameters};
    use std::borrow::Cow;
    use std::io::Error;
    property_text!(Name, "NAME");
    property_with_parameter!(RefreshInterval, "REFRESH-INTERVAL", "DURATION");
    property_with_parameter!(Source, "SOURCE", "URI");
    property_text!(Color, "COLOR");
    property_with_parameter!(Conference, "CONFERENCE", "URI");

    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    enum Data<'a> {
        Link(Cow<'a, str>),
        Binary(Cow<'a, [u8]>)
    }

    /// `IMAGE` Property
    ///
    /// Newer properties that have a different value type than `TEXT` have to
    /// include the `VALUE` parameter. This property already contains the
    /// `VALUE` parameter, do not add this parameter manually. Depending on
    /// the constructor the value can be either `URI` or `BINARY`.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Image<'a> {
        value: Data<'a>,
        parameters: Vec<Parameter<'a>>
    }

    impl Image<'_> {
        /// The associated specification name of the property in upper case.
        pub const NAME: &'static str = "IMAGE";
    }

    impl<'a> Image<'a> {
        /// Creates a new `IMAGE` Property with the given value. The value type
        /// is `URI`.
        pub fn uri<S>(uri: impl Into<Cow<'a, str>>) -> Self {
            Image {
                value: Data::Link(uri.into()),
                parameters: parameters!("VALUE" => "URI")
            }
        }

        /// Creates a new `IMAGE` Property with the given value.
        /// The value type is `BINARY` which is why the `ENCODING` parameter
        /// with the value `BASE64` is also added.
        pub fn binary(binary: impl Into<Cow<'a, [u8]>>) -> Self {
            Image {
                value: Data::Binary(binary.into()),
                parameters: parameters!("ENCODING" => "BASE64"; "VALUE" => "BINARY")
            }
        }

        /// Adds a parameter to the property.
        pub fn add(&mut self, parameter: impl Into<Parameter<'a>>) {
            self.parameters.push(parameter.into())
        }

        /// Adds several parameters at once to the property. For creating
        /// several parameters at once, consult the documentation of
        /// the [`parameters!`] macro.
        pub fn append(&mut self, parameters: &mut Parameters<'a>) {
            self.parameters.append(parameters)
        }
    }

    impl PropertyWrite for Image<'_> {
        fn write(&self, w: &mut LineWriter<'_>) -> Result<(), Error> {
            w.write_name_unchecked(Self::NAME);
            for parameter in &self.parameters {
                w.write_parameter(&parameter.name, &parameter.value)?;
            }
            match &self.value {
                Data::Link(uri) => w.write_value(uri),
                Data::Binary(binary) => w.write_binary_value(binary)
            }
        }
    }
}
