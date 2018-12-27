use components::{Component, Property};
use properties::{
    Action, Description, DtStamp, DtStart, ProdID, Summary, Trigger, TzID, TzOffsetFrom,
    TzOffsetTo, Version, UID
};
use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// The iCalendar object specified as VCALENDAR.
///
/// An `ICalendar` consists of calendar properties and one or more calendar
/// components. Properties are attributes that apply to the calendar object as a
/// whole. (see [RFC5545 3.4 iCalendar Object](https://tools.ietf.org/html/rfc5545#section-3.4))
/// ICalendar can be thought of as the iCalendar file. This is where the
/// specified components are added. To save the object as file, it needs to be
/// written to a file.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ICalendar<'a>(Component<'a>);

impl<'a> ICalendar<'a> {
    /// Creates a new iCalendar object/"VCALENDAR" calendar component. The
    /// "VERSION" and "PRODID" properties are required.
    pub fn new<V, P>(version: V, prodid: P) -> Self
    where
        V: Into<Cow<'a, str>>,
        P: Into<Cow<'a, str>>
    {
        let mut cal = ICalendar(Component::new("VCALENDAR"));
        cal.push(Version::new(version));
        cal.push(ProdID::new(prodid));
        cal
    }

    /// Adds a property to the iCalendar object. Calendar properties are like
    /// calendar attributes.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }

    /// Adds a `Component` to the iCalendar object. This should be only used
    /// for IANA/non-standard components.
    pub fn add_component<C>(&mut self, component: C)
    where
        C: Into<Component<'a>>
    {
        self.0.add_component(component);
    }

    /// Adds an `Event` component to the iCalendar object.
    pub fn add_event(&mut self, event: Event<'a>) {
        self.add_component(event);
    }

    /// Adds a `ToDo` component to the iCalendar object.
    pub fn add_todo(&mut self, todo: ToDo<'a>) {
        self.add_component(todo);
    }

    /// Adds a `Journal` component to the iCalendar object.
    pub fn add_journal(&mut self, journal: Journal<'a>) {
        self.add_component(journal);
    }

    /// Adds a `FreeBusy` component to the iCalendar object.
    pub fn add_freebusy(&mut self, freebusy: FreeBusy<'a>) {
        self.add_component(freebusy);
    }

    /// Adds a `TimeZone` component to the iCalendar object.
    pub fn add_timezone(&mut self, timezone: TimeZone<'a>) {
        self.add_component(timezone);
    }

    /// Creates file with given path and saves iCalendar object.
    pub fn save_file<P: AsRef<Path>>(&self, filename: P) -> io::Result<()> {
        self.save(File::create(filename)?)
    }

    /// Saves iCalendar object in file.
    pub fn save(&self, mut file: File) -> io::Result<()> {
        write!(file, "{}", self)
    }
}

/// The VEVENT calendar component.
///
/// An `Event` component is a grouping of component properties, possibly
/// including an Alarm, that represents a scheduled amount of time on a
/// calendar. (see [RFC5545 3.6.1. Event Component](https://tools.ietf.org/html/rfc5545#section-3.6.1))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Event<'a>(Component<'a>);

impl<'a> Event<'a> {
    /// Creates a new "VEVENT" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        let mut event = Event(Component::new("VEVENT"));
        event.push(UID::new(uid));
        event.push(DtStamp::new(dtstamp));
        event
    }

    /// Adds a property to the event. RFC5545 and RFC7986 specify which
    /// properties can be added to an event.
    pub fn push<P: Into<Property<'a>>>(&mut self, property: P) {
        self.0.add_property(property);
    }

    /// Adds an alarm to the event.
    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.0.add_component(alarm);
    }
}

/// The VTODO calendar component.
///
/// A ToDo component is a grouping of component properties, possibly including
/// an Alarm, that represent an action-item or assignment. (see [RFC5545 3.6.2. To-Do Component](https://tools.ietf.org/html/rfc5545#section-3.6.2))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToDo<'a>(Component<'a>);

impl<'a> ToDo<'a> {
    /// Creates a new "VTODO" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        let mut todo = ToDo(Component::new("VTODO"));
        todo.push(UID::new(uid));
        todo.push(DtStamp::new(dtstamp));
        todo
    }

    /// Adds a property to the to-do. RFC5545 and RFC7986 specify which
    /// properties can be added to a to-do.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }

    /// Adds an alarm to the to-do.
    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.0.add_component(alarm);
    }
}

/// The VJOURNAL calendar component.
///
/// A `Journal` component is a grouping of component properties that represent
/// one or more descriptive text notes associated with a particular calendar
/// date. (see [RFC5545 3.6.3. Journal Component](https://tools.ietf.org/html/rfc5545#section-3.6.3))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Journal<'a>(Component<'a>);

impl<'a> Journal<'a> {
    /// Creates a new "VJOURNAL" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        let mut journal = Journal(Component::new("VJOURNAL"));
        journal.push(UID::new(uid));
        journal.push(DtStamp::new(dtstamp));
        journal
    }

    /// Adds a property to the journal. RFC5545 and RFC7986 specify which
    /// properties can be added to a journal.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }
}

/// The VFREEBUSY calendar component.
///
///  A `FreeBusy` component is a grouping of component properties that
/// represents either a request for free or busy time information, a reply to a
/// request for free or busy time information, or a published set of busy time
/// information. (see [RFC5545 3.6.4. Free/Busy Component Component](https://tools.ietf.org/html/rfc5545#section-3.6.4))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FreeBusy<'a>(Component<'a>);

