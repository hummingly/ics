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
/// The ICalendar struct can be thought of as the iCalendar object. This is
/// where the specified components are added. To save the object as file, it
/// needs to be written to a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

    /// Generic convenience method to write the content of the iCalendar object
    /// to a writer in the iCalendar format.
    pub fn write<W>(&self, mut writer: W) -> io::Result<()>
    where
        W: Write
    {
        write!(writer, "{}", self)
    }

    /// Creates a file from the path and saves the content of the iCalendar
    /// object in the iCalendar format inside the file.
    pub fn save_file<P>(&self, filename: P) -> io::Result<()>
    where
        P: AsRef<Path>
    {
        self.write(File::create(filename)?)
    }
}

impl<'a> fmt::Display for ICalendar<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<ICalendar<'a>> for Component<'a> {
    fn from(component: ICalendar<'a>) -> Self {
        component.0
    }
}

/// The VEVENT calendar component.
///
/// An `Event` component is a grouping of component properties, possibly
/// including an Alarm, that represents a scheduled amount of time on a
/// calendar. (see [RFC5545 3.6.1. Event Component](https://tools.ietf.org/html/rfc5545#section-3.6.1))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Event<'a> {
    properties: Vec<Property<'a>>,
    alarms: Vec<Alarm<'a>>
}

impl<'a> Event<'a> {
    const COMPONENT_NAME: &'static str = "VEVENT";
    /// Creates a new "VEVENT" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        Self {
            properties: vec![UID::new(uid).into(), DtStamp::new(dtstamp).into()],
            alarms: Vec::new()
        }
    }

    /// Adds a property to the event. RFC5545 and RFC7986 specify which
    /// properties can be added to an event.
    pub fn push<P: Into<Property<'a>>>(&mut self, property: P) {
        self.properties.push(property.into());
    }

    /// Adds an alarm to the event.
    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.alarms.push(alarm);
    }
}

impl<'a> fmt::Display for Event<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", Self::COMPONENT_NAME)?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for alarm in &self.alarms {
            write!(f, "{}", alarm)?;
        }
        writeln!(f, "END:{}\r", Self::COMPONENT_NAME)
    }
}

impl<'a> From<Event<'a>> for Component<'a> {
    fn from(component: Event<'a>) -> Self {
        Component {
            name: Event::COMPONENT_NAME.into(),
            properties: component.properties,
            subcomponents: component.alarms.into_iter().map(Component::from).collect()
        }
    }
}

/// The VTODO calendar component.
///
/// A ToDo component is a grouping of component properties, possibly including
/// an Alarm, that represent an action-item or assignment. (see [RFC5545 3.6.2. To-Do Component](https://tools.ietf.org/html/rfc5545#section-3.6.2))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToDo<'a> {
    properties: Vec<Property<'a>>,
    alarms: Vec<Alarm<'a>>
}

impl<'a> ToDo<'a> {
    const COMPONENT_NAME: &'static str = "VTODO";
    /// Creates a new "VTODO" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        Self {
            properties: vec![UID::new(uid).into(), DtStamp::new(dtstamp).into()],
            alarms: Vec::new()
        }
    }

    /// Adds a property to the to-do. RFC5545 and RFC7986 specify which
    /// properties can be added to a to-do.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.properties.push(property.into());
    }

    /// Adds an alarm to the to-do.
    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.alarms.push(alarm);
    }
}

impl<'a> fmt::Display for ToDo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", Self::COMPONENT_NAME)?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for alarm in &self.alarms {
            write!(f, "{}", alarm)?;
        }
        writeln!(f, "END:{}\r", Self::COMPONENT_NAME)
    }
}

impl<'a> From<ToDo<'a>> for Component<'a> {
    fn from(component: ToDo<'a>) -> Self {
        Component {
            name: ToDo::COMPONENT_NAME.into(),
            properties: component.properties,
            subcomponents: component.alarms.into_iter().map(Component::from).collect()
        }
    }
}

/// The VJOURNAL calendar component.
///
/// A `Journal` component is a grouping of component properties that represent
/// one or more descriptive text notes associated with a particular calendar
/// date. (see [RFC5545 3.6.3. Journal Component](https://tools.ietf.org/html/rfc5545#section-3.6.3))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Journal<'a>(Vec<Property<'a>>);

