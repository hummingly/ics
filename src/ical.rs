use crate::components::{Component, Property};
use crate::properties::{
    Action, Description, DtStamp, DtStart, ProdID, Summary, Trigger, TzID, TzOffsetFrom,
    TzOffsetTo, Version, UID
};
use std::borrow::Cow;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

/// The iCalendar object specified as `VCALENDAR` component
///
/// An [`ICalendar`] consists of calendar properties and one or more calendar
/// components. Properties are attributes that apply to the calendar object as a
/// whole. (see [RFC5545 3.4 iCalendar Object](https://tools.ietf.org/html/rfc5545#section-3.4))
/// The ICalendar struct can be thought of as the iCalendar object. This is
/// where the specified components are added. To save the object as file, it
/// needs to be written to a file.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ICalendar<'a>(Component<'a>);

impl<'a> ICalendar<'a> {
    /// Creates a new iCalendar object/`VCALENDAR` calendar component. The
    /// `VERSION` and `PRODID` properties are required.
    pub fn new<V, P>(version: V, prodid: P) -> Self
    where
        V: Into<Cow<'a, str>>,
        P: Into<Cow<'a, str>>
    {
        ICalendar(Component {
            name: Cow::Borrowed("VCALENDAR"),
            properties: vec![Version::new(version).into(), ProdID::new(prodid).into()],
            subcomponents: Vec::new()
        })
    }

    /// Adds a property to the iCalendar object. Calendar properties are like
    /// calendar attributes.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.add_property(property);
    }

    /// Adds a [`Component`] to the iCalendar object. This should be only used
    /// for IANA/non-standard components.
    pub fn add_component<C>(&mut self, component: C)
    where
        C: Into<Component<'a>>
    {
        self.0.add_component(component);
    }

    /// Adds an `VEVENT` component to the iCalendar object.
    pub fn add_event(&mut self, event: Event<'a>) {
        self.add_component(event);
    }

    /// Adds a `VTODO` component to the iCalendar object.
    pub fn add_todo(&mut self, todo: ToDo<'a>) {
        self.add_component(todo);
    }

    /// Adds a `VJOURNAL` component to the iCalendar object.
    pub fn add_journal(&mut self, journal: Journal<'a>) {
        self.add_component(journal);
    }

    /// Adds a `VFREEBUSY` component to the iCalendar object.
    pub fn add_freebusy(&mut self, freebusy: FreeBusy<'a>) {
        self.add_component(freebusy);
    }

    /// Adds a `VTIMEZONE` component to the iCalendar object.
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
    /// object in the iCalendar format.
    pub fn save_file<P>(&self, filename: P) -> io::Result<()>
    where
        P: AsRef<Path>
    {
        self.write(File::create(filename)?)
    }
}

impl fmt::Display for ICalendar<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a> From<ICalendar<'a>> for Component<'a> {
    fn from(component: ICalendar<'a>) -> Self {
        component.0
    }
}

/// The `VEVENT` calendar component
///
/// An [`Event`] component is a grouping of component properties, possibly
/// including an [`Alarm`], that represents a scheduled amount of time on a
/// calendar. (see [RFC5545 3.6.1. Event Component](https://tools.ietf.org/html/rfc5545#section-3.6.1))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Event<'a> {
    properties: Vec<Property<'a>>,
    alarms: Vec<Alarm<'a>>
}

impl<'a> Event<'a> {
    /// Creates a new `VEVENT` calendar component. The `UID` and `DTSTAMP`
    /// properties are required. A `UID` should be generated randomly for
    /// security reasons.
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
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.properties.push(property.into());
    }

    /// Adds a `VALARM` to the event.
    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.alarms.push(alarm);
    }
}

impl fmt::Display for Event<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:VEVENT\r")?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for component in &self.alarms {
            write!(f, "{}", component)?;
        }
        writeln!(f, "END:VEVENT\r")
    }
}

impl<'a> From<Event<'a>> for Component<'a> {
    fn from(component: Event<'a>) -> Self {
        Component {
            name: Cow::Borrowed("VEVENT"),
            properties: component.properties,
            subcomponents: component.alarms.into_iter().map(Component::from).collect()
        }
    }
}

/// The `VTODO` calendar component
///
/// A [`ToDo`] component is a grouping of component properties, possibly
/// including an [`Alarm`], that represent an action-item or assignment. (see [RFC5545 3.6.2. To-Do Component](https://tools.ietf.org/html/rfc5545#section-3.6.2))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToDo<'a> {
    properties: Vec<Property<'a>>,
    alarms: Vec<Alarm<'a>>
}

impl<'a> ToDo<'a> {
    /// Creates a new `VTODO` calendar component. The `UID` and `DTSTAMP`
    /// properties are required. A `UID` should be generated randomly for
    /// security reasons.
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

    /// Adds an `VALARM` to the to-do.
    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.alarms.push(alarm);
    }
}

impl fmt::Display for ToDo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:VTODO\r")?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for component in &self.alarms {
            write!(f, "{}", component)?;
        }
        writeln!(f, "END:VTODO\r")
    }
}