impl<'a> FreeBusy<'a> {
    /// Creates a new "VFREEBUSY" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        let mut freebusy = FreeBusy(Component::new("VFREEBUSY"));
        freebusy.push(UID::new(uid));
        freebusy.push(DtStamp::new(dtstamp));
        freebusy
    }

    /// Adds a property to the free busy schedule. The RFC5545 specifies which
    /// properties can be added to a free busy schedule.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }
}

/// The VTIMEZONE calendar component.
///
///  A `TimeZone` component is unambiguously defined by the set of time
/// measurement rules (`ZoneTime`) determined by the governing body for a given
/// geographic area. (see [RFC5545 3.6.5. Time Zone Component Component](https://tools.ietf.org/html/rfc5545#section-3.6.5))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeZone<'a>(Component<'a>);

impl<'a> TimeZone<'a> {
    /// Creates a new "VTIMEZONE" calendar component. The "TZID" property and
    /// at least one zone time component ("STANDARD" or "DAYLIGHT"
    /// sub-component) are required.
    pub fn new<S>(tzid: S, definition: ZoneTime<'a>) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        let mut timezone = TimeZone(Component::new("VTIMEZONE"));
        timezone.push(TzID::new(tzid));
        timezone.add_zonetime(definition);
        timezone
    }

    /// Adds a property to the time zone. The RFC5545 specifies which
    /// properties can be added to a time zone.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }

    /// Adds an additional zone time to the time zone. For more time zone
    /// definitions, the IANA database could prove helpful.
    pub fn add_zonetime(&mut self, zone_time: ZoneTime<'a>) {
        self.0.add_component(zone_time);
    }
}

/// The STRANDARD or DAYLIGHT sub-component of VTIMEZONE.
///
///  Each "VTIMEZONE" calendar component consists of a collection of one or more
/// sub-components that describe the rule for a particular observance (either a
/// Standard Time or a Daylight Saving Time observance). (see [RFC5545 3.6.5. Time Zone Component Component](https://tools.ietf.org/html/rfc5545#page-63))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZoneTime<'a>(Component<'a>);

impl<'a> ZoneTime<'a> {
    /// Creates a new "STANDARD" sub-component. The "DTSTART", "TZOFFSETFROM"
    /// and "TZOFFSETTO" properties are required. The "STANDARD" sub-component
    /// consists of a collection of properties that describe Standard Time.
    pub fn standard<S, T, F>(dtstart: S, tz_offset_from: F, tz_offset_to: T) -> Self
    where
        S: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>
    {
        let mut zone_time = ZoneTime(Component::new("STANDARD"));
        zone_time.push(DtStart::new(dtstart));
        zone_time.push(TzOffsetFrom::new(tz_offset_from));
        zone_time.push(TzOffsetTo::new(tz_offset_to));
        zone_time
    }

    /// Creates a new "DAYLIGHT" sub-component. The "DTSTART", "TZOFFSETFROM"
    /// and "TZOFFSETTO" properties are required. The "DAYLIGHT" sub-component
    /// consists of a collection of properties that describe Daylight Saving
    /// Time.
    pub fn daylight<S, T, F>(dtstart: S, tz_offset_from: F, tz_offset_to: T) -> Self
    where
        S: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>
    {
        let mut zone_time = ZoneTime(Component::new("DAYLIGHT"));
        zone_time.push(DtStart::new(dtstart));
        zone_time.push(TzOffsetFrom::new(tz_offset_from));
        zone_time.push(TzOffsetTo::new(tz_offset_to));
        zone_time
    }

    /// Adds a property to the zone time. The RFC5545 specifies which
    /// properties can be added to a zone time.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }
}

/// The VALARM calendar component, a sub-component for VEVENT and VTODO.
///
/// An `Alarm` component is a grouping of component properties that is a
/// reminder or alarm for an event or a to-do. For example, it may be used to
/// define a reminder for a pending event or an overdue to-do. (see [RFC5545 3.6.6. Alarm Component](https://tools.ietf.org/html/rfc5545#section-3.6.6))
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Alarm<'a>(Component<'a>);

// The specific constructors use the specific property builder types since the
// required properties can have defined parameters.
impl<'a> Alarm<'a> {
    /// Creates a new "VALARM" calendar component. The "ACTION" and "TRIGGER"
    /// properties are required.
    pub fn new(action: Action<'a>, trigger: Trigger<'a>) -> Self {
        let mut alarm = Alarm(Component::new("VALARM"));
        alarm.push(action);
        alarm.push(trigger);
        alarm
    }

    /// Creates a new audio alarm. The "TRIGGER" property is required.
    pub fn audio(trigger: Trigger<'a>) -> Self {
        Alarm::new(Action::new("AUDIO"), trigger)
    }

    /// Creates a new display alarm. The "TRIGGER" and "DESCRIPTION" properties
    /// are required.
    pub fn display(trigger: Trigger<'a>, description: Description<'a>) -> Self {
        let mut alarm = Alarm::new(Action::new("DISPLAY"), trigger);
        alarm.push(description);
        alarm
    }

    /// Creates a new email alarm. The "TRIGGER", "DESCRIPTION" and "SUMMARY"
    /// properties are required.
    pub fn email(trigger: Trigger<'a>, description: Description<'a>, summary: Summary<'a>) -> Self {
        let mut alarm = Alarm::new(Action::new("EMAIL"), trigger);
        alarm.push(description);
        alarm.push(summary);
        alarm
    }

    /// Adds a property to the alarm. The RFC5545 specifies which property can
    /// be added depending on the kind of alarm.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }
}

impl_component!(ICalendar);
impl_component!(Event);
impl_component!(ToDo);
impl_component!(Journal);
impl_component!(FreeBusy);
impl_component!(TimeZone);
impl_component!(ZoneTime);
impl_component!(Alarm);
