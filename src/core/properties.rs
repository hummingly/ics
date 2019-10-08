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
use components::{Parameter, Parameters, Property};
use std::borrow::Cow;
use std::collections::BTreeMap;
use value::{Binary, Resource};

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
    /// Creates a new ATTACH Property with the given values..
    pub fn uri<S>(value: S) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Self {
            value: Resource::Link(value.into()),
            parameters: BTreeMap::new()
        }
    }

    /// Creates a new ATTACH Property with the given value. The value type is
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
            value: builder.value.into_value(),
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
    latitude: f32,
    longitude: f32,
    parameters: Parameters<'a>
}

impl<'a> Geo<'a> {
    /// Creates a new GEO Property with the given values.
    pub fn new(latitude: f32, longitude: f32) -> Self {
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
    value: i32,
    parameters: Parameters<'a>
}

impl<'a> PercentComplete<'a> {
    /// Creates a new PERCENT-COMPLETE Property with the given value.
    pub fn new(value: i32) -> Self {
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
    value: i32,
    parameters: Parameters<'a>
}

impl<'a> Priority<'a> {
    /// Creates a new PRIORITY Property with the given value.
    pub fn new(value: i32) -> Self {
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

/// REPEAT Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Repeat<'a> {
    value: i32,
    parameters: Parameters<'a>
}

impl<'a> Repeat<'a> {
    /// Creates a new REPEAT Property with the given value.
    pub fn new(value: i32) -> Self {
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
property!(Created, "CREATED");
property!(DtStamp, "DTSTAMP");
property!(LastModified, "LAST-MODIFIED");

/// SEQUENCE Property
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Sequence<'a> {
    value: i32,
    parameters: Parameters<'a>
}

impl<'a> Sequence<'a> {
    /// Creates a new SEQUENCE Property with the given value.
    pub fn new(value: i32) -> Self {
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
    use components::{Parameter, Parameters, Property};
    use std::borrow::Cow;
    use std::collections::BTreeMap;
    use value::{Binary, Resource};

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
                value: builder.value.into_value(),
                parameters: builder.parameters
            }
        }
    }
}