impl<'a> From<ToDo<'a>> for Component<'a> {
    fn from(component: ToDo<'a>) -> Self {
        Component {
            name: Cow::Borrowed("VTODO"),
            properties: component.properties,
            subcomponents: component.alarms.into_iter().map(Component::from).collect()
        }
    }
}

/// The `VJOURNAL` calendar component
///
/// A [`Journal`] component is a grouping of component properties that represent
/// one or more descriptive text notes associated with a particular calendar
/// date. (see [RFC5545 3.6.3. Journal Component](https://tools.ietf.org/html/rfc5545#section-3.6.3))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Journal<'a>(Vec<Property<'a>>);

impl<'a> Journal<'a> {
    /// Creates a new `VJOURNAL` calendar component. The `UID` and `DTSTAMP`
    /// properties are required. A `UID` should be generated randomly for
    /// security reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        Journal(vec![UID::new(uid).into(), DtStamp::new(dtstamp).into()])
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

impl fmt::Display for Journal<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:VJOURNAL\r")?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:VJOURNAL\r")
    }
}

impl<'a> From<Journal<'a>> for Component<'a> {
    fn from(component: Journal<'a>) -> Self {
        Component {
            name: Cow::Borrowed("VJOURNAL"),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}

/// The `VFREEBUSY` calendar component
///
///  A [`FreeBusy`] component is a grouping of component properties that
/// represents either a request for free or busy time information, a reply to a
/// request for free or busy time information, or a published set of busy time
/// information. (see [RFC5545 3.6.4. Free/Busy Component Component](https://tools.ietf.org/html/rfc5545#section-3.6.4))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FreeBusy<'a>(Vec<Property<'a>>);

impl<'a> FreeBusy<'a> {
    /// Creates a new `VFREEBUSY` calendar component. The `UID` and `DTSTAMP`
    /// properties are required. A `UID` should be generated randomly for
    /// security reasons.
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>
    {
        FreeBusy(vec![UID::new(uid).into(), DtStamp::new(dtstamp).into()])
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

impl fmt::Display for FreeBusy<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:VFREEBUSY\r")?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:VFREEBUSY\r")
    }
}

impl<'a> From<FreeBusy<'a>> for Component<'a> {
    fn from(component: FreeBusy<'a>) -> Self {
        Component {
            name: Cow::Borrowed("VFREEBUSY"),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}

/// The `VTIMEZONE` calendar component
///
///  A [`TimeZone`] component is unambiguously defined by the set of time
/// measurement rules ([`Standard`] or [`Daylight`]) determined by the governing
/// body for a given geographic area. (see [RFC5545 3.6.5. Time Zone Component Component](https://tools.ietf.org/html/rfc5545#section-3.6.5))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeZone<'a> {
    properties: Vec<Property<'a>>,
    zone_times: Vec<ZoneTime<'a>>
}

impl<'a> TimeZone<'a> {
    /// Creates a new `VTIMEZONE` calendar component from `TZID` property and
    /// `STANDARD` sub-component. More definitions can be added with
    /// [`TimeZone::add_standard()`] or [`TimeZone::add_daylight()`].
    pub fn standard<S>(tzid: S, definition: Standard<'a>) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Self {
            properties: vec![TzID::new(tzid).into()],
            zone_times: vec![ZoneTime::Standard(definition)]
        }
    }

    /// Creates a new `VTIMEZONE` calendar component from `TZID` property and
    /// `DAYLIGHT` sub-component. More definitions can be added with
    /// [`TimeZone::add_standard()`] or [`TimeZone::add_daylight()`].
    pub fn daylight<S>(tzid: S, definition: Daylight<'a>) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Self {
            properties: vec![TzID::new(tzid).into()],
            zone_times: vec![ZoneTime::Daylight(definition)]
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

    /// Adds an additional `STANDARD` definition to the time zone. For more
    /// time zone definitions, the IANA database could prove helpful.
    pub fn add_standard(&mut self, definition: Standard<'a>) {
        self.zone_times.push(ZoneTime::Standard(definition));
    }

    /// Adds an additional `DAYLIGHT` definition to the time zone. For more
    /// time zone definitions, the IANA database could prove helpful.
    pub fn add_daylight(&mut self, definition: Daylight<'a>) {
        self.zone_times.push(ZoneTime::Daylight(definition));
    }
}

impl fmt::Display for TimeZone<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:VTIMEZONE\r")?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for component in &self.zone_times {
            match component {
                ZoneTime::Daylight(p) => write!(f, "{}", p),
                ZoneTime::Standard(p) => write!(f, "{}", p)
            }?;
        }
        writeln!(f, "END:VTIMEZONE\r")
    }
}

