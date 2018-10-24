use components::{Component, Property};
use properties::{
    Action, Description, DtStamp, DtStart, ProdID, Summary, Trigger, TzID, TzOffsetFrom,
    TzOffsetTo, Version, UID,
};
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ICalendar<'a>(Component<'a>);

impl<'a> ICalendar<'a> {
    pub fn new<V, P>(version: V, prodid: P) -> Self
    where
        V: Into<Cow<'a, str>>,
        P: Into<Cow<'a, str>>,
    {
        let mut cal = ICalendar(Component::new("VCALENDAR"));
        cal.push(Version::new(version));
        cal.push(ProdID::new(prodid));
        cal
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }

    pub fn add_component<C>(&mut self, component: C)
    where
        C: Into<Component<'a>>,
    {
        self.0.add_component(component);
    }

    pub fn add_event(&mut self, event: Event<'a>) {
        self.add_component(event);
    }

    pub fn add_todo(&mut self, todo: ToDo<'a>) {
        self.add_component(todo);
    }

    pub fn add_journal(&mut self, journal: Journal<'a>) {
        self.add_component(journal);
    }

    pub fn add_freebusy(&mut self, freebusy: FreeBusy<'a>) {
        self.add_component(freebusy);
    }

    pub fn add_timezone(&mut self, timezone: TimeZone<'a>) {
        self.add_component(timezone);
    }
}

// TODO: Docs explaining that these properties are added by default.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Event<'a>(Component<'a>);

impl<'a> Event<'a> {
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let mut event = Event(Component::new("VEVENT"));
        event.push(UID::new(uid));
        event.push(DtStamp::new(dtstamp));
        event
    }

    pub fn push<P: Into<Property<'a>>>(&mut self, property: P) {
        self.0.add_property(property);
    }

    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.0.add_component(alarm);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToDo<'a>(Component<'a>);

impl<'a> ToDo<'a> {
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let mut todo = ToDo(Component::new("VTODO"));
        todo.push(UID::new(uid));
        todo.push(DtStamp::new(dtstamp));
        todo
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }

    pub fn add_alarm(&mut self, alarm: Alarm<'a>) {
        self.0.add_component(alarm);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Journal<'a>(Component<'a>);

impl<'a> Journal<'a> {
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let mut journal = Journal(Component::new("VJOURNAL"));
        journal.push(UID::new(uid));
        journal.push(DtStamp::new(dtstamp));
        journal
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FreeBusy<'a>(Component<'a>);

impl<'a> FreeBusy<'a> {
    pub fn new<U, D>(uid: U, dtstamp: D) -> Self
    where
        U: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let mut freebusy = FreeBusy(Component::new("VFREEBUSY"));
        freebusy.push(UID::new(uid));
        freebusy.push(DtStamp::new(dtstamp));
        freebusy
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeZone<'a>(Component<'a>);

impl<'a> TimeZone<'a> {
    pub fn new<S>(tzid: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let mut timezone = TimeZone(Component::new("VTIMEZONE"));
        timezone.push(TzID::new(tzid));
        timezone
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }

    pub fn add_zonetime(&mut self, zone_time: ZoneTime<'a>) {
        self.0.add_component(zone_time);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZoneTime<'a>(Component<'a>);

impl<'a> ZoneTime<'a> {
    pub fn standard<S, T, F>(dtstart: S, tz_offset_to: T, tz_offset_from: F) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
    {
        let mut zone_time = ZoneTime(Component::new("STANDARD"));
        zone_time.push(DtStart::new(dtstart));
        zone_time.push(TzOffsetFrom::new(tz_offset_from));
        zone_time.push(TzOffsetTo::new(tz_offset_to));
        zone_time
    }

    pub fn daylight<S, T, F>(dtstart: S, tz_offset_to: T, tz_offset_from: F) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        F: Into<Cow<'a, str>>,
    {
        let mut zone_time = ZoneTime(Component::new("DAYLIGHT"));
        zone_time.push(DtStart::new(dtstart));
        zone_time.push(TzOffsetFrom::new(tz_offset_from));
        zone_time.push(TzOffsetTo::new(tz_offset_to));
        zone_time
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Alarm<'a>(Component<'a>);

impl<'a> Alarm<'a> {
    fn new<S>(kind: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        let mut alarm = Alarm(Component::new("VALARM"));
        alarm.push(Action::new(kind));
        alarm
    }

    pub fn audio<T>(trigger: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        let mut alarm = Alarm::new("AUDIO");
        alarm.push(Trigger::new(trigger));
        alarm
    }

    pub fn display<T, D>(trigger: T, description: D) -> Self
    where
        T: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
    {
        let mut alarm = Alarm::new("DISPLAY");
        alarm.push(Trigger::new(trigger));
        alarm.push(Description::new(description));
        alarm
    }

    pub fn email<T, D, S>(trigger: T, description: D, summary: S) -> Self
    where
        T: Into<Cow<'a, str>>,
        D: Into<Cow<'a, str>>,
        S: Into<Cow<'a, str>>,
    {
        let mut alarm = Alarm::new("EMAIL");
        alarm.push(Trigger::new(trigger));
        alarm.push(Description::new(description));
        alarm.push(Summary::new(summary));
        alarm
    }

    pub fn push<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        self.0.add_property(property);
    }
}

impl_display_comps!(ICalendar);
impl_display_comps!(Event);
impl_display_comps!(ToDo);
impl_display_comps!(Journal);
impl_display_comps!(FreeBusy);
impl_display_comps!(TimeZone);
impl_display_comps!(ZoneTime);
impl_display_comps!(Alarm);

impl_component_conversion!(ICalendar);
impl_component_conversion!(Event);
impl_component_conversion!(ToDo);
impl_component_conversion!(Journal);
impl_component_conversion!(FreeBusy);
impl_component_conversion!(TimeZone);
impl_component_conversion!(ZoneTime);
impl_component_conversion!(Alarm);