impl<'a> Journal<'a> {
    const COMPONENT_NAME: &'static str = "VJOURNAL";
    /// Creates a new "VJOURNAL" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        Self(vec![UID::new(uid).into(), DtStamp::new(dtstamp).into()])
    }

    /// Adds a property to the journal. RFC5545 and RFC7986 specify which
    /// properties can be added to a journal.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.push(property.into());
    }
}

impl<'a> fmt::Display for Journal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", Self::COMPONENT_NAME)?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:{}\r", Self::COMPONENT_NAME)
    }
}

impl<'a> From<Journal<'a>> for Component<'a> {
    fn from(component: Journal<'a>) -> Self {
        Component {
            name: Journal::COMPONENT_NAME.into(),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}

/// The VFREEBUSY calendar component.
///
///  A `FreeBusy` component is a grouping of component properties that
/// represents either a request for free or busy time information, a reply to a
/// request for free or busy time information, or a published set of busy time
/// information. (see [RFC5545 3.6.4. Free/Busy Component Component](https://tools.ietf.org/html/rfc5545#section-3.6.4))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FreeBusy<'a>(Vec<Property<'a>>);

impl<'a> FreeBusy<'a> {
    const COMPONENT_NAME: &'static str = "VFREEBUSY";
    /// Creates a new "VFREEBUSY" calendar component. The "UID" and "DTSTAMP"
    /// properties are required. A UID should be generated randomly for security
    /// reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        Self(vec![UID::new(uid).into(), DtStamp::new(dtstamp).into()])
    }

    /// Adds a property to the free busy schedule. The RFC5545 specifies which
    /// properties can be added to a free busy schedule.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.push(property.into());
    }
}

impl<'a> fmt::Display for FreeBusy<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", Self::COMPONENT_NAME)?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:{}\r", Self::COMPONENT_NAME)
    }
}

impl<'a> From<FreeBusy<'a>> for Component<'a> {
    fn from(component: FreeBusy<'a>) -> Self {
        Component {
            name: FreeBusy::COMPONENT_NAME.into(),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}

/// The VTIMEZONE calendar component.
///
///  A `TimeZone` component is unambiguously defined by the set of time
/// measurement rules (`ZoneTime`) determined by the governing body for a given
/// geographic area. (see [RFC5545 3.6.5. Time Zone Component Component](https://tools.ietf.org/html/rfc5545#section-3.6.5))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeZone<'a> {
    properties: Vec<Property<'a>>,
    zone_times: Vec<ZoneTime<'a>>
}

impl<'a> TimeZone<'a> {
    const COMPONENT_NAME: &'static str = "VTIMEZONE";
    /// Creates a new "VTIMEZONE" calendar component. The "TZID" property and
    /// at least one zone time component ("STANDARD" or "DAYLIGHT"
    /// sub-component) are required.
    pub fn new<S>(tzid: S, definition: ZoneTime<'a>) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Self {
            properties: vec![TzID::new(tzid).into()],
            zone_times: vec![definition]
        }
    }

    /// Adds a property to the time zone. The RFC5545 specifies which
    /// properties can be added to a time zone.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.properties.push(property.into());
    }

    /// Adds an additional zone time to the time zone. For more time zone
    /// definitions, the IANA database could prove helpful.
    pub fn add_zonetime(&mut self, zone_time: ZoneTime<'a>) {
        self.zone_times.push(zone_time);
    }
}

impl<'a> fmt::Display for TimeZone<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", Self::COMPONENT_NAME)?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for zone in &self.zone_times {
            write!(f, "{}", zone)?;
        }
        writeln!(f, "END:{}\r", Self::COMPONENT_NAME)
    }
}

impl<'a> From<TimeZone<'a>> for Component<'a> {
    fn from(component: TimeZone<'a>) -> Self {
        Component {
            name: TimeZone::COMPONENT_NAME.into(),
            properties: component.properties,
            subcomponents: component
                .zone_times
                .into_iter()
                .map(Component::from)
                .collect()
        }
    }
}

