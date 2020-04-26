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
//! use ics::values::Text;
//!
//! // Using associated functions should be preferred over using the generic
//! // constructors whenever possible
//! let confidential = Class::confidential();
//!
//! assert_eq!(Class::new(Text::new("CONFIDENTIAL")), confidential);
//! assert_eq!(Property::new("CLASS", "CONFIDENTIAL"), confidential.into());
//! ```
//! For more information on properties, please refer to the specification [RFC5545 3.7. Calendar Properties](https://tools.ietf.org/html/rfc5545#section-3.7) and [RFC7986 5. Properties](https://tools.ietf.org/html/rfc7986#section-5).
use components::{Parameter, Parameters, Property};
use parameters::TzIDParam;
use std::borrow::Cow;
use std::collections::BTreeMap;
use values::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Resource<'a> {
    Link(Cow<'a, str>),
    Data(Binary)
}

impl<'a> From<Resource<'a>> for Cow<'a, str> {
    fn from(value: Resource<'a>) -> Self {
        match value {
            Resource::Link(uri) => uri,
            Resource::Data(binary) => Cow::Owned(binary.0)
        }
    }
}

// impl<'a> fmt::Display for Resource<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Resource::Link(uri) => write!(f, "{}", uri),
//             Resource::Data(binary) => write!(f, "{}", binary)
//         }
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum TimeStamp<T = Local> {
    Date(Date),
    DateTime(DateTime<T>)
}

impl<'a> From<TimeStamp> for Cow<'a, str> {
    fn from(value: TimeStamp) -> Self {
        Cow::Owned(match value {
            TimeStamp::Date(d) => d.to_string(),
            TimeStamp::DateTime(d) => d.to_string()
        })
    }
}

impl<'a> From<TimeStamp<Utc>> for Cow<'a, str> {
    fn from(value: TimeStamp<Utc>) -> Self {
        Cow::Owned(match value {
            TimeStamp::Date(d) => d.to_string(),
            TimeStamp::DateTime(d) => d.to_string()
        })
    }
}

property!(CalScale, "CALSCALE");
property!(Method, "METHOD");
property!(ProdID, "PRODID");
property!(Version, "VERSION");

/// ATTACH Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Attach<'a> {
    value: Resource<'a>,
    parameters: Parameters<'a>
}

