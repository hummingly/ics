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
use crate::contentline::{ContentLine, PropertyWrite};
use crate::parameters::{Parameter, Parameters};
use crate::value::{Float, Integer, StatusValue, TranspValue};
use std::borrow::Cow;
use std::io;

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
    fn public() { "PUBLIC" };
    fn private() { "PRIVATE" };
    fn confidential() { "CONFIDENTIAL" }
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
        match self
            .parameters
            .iter_mut()
            .find(|p| p.name == parameter.name)
        {
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

impl PropertyWrite for Geo<'_> {
    fn write<W: io::Write>(&self, line: &mut ContentLine<W>) -> Result<(), io::Error> {
        line.write_name_unchecked("GEO");
        for parameter in &self.parameters {
            line.write_parameter(parameter)?;
        }
        line.write_fmt_value(format_args!("{};{}", self.latitude, self.longitude))
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

impl<'a> Status<'a> {
    /// Creates a new `STATUS` Property with the given value.
    pub const fn new(value: StatusValue) -> Self {
        Self {
            value,
            parameters: Vec::new()
        }
    }

    /// Status for a tentative event
    pub const fn tentative() -> Self {
        Self::new(StatusValue::Tentative)
    }

    /// Status for a definite event
    pub const fn confirmed() -> Self {
        Self::new(StatusValue::Confirmed)
    }

    /// Status for a cancelled Event, To-Do or Journal
    pub const fn cancelled() -> Self {
        Self::new(StatusValue::Cancelled)
    }

    /// Status for a To-Do that needs action
    pub const fn needs_action() -> Self {
        Self::new(StatusValue::NeedsAction)
    }

    /// Status for a completed To-Do
    pub const fn completed() -> Self {
        Self::new(StatusValue::Completed)
    }

    /// Status for an in-process To-Do
    pub const fn in_process() -> Self {
        Self::new(StatusValue::InProcess)
    }

    /// Status for a draft Journal
    pub const fn draft() -> Self {
        Self::new(StatusValue::Draft)
    }

    /// Status for a final Journal
    pub const fn final_() -> Self {
        Self::new(StatusValue::Final)
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let parameter = parameter.into();
        match self
            .parameters
            .iter_mut()
            .find(|p| p.name == parameter.name)
        {
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

impl PropertyWrite for Status<'_> {
    fn write<W: io::Write>(&self, line: &mut ContentLine<W>) -> Result<(), io::Error> {
        line.write_name_unchecked("STATUS");
        for parameter in &self.parameters {
            line.write_parameter(parameter)?;
        }
        line.write_value_text(self.value.as_str())
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

impl<'a> Transp<'a> {
    /// Creates a new `STATUS` Property with the given value.
    pub const fn new(value: TranspValue) -> Self {
        Self {
            value,
            parameters: Vec::new()
        }
    }

    /// Blocks or opaque on busy time searches.
    pub const fn opaque() -> Self {
        Self::new(TranspValue::Opaque)
    }

    /// Transparent on busy time searches.
    pub const fn transparent() -> Self {
        Self::new(TranspValue::Transparent)
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let parameter = parameter.into();
        match self
            .parameters
            .iter_mut()
            .find(|p| p.name == parameter.name)
        {
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

impl PropertyWrite for Transp<'_> {
    fn write<W: io::Write>(&self, line: &mut ContentLine<W>) -> Result<(), io::Error> {
        line.write_name_unchecked("TRANSP");
        for parameter in &self.parameters {
            line.write_parameter(parameter)?;
        }
        line.write_value_text(self.value.as_str())
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
// TODO: statcode ";" statdesc [";" extdata]
property!(RequestStatus, "REQUEST-STATUS");

impl Default for Class<'_> {
    fn default() -> Self {
        Self::public()
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
    use crate::contentline::{ContentLine, PropertyWrite};
    use crate::parameters::{Parameter, Parameters};
    use std::borrow::Cow;
    use std::io;
    property_text!(Name, "NAME");
    property_with_parameter!(RefreshInterval, "REFRESH-INTERVAL", "DURATION");
    property_with_parameter!(Source, "SOURCE", "URI");
    property_text!(Color, "COLOR");
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
            match self
                .parameters
                .iter_mut()
                .find(|p| p.name == parameter.name)
            {
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

    impl_property_write!(Image, "IMAGE");
}
