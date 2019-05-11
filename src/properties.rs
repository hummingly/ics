//! In the RFC5545 and RFC7986 specified properties except for IANA and
//! non-standard properties ("X"-prefix parameters).
//!
//! Properties are key-value pairs which can have optionally several
//! parameters. A property forms a content line which is line folded (CRLF +
//! whitespace) after 75 bytes.
//!
//! For more information on properties, please refer to the specification [RFC5545 3.7. Calendar Properties](https://tools.ietf.org/html/rfc5545#section-3.7) and [RFC7986 5. Properties](https://tools.ietf.org/html/rfc7986#section-5).
use components::{Parameter, Parameters, Property};
use std::borrow::Cow;
use std::collections::BTreeMap;

property_builder!(CalScale, "CALSCALE", "GREGORIAN");
property_builder!(Method, "METHOD");
property_builder!(ProdID, "PRODID");
property_builder!(Version, "VERSION");
property_builder!(Attach, "ATTACH");
property_builder!(Categories, "CATEGORIES");
property_builder!(Class, "CLASS", "PUBLIC");
property_builder!(Comment, "COMMENT");
property_builder!(Description, "DESCRIPTION");
property_builder!(Geo, "GEO");
property_builder!(Location, "LOCATION");
property_builder!(PercentComplete, "PERCENT-COMPLETE");
property_builder!(Priority, "PRIORITY", "0");
property_builder!(Resources, "RESOURCES");
property_builder!(Status, "STATUS");
property_builder!(Summary, "SUMMARY");
property_builder!(Completed, "COMPLETED");
property_builder!(DtEnd, "DTEND");
property_builder!(Due, "DUE");
property_builder!(DtStart, "DTSTART");
property_builder!(Duration, "DURATION");
property_builder!(FreeBusyTime, "FREEBUSY");
property_builder!(Transp, "TRANSP", "OPAQUE");
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
property_builder!(Repeat, "REPEAT", "0");
property_builder!(Trigger, "TRIGGER");
property_builder!(Created, "CREATED");
property_builder!(DtStamp, "DTSTAMP");
property_builder!(LastModified, "LAST-MODIFIED");
property_builder!(Sequence, "SEQUENCE", "0");
property_builder!(RequestStatus, "REQUEST-STATUS");

#[cfg(feature = "rfc7986")]
pub use self::rfc7986::*;

#[cfg(feature = "rfc7986")]
mod rfc7986 {
    use components::{Parameter, Parameters, Property};
    use std::borrow::Cow;
    property_builder!(Name, "NAME");
    property_builder_with_value_param!(RefreshInterval, "REFRESH-INTERVAL", "DURATION");
    property_builder_with_value_param!(Source, "SOURCE", "URI");
    property_builder!(Color, "COLOR");
    property_builder_with_value_param!(Conference, "CONFERENCE", "URI");

    /// IMAGE Property
    ///
    /// Newer properties that have a different value type than TEXT have to
    /// include the "VALUE" parameter. This property already contains the
    /// "VALUE" parameter, do not add this parameter manually. Depending on
    /// the constructor the value can be either "URI" or "BINARY".
    #[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct Image<'a> {
        value: Cow<'a, str>,
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
                value: value.into(),
                parameters: parameters!("VALUE", "URI")
            }
        }

        /// Creates a new IMAGE Property with the given value. The value type is
        /// "BINARY" which is why the "ENCODING" parameter with the value
        /// "BASE64" is also added.
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

    impl<'a> From<Image<'a>> for Property<'a> {
        fn from(builder: Image<'a>) -> Self {
            Property {
                key: "IMAGE".into(),
                value: builder.value,
                parameters: builder.parameters
            }
        }
    }
}