impl<'a> Attach<'a> {
    /// Creates a new ATTACH Property from a URI to the attachment.
    pub fn uri<S>(value: S) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Self {
            value: Resource::Link(value.into()),
            parameters: BTreeMap::new()
        }
    }

    /// Creates a new ATTACH Property from binary content. The value type is
    /// "BINARY" which is why the "ENCODING" parameter with the value
    /// "BASE64" is also added.
    pub fn binary(value: Binary) -> Self {
        Self {
            value: Resource::Data(value),
            parameters: parameters!("ENCODING" => "BASE64"; "VALUE" => "BINARY")
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Attach<'a>> for Property<'a> {
    fn from(builder: Attach<'a>) -> Self {
        Property {
            key: "ATTACH".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

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

/// GEO Property
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Geo<'a> {
    latitude: Float,
    longitude: Float,
    parameters: Parameters<'a>
}

impl<'a> Geo<'a> {
    /// Creates a new GEO Property with the given values.
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Geo<'a>> for Property<'a> {
    fn from(builder: Geo<'a>) -> Self {
        Property {
            key: "GEO".into(),
            value: Cow::Owned(format!("{:02};{}", builder.latitude, builder.longitude)),
            parameters: builder.parameters
        }
    }
}

property!(Location, "LOCATION");

/// PERCENT-COMPLETE Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PercentComplete<'a> {
    value: Integer,
    parameters: Parameters<'a>
}

impl<'a> PercentComplete<'a> {
    /// Creates a new PERCENT-COMPLETE Property with the given value.
    pub fn new(value: Integer) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<PercentComplete<'a>> for Property<'a> {
    fn from(builder: PercentComplete<'a>) -> Self {
        Property {
            key: "PERCENT-COMPLETE".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

/// PRIORITY Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Priority<'a> {
    value: Integer,
    parameters: Parameters<'a>
}

impl<'a> Priority<'a> {
    /// Creates a new PRIORITY Property with the given value.
    pub fn new(value: Integer) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Priority<'a>> for Property<'a> {
    fn from(builder: Priority<'a>) -> Self {
        Property {
            key: "PRIORITY".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

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

/// COMPLETED Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Completed<'a> {
    value: DateTime<Utc>,
    parameters: Parameters<'a>
}

impl<'a> Completed<'a> {
    /// Creates a new COMPLETED Property from a date time with UTC time.
    pub fn new(value: DateTime<Utc>) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Completed<'a>> for Property<'a> {
    fn from(builder: Completed<'a>) -> Self {
        Property {
            key: "COMPLETED".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

/// DTEND Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DtEnd<'a, T = Local> {
    value: TimeStamp<T>,
    parameters: Parameters<'a>
}

impl<'a> DtEnd<'a> {
    /// Creates a new DTEND Property from a local date time.
    pub fn local(value: DateTime) -> Self {
        Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        }
    }

    /// Creates a new DTEND Property from a local date time with a time zone
    /// reference.
    pub fn with_tzid(value: DateTime, tzid: TzIDParam<'a>) -> Self {
        let mut end = Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        };
        end.add(tzid);
        end
    }

    /// Creates a new DTEND Property from a date. The VALUE parameter is set to
    /// DATE.
    pub fn date(value: Date) -> Self {
        Self {
            value: TimeStamp::Date(value),
            parameters: parameters!("VALUE" => "DATE")
        }
    }
}

impl<'a> DtEnd<'a, Utc> {
    /// Creates a new DTEND Property from a local date time.
    pub fn utc(value: DateTime<Utc>) -> Self {
        Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        }
    }
}

impl<'a, T> DtEnd<'a, T> {
    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<DtEnd<'a>> for Property<'a> {
    fn from(builder: DtEnd<'a>) -> Self {
        Property {
            key: "DTEND".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

impl<'a> From<DtEnd<'a, Utc>> for Property<'a> {
    fn from(builder: DtEnd<'a, Utc>) -> Self {
        Property {
            key: "DTEND".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

/// DUE Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Due<'a, T = Local> {
    value: TimeStamp<T>,
    parameters: Parameters<'a>
}

impl<'a> Due<'a> {
    /// Creates a new DUE Property from a local date time.
    pub fn local(value: DateTime) -> Self {
        Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        }
    }

    /// Creates a new DUE Property from a local date time with a time zone
    /// reference.
    pub fn with_tzid(value: DateTime, tzid: TzIDParam<'a>) -> Self {
        let mut end = Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        };
        end.add(tzid);
        end
    }

    /// Creates a new DUE Property from a date. The VALUE parameter is set to
    /// DATE.
    pub fn date(value: Date) -> Self {
        Self {
            value: TimeStamp::Date(value),
            parameters: parameters!("VALUE" => "DATE")
        }
    }
}

impl<'a> Due<'a, Utc> {
    /// Creates a new DUE Property from a local date time.
    pub fn utc(value: DateTime<Utc>) -> Self {
        Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        }
    }
}

impl<'a, T> Due<'a, T> {
    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Due<'a>> for Property<'a> {
    fn from(builder: Due<'a>) -> Self {
        Property {
            key: "DUE".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

impl<'a> From<Due<'a, Utc>> for Property<'a> {
    fn from(builder: Due<'a, Utc>) -> Self {
        Property {
            key: "DUE".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

/// DTSTART Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DtStart<'a, T = Local> {
    value: TimeStamp<T>,
    parameters: Parameters<'a>
}

impl<'a> DtStart<'a> {
    /// Creates a new DTSTART Property from a local date time.
    pub fn local(value: DateTime) -> Self {
        Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        }
    }

    /// Creates a new DTSTART Property from a local date time with a time zone
    /// reference.
    pub fn with_tzid(value: DateTime, tzid: TzIDParam<'a>) -> Self {
        let mut end = Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        };
        end.add(tzid);
        end
    }

    /// Creates a new DTSTART Property from a date. The VALUE parameter is set
    /// to DATE.
    pub fn date(value: Date) -> Self {
        Self {
            value: TimeStamp::Date(value),
            parameters: parameters!("VALUE" => "DATE")
        }
    }
}

impl<'a> DtStart<'a, Utc> {
    /// Creates a new DTSTART Property from a local date time.
    pub fn utc(value: DateTime<Utc>) -> Self {
        Self {
            value: TimeStamp::DateTime(value),
            parameters: BTreeMap::new()
        }
    }
}

impl<'a, T> DtStart<'a, T> {
    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<DtStart<'a>> for Property<'a> {
    fn from(builder: DtStart<'a>) -> Self {
        Property {
            key: "DTSTART".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

impl<'a> From<DtStart<'a, Utc>> for Property<'a> {
    fn from(builder: DtStart<'a, Utc>) -> Self {
        Property {
            key: "DTSTART".into(),
            value: builder.value.into(),
            parameters: builder.parameters
        }
    }
}

property!(Duration, "DURATION");
property!(FreeBusyTime, "FREEBUSY");

/// TRANSP Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Transp<'a> {
    value: bool,
    parameters: Parameters<'a>
}

impl<'a> Transp<'a> {
    const OPAQUE: bool = false;
    const TRANSPARENT: bool = true;

    /// Creates a new TRANSP Property set to OPAQUE.
    pub fn opaque() -> Self {
        Self {
            value: Self::OPAQUE,
            parameters: BTreeMap::new()
        }
    }

    /// Creates a new TRANSP Property set to TRANSPARENT.
    pub fn transparent() -> Self {
        Self {
            value: Self::TRANSPARENT,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Transp<'a>> for Property<'a> {
    fn from(builder: Transp<'a>) -> Self {
        Property {
            key: "TRANSP".into(),
            value: Cow::Borrowed(if builder.value == Transp::OPAQUE {
                "OPAQUE"
            } else {
                "TRANSPARENT"
            }),
            parameters: builder.parameters
        }
    }
}

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

/// REPEAT Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Repeat<'a> {
    value: Integer,
    parameters: Parameters<'a>
}

impl<'a> Repeat<'a> {
    /// Creates a new REPEAT Property with the given value.
    pub fn new(value: Integer) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Repeat<'a>> for Property<'a> {
    fn from(builder: Repeat<'a>) -> Self {
        Property {
            key: "REPEAT".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

property!(Trigger, "TRIGGER");

/// CREATED Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Created<'a> {
    value: DateTime<Utc>,
    parameters: Parameters<'a>
}

impl<'a> Created<'a> {
    /// Creates a new CREATED Property from a date time with UTC time.
    pub fn new(value: DateTime<Utc>) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Created<'a>> for Property<'a> {
    fn from(builder: Created<'a>) -> Self {
        Property {
            key: "CREATED".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

/// DTSTAMP Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DtStamp<'a> {
    value: DateTime<Utc>,
    parameters: Parameters<'a>
}

impl<'a> DtStamp<'a> {
    /// Creates a new DTSTAMP Property from a date time with UTC time.
    pub fn new(value: DateTime<Utc>) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<DtStamp<'a>> for Property<'a> {
    fn from(builder: DtStamp<'a>) -> Self {
        Property {
            key: "DTSTAMP".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

/// LAST-MODIFIED Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LastModified<'a> {
    value: DateTime<Utc>,
    parameters: Parameters<'a>
}

impl<'a> LastModified<'a> {
    /// Creates a new LAST-MODIFIED Property from a date time with UTC time.
    pub fn new(value: DateTime<Utc>) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<LastModified<'a>> for Property<'a> {
    fn from(builder: LastModified<'a>) -> Self {
        Property {
            key: "LAST-MODIFIED".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

/// SEQUENCE Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Sequence<'a> {
    value: Integer,
    parameters: Parameters<'a>
}

impl<'a> Sequence<'a> {
    /// Creates a new SEQUENCE Property with the given value.
    pub fn new(value: Integer) -> Self {
        Self {
            value,
            parameters: BTreeMap::new()
        }
    }

    /// Adds a parameter to the property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let param = parameter.into();
        self.parameters.insert(param.key, param.value);
    }

    /// Adds several parameters at once to the property. For creating
    /// several parameters at once, consult the documentation of
    /// the `parameters!` macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }
}

impl<'a> From<Sequence<'a>> for Property<'a> {
    fn from(builder: Sequence<'a>) -> Self {
        Property {
            key: "SEQUENCE".into(),
            value: Cow::Owned(builder.value.to_string()),
            parameters: builder.parameters
        }
    }
}

property!(RequestStatus, "REQUEST-STATUS");

impl<'a> Default for Priority<'a> {
    fn default() -> Self {
        Self {
            value: 0,
            parameters: BTreeMap::new()
        }
    }
}

impl<'a> Default for Repeat<'a> {
    fn default() -> Self {
        Self {
            value: 0,
            parameters: BTreeMap::new()
        }
    }
}

impl<'a> Default for Sequence<'a> {
    fn default() -> Self {
        Self {
            value: 0,
            parameters: BTreeMap::new()
        }
    }
}

impl<'a> Default for Class<'a> {
    fn default() -> Self {
        Self::public()
    }
}

impl<'a> Default for Transp<'a> {
    fn default() -> Self {
        Self::opaque()
    }
}

impl_default_prop!(CalScale, "GREGORIAN");

#[cfg(feature = "rfc7986")]
pub use self::rfc7986::*;

#[cfg(feature = "rfc7986")]
mod rfc7986 {
    use super::Resource;
    use components::{Parameter, Parameters, Property};
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use values::{Binary, Text};

    property!(Name, "NAME");
    property_with_parameter!(RefreshInterval, "REFRESH-INTERVAL", "DURATION");
    property_with_parameter!(Source, "SOURCE", "URI");
    property!(Color, "COLOR");
    property_with_parameter!(Conference, "CONFERENCE", "URI");

    /// IMAGE Property
    ///
    /// Newer properties that have a different value type than TEXT have to
    /// include the "VALUE" parameter. This property already contains the
    /// "VALUE" parameter, do not add this parameter manually. Depending on
    /// the constructor the value can be either "URI" or "BINARY".
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Image<'a> {
        value: Resource<'a>,
        parameters: Parameters<'a>
    }

    impl<'a> Image<'a> {
        /// Creates a new IMAGE Property with the given value. The value type is
        /// "URI".
        pub fn uri<S>(value: S) -> Self
        where
            S: Into<Cow<'a, str>>
        {
            Image {
                value: Resource::Link(value.into()),
                parameters: parameters!("VALUE" => "URI")
            }
        }

        /// Creates a new IMAGE Property with the given value. The value type is
        /// "BINARY" which is why the "ENCODING" parameter with the value
        /// "BASE64" is also added.
        pub fn binary(value: Binary) -> Self {
            Image {
                value: Resource::Data(value),
                parameters: parameters!("ENCODING" => "BASE64"; "VALUE" => "BINARY")
            }
        }

        /// Adds a parameter to the property.
        pub fn add<P>(&mut self, parameter: P)
        where
            P: Into<Parameter<'a>>
        {
            let param = parameter.into();
            self.parameters.insert(param.key, param.value);
        }

        /// Adds several parameters at once to the property. For creating
        /// several parameters at once, consult the documentation of
        /// the `parameters!` macro.
        pub fn append(&mut self, mut parameters: Parameters<'a>) {
            self.parameters.append(&mut parameters);
        }
    }

    impl<'a> From<Image<'a>> for Property<'a> {
        fn from(builder: Image<'a>) -> Self {
            Property {
                key: "IMAGE".into(),
                value: builder.value.into(),
                parameters: builder.parameters
            }
        }
    }
}