impl<'a> From<TimeZone<'a>> for Component<'a> {
    fn from(component: TimeZone<'a>) -> Self {
        Component {
            name: Cow::Borrowed("VTIMEZONE"),
            properties: component.properties,
            subcomponents: component
                .zone_times
                .into_iter()
                .map(Component::from)
                .collect()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum ZoneTime<'a> {
    /// Standard Time
    Standard(Standard<'a>),
    /// Daylight Saving Time
    Daylight(Daylight<'a>)
}

impl<'a> From<ZoneTime<'a>> for Component<'a> {
    fn from(component: ZoneTime<'a>) -> Self {
        match component {
            ZoneTime::Daylight(p) => Self::from(p),
            ZoneTime::Standard(p) => Self::from(p)
        }
    }
}

/// The `STANDARD` calendar sub-component of `VTIMEZONE`
///
/// A [`Standard`] component is a sub-component of the [`TimeZone`] component
/// which describes rules for standard time, also known as winter time.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Standard<'a>(Vec<Property<'a>>);

impl<'a> Standard<'a> {
    /// Creates a new `STANDARD` sub-component. The properties `DTSTART`,
    /// `TZOFFSETFROM` and `TZOFFSETTO` are required.
    pub fn new<S, T, F>(dtstart: S, tz_offset_from: F, tz_offset_to: T) -> Self
    where
        S: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>
    {
        Standard(vec![
            DtStart::new(dtstart).into(),
            TzOffsetFrom::new(tz_offset_from).into(),
            TzOffsetTo::new(tz_offset_to).into(),
        ])
    }

    /// Adds a property to the zone time. The RFC5545 specifies which
    /// properties can be added to a zone time.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.push(property.into());
    }
}

impl fmt::Display for Standard<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:STANDARD\r")?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:STANDARD\r")
    }
}

impl<'a> From<Standard<'a>> for Component<'a> {
    fn from(component: Standard<'a>) -> Self {
        Component {
            name: Cow::Borrowed("STANDARD"),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}

/// The `DAYLIGHT` calendar sub-component of `VTIMEZONE`
///
/// A `[`Daylight`] component is a sub-component of the [`TimeZone`] component
/// which describes rules for daylight saving time, also known as advanced time,
/// summer time, or legal time in certain countries.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Daylight<'a>(Vec<Property<'a>>);

impl<'a> Daylight<'a> {
    /// Creates a new `DAYLIGHT` sub-component. The properties `DTSTART`,
    /// `TZOFFSETFROM` and `TZOFFSETTO` are required.
    pub fn new<S, T, F>(dtstart: S, tz_offset_from: F, tz_offset_to: T) -> Self
    where
        S: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>
    {
        Daylight(vec![
            DtStart::new(dtstart).into(),
            TzOffsetFrom::new(tz_offset_from).into(),
            TzOffsetTo::new(tz_offset_to).into(),
        ])
    }

    /// Adds a property to the zone time. The RFC5545 specifies which
    /// properties can be added to a zone time.
    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.0.push(property.into());
    }
}

impl fmt::Display for Daylight<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:DAYLIGHT\r")?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:DAYLIGHT\r")
    }
}

impl<'a> From<Daylight<'a>> for Component<'a> {
    fn from(component: Daylight<'a>) -> Self {
        Component {
            name: Cow::Borrowed("DAYLIGHT"),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}

/// The `VALARM` calendar sub-component of `VEVENT` and `VTODO`.
///
/// An [`Alarm`] component is a grouping of component properties that is a
/// reminder or alarm for an [`Event`] or a [`ToDo`]. For example, it may be
/// used to define a reminder for a pending event or an overdue to-do. (see [RFC5545 3.6.6. Alarm Component](https://tools.ietf.org/html/rfc5545#section-3.6.6))
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Alarm<'a>(Vec<Property<'a>>);

// The specific constructors use the specific property builder types since the
// required properties can have defined parameters.
impl<'a> Alarm<'a> {
    /// Creates a new `VALARM` calendar component. The `ACTION` and `TRIGGER`
    /// properties are required.
    pub fn new(action: Action<'a>, trigger: Trigger<'a>) -> Self {
        Alarm(vec![action.into(), trigger.into()])
    }

    /// Creates a new audio alarm. The `TRIGGER` property is required.
    pub fn audio(trigger: Trigger<'a>) -> Self {
        Self::new(Action::audio(), trigger)
    }

    /// Creates a new display alarm. The `TRIGGER` and `DESCRIPTION` properties
    /// are required.
    pub fn display(trigger: Trigger<'a>, description: Description<'a>) -> Self {
        Alarm(vec![
            Action::display().into(),
            trigger.into(),
            description.into(),
        ])
    }

    /// Creates a new email alarm. The `TRIGGER`, `DESCRIPTION` and `SUMMARY`
    /// properties are required.
    pub fn email(trigger: Trigger<'a>, description: Description<'a>, summary: Summary<'a>) -> Self {
        Alarm(vec![
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

impl fmt::Display for Alarm<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:VALARM\r")?;
        for property in &self.0 {
            write!(f, "{}", property)?;
        }
        writeln!(f, "END:VALARM\r")
    }
}

impl<'a> From<Alarm<'a>> for Component<'a> {
    fn from(component: Alarm<'a>) -> Self {
        Component {
            name: Cow::Borrowed("VALARM"),
            properties: component.0,
            subcomponents: Vec::new()
        }
    }
}
