//! In the RFC 5545 specified properties except for IANA and non-standard
//! properties ("X"-prefix parameters).
//!
//! Properties are key-value pairs which can have optionally several
//! parameters. A property forms a content line which is line folded (CRLF +
//! whitespace) after 75 bytes.
//!
//! For more information on the properties, please refer to the specification [RFC 5545 3.7. Calendar Properties](https://tools.ietf.org/html/rfc5545#section-3.7).
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
property_builder!(FBTime, "FREEBUSY");
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
impl_default_property!(FBTime);
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
