//! In the RFC5545 and RFC7986 specified properties except for IANA and
//! non-standard properties ("X"-prefix parameters).
//!
//! Properties are key-value pairs which can have optionally several
//! parameters. A property forms a content line which is line folded (CRLF +
//! whitespace) after 75 bytes.
//!
//! For more information on the properties, please refer to the specification [RFC5545 3.7. Calendar Properties](https://tools.ietf.org/html/rfc5545#section-3.7).
use components::{Parameter, Parameters, Property};
use std::borrow::Cow;
use std::collections::BTreeMap;

property_builder!(CalScale, "CALSCALE");
property_builder!(Method, "METHOD");
property_builder!(ProdID, "PRODID");
property_builder!(Version, "VERSION");
property_builder!(Attach, "ATTACH");
property_builder!(Categories, "CATEGORIES");
property_builder!(Class, "CLASS");
property_builder!(Comment, "COMMENT");
property_builder!(Description, "DESCRIPTION");
property_builder!(Geo, "GEO");
property_builder!(Location, "LOCATION");
property_builder!(PercentComplete, "PERCENT-COMPLETE");
property_builder!(Priority, "PRIORITY");
property_builder!(Resources, "RESOURCES");
property_builder!(Status, "STATUS");
property_builder!(Summary, "SUMMARY");
property_builder!(Completed, "COMPLETED");
property_builder!(DtEnd, "DTEND");
property_builder!(Due, "DUE");
property_builder!(DtStart, "DTSTART");
property_builder!(Duration, "DURATION");
property_builder!(FreeBusyTime, "FREEBUSY");
property_builder!(Transp, "TRANSP");
property_builder!(TzID, "TZID");
property_builder!(TzName, "TZNAME");
property_builder!(TzOffsetFrom, "TZOFFSETFROM");
property_builder!(TzOffsetTo, "TZOFFSETTO");
property_builder!(TzURL, "TZURL");
property_builder!(Attendee, "ATTENDEE");
property_builder!(Contact, "CONTACT");
property_builder!(Organizer, "ORGANIZER");
property_builder!(RecurrenceID, "RECURRENCE-ID");
property_builder!(RelatedTo, "RELATED-TO");
property_builder!(URL, "URL");
property_builder!(UID, "UID");
property_builder!(ExDate, "EXDATE");
property_builder!(RDate, "RDATE");
property_builder!(RRule, "RRULE");
property_builder!(Action, "ACTION");
property_builder!(Repeat, "REPEAT");
property_builder!(Trigger, "TRIGGER");
property_builder!(Created, "CREATED");
property_builder!(DtStamp, "DTSTAMP");
property_builder!(LastModified, "LAST-MODIFIED");
property_builder!(Sequence, "SEQUENCE");
property_builder!(RequestStatus, "REQUEST-STATUS");
#[cfg(feature = "rfc7986")]
property_builder!(Name, "Name");
#[cfg(feature = "rfc7986")]
property_builder!(RefreshInterval, "REFRESH-INTERVAL", "DURATION");
#[cfg(feature = "rfc7986")]
property_builder!(Source, "SOURCE", "URI");
#[cfg(feature = "rfc7986")]
property_builder!(Color, "COLOR");
#[cfg(feature = "rfc7986")]
property_builder!(Conference, "CONFERENCE", "URI");

/// IMAGE Property
///
/// Newer properties that have a different value type than TEXT have to include
/// the "VALUE" parameter. This property already contains the "VALUE" parameter,
/// do not add this parameter manually. Depending on the constructor the value
/// can be either "URI" or "BINARY".
#[cfg(feature = "rfc7986")]
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Image<'a> {
    value: Cow<'a, str>,
    parameters: Parameters<'a>
}

#[cfg(feature = "rfc7986")]
impl<'a> Image<'a> {
    /// Creates a new IMAGE Property with the given value. The value type is
    /// "URI".
    pub fn uri<S>(value: S) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Image {
            value: value.into(),
            parameters: parameters!("VALUE", "URI")
        }
    }

    /// Creates a new IMAGE Property with the given value. The value type is
    /// "BINARY" which is why the "ENCODING" parameter with the value "BASE64"
    /// is also added.
    pub fn binary<S>(value: S) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Image {
            value: value.into(),
            parameters: parameters!("ENCODING", "BASE64"; "VALUE", "BINARY")
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
    pub fn append(&mut self, mut parameter: Parameters<'a>) {
        self.parameters.append(&mut parameter);
    }
}

#[cfg(feature = "rfc7986")]
impl<'a> From<Image<'a>> for Property<'a> {
    fn from(builder: Image<'a>) -> Self {
        Property {
            key: "IMAGE".into(),
            value: builder.value,
            parameters: builder.parameters
        }
    }
}

impl_default_property!(CalScale, "GREGORIAN");
impl_default_property!(Method);
impl_default_property!(ProdID);
impl_default_property!(Version);
impl_default_property!(Attach);
impl_default_property!(Categories);
impl_default_property!(Class, "PUBLIC");
impl_default_property!(Comment);
impl_default_property!(Description);
impl_default_property!(Geo);
impl_default_property!(Location);
impl_default_property!(PercentComplete);
impl_default_property!(Priority, "0");
impl_default_property!(Resources);
impl_default_property!(Status);
impl_default_property!(Summary);
impl_default_property!(Completed);
impl_default_property!(DtEnd);
impl_default_property!(Due);
impl_default_property!(DtStart);
impl_default_property!(Duration);
impl_default_property!(FreeBusyTime);
impl_default_property!(Transp, "OPAQUE");
impl_default_property!(TzID);
impl_default_property!(TzName);
impl_default_property!(TzOffsetFrom);
impl_default_property!(TzOffsetTo);
impl_default_property!(TzURL);
impl_default_property!(Attendee);
impl_default_property!(Contact);
impl_default_property!(Organizer);
impl_default_property!(RecurrenceID);
impl_default_property!(RelatedTo);
impl_default_property!(URL);
impl_default_property!(UID);
impl_default_property!(ExDate);
impl_default_property!(RDate);
impl_default_property!(RRule);
impl_default_property!(Action);
impl_default_property!(Repeat, "0");
impl_default_property!(Trigger);
impl_default_property!(Created);
impl_default_property!(DtStamp);
impl_default_property!(LastModified);
impl_default_property!(Sequence, "0");
impl_default_property!(RequestStatus);
#[cfg(feature = "rfc7986")]
impl_default_property!(Name);
#[cfg(feature = "rfc7986")]
impl_default_property!(RefreshInterval);
#[cfg(feature = "rfc7986")]
impl_default_property!(Source);
#[cfg(feature = "rfc7986")]
impl_default_property!(Color);
#[cfg(feature = "rfc7986")]
impl_default_property!(Conference);