/// The STRANDARD or DAYLIGHT sub-component of VTIMEZONE.
///
///  Each "VTIMEZONE" calendar component consists of a collection of one or more
/// sub-components that describe the rule for a particular observance (either a
/// Standard Time or a Daylight Saving Time observance). (see [RFC5545 3.6.5. Time Zone Component Component](https://tools.ietf.org/html/rfc5545#page-63))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZoneTime<'a> {
    name: ZoneTimeType,
    properties: Vec<Property<'a>>
}

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
        Self {
            name: ZoneTimeType::Standard,
            properties: vec![
                DtStart::new(dtstart).into(),
                TzOffsetFrom::new(tz_offset_from).into(),
                TzOffsetTo::new(tz_offset_to).into(),
            ]
        }
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
        Self {
            name: ZoneTimeType::Daylight,
            properties: vec![
                DtStart::new(dtstart).into(),
                TzOffsetFrom::new(tz_offset_from).into(),
                TzOffsetTo::new(tz_offset_to).into(),
            ]
        }
    }

    /// Adds a property to the zone time. The RFC5545 specifies which
    /// properties can be added to a zone time.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.properties.push(property.into());
    }
}

impl<'a> fmt::Display for ZoneTime<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", self.name.as_str())?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:{}\r", self.name.as_str())
    }
}

impl<'a> From<ZoneTime<'a>> for Component<'a> {
    fn from(component: ZoneTime<'a>) -> Self {
        Component {
            name: component.name.as_cow(),
            properties: component.properties,
            subcomponents: Vec::new()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ZoneTimeType {
    Standard,
    Daylight
}

impl ZoneTimeType {
    pub fn as_str(&self) -> &'static str {
        match *self {
            ZoneTimeType::Daylight => "DAYLIGHT",
            ZoneTimeType::Standard => "STANDARD"
        }
    }

    pub fn as_cow<'a>(&self) -> Cow<'a, str> {
        Cow::Borrowed(self.as_str())
    }
}

/// The VALARM calendar component, a sub-component for VEVENT and VTODO.
///
/// An `Alarm` component is a grouping of component properties that is a
/// reminder or alarm for an event or a to-do. For example, it may be used to
/// define a reminder for a pending event or an overdue to-do. (see [RFC5545 3.6.6. Alarm Component](https://tools.ietf.org/html/rfc5545#section-3.6.6))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Alarm<'a>(Vec<Property<'a>>);

// The specific constructors use the specific property builder types since the
// required properties can have defined parameters.
impl<'a> Alarm<'a> {
    const COMPONENT_NAME: &'static str = "VALARM";
    /// Creates a new "VALARM" calendar component. The "ACTION" and "TRIGGER"
    /// properties are required.
    pub fn new(action: Action<'a>, trigger: Trigger<'a>) -> Self {
        Self(vec![action.into(), trigger.into()])
    }

    /// Creates a new audio alarm. The "TRIGGER" property is required.
    pub fn audio(trigger: Trigger<'a>) -> Self {
        Self::new(Action::audio(), trigger)
    }

    /// Creates a new display alarm. The "TRIGGER" and "DESCRIPTION" properties
    /// are required.
    pub fn display(trigger: Trigger<'a>, description: Description<'a>) -> Self {
        Self(vec![
            Action::display().into(),
            trigger.into(),
            description.into(),
        ])
    }

    /// Creates a new email alarm. The "TRIGGER", "DESCRIPTION" and "SUMMARY"
    /// properties are required.
    pub fn email(trigger: Trigger<'a>, description: Description<'a>, summary: Summary<'a>) -> Self {
        Self(vec![
            Action::email().into(),
            trigger.into(),
            description.into(),
            summary.into(),
        ])
    }

    /// Adds a property to the alarm. The RFC5545 specifies which property can
    /// be added depending on the kind of alarm.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.push(property.into());
    }
}

impl<'a> fmt::Display for Alarm<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", Self::COMPONENT_NAME)?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:{}\r", Self::COMPONENT_NAME)
    }
}

impl<'a> From<Alarm<'a>> for Component<'a> {
    fn from(component: Alarm<'a>) -> Self {
        Component {
            name: Alarm::COMPONENT_NAME.into(),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}
