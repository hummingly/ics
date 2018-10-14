/// The high-level interface of the library.
/// Common objects in an iCalendar file.
pub mod parameter_builder;
pub mod property_builder;
use components::{Component, Property};
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ICalendar(Component);

// TODO: Docs explaining that these properties are added by default.
impl ICalendar {
    pub fn new(version: String, prodid: String) -> Self {
        let mut cal = ICalendar(Component::new(String::from("VCALENDAR")));
        cal.push(Property::new(String::from("VERSION"), version));
        cal.push(Property::new(String::from("PRODID"), prodid));
        cal
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }

    pub fn add_component<C: Into<Component>>(&mut self, component: C) {
        self.0.add_component(component);
    }

    pub fn add_event(&mut self, event: Event) {
        self.add_component(event);
    }

    pub fn add_todo(&mut self, todo: ToDo) {
        self.add_component(todo);
    }

    pub fn add_journal(&mut self, journal: Journal) {
        self.add_component(journal);
    }

    pub fn add_freebusy(&mut self, freebusy: FreeBusy) {
        self.add_component(freebusy);
    }

    pub fn add_timezone(&mut self, timezone: TimeZone) {
        self.add_component(timezone);
    }
}

// TODO: Docs explaining that these properties are added by default.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Event(Component);

impl Event {
    pub fn new(uid: String, dtstart: String) -> Self {
        let mut event = Event(Component::new(String::from("VEVENT")));
        event.push(Property::new(String::from("UID"), uid));
        event.push(Property::new(String::from("DTSTART"), dtstart));
        event
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }

    pub fn add_alarm(&mut self, alarm: Alarm) {
        self.0.add_component(alarm);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ToDo(Component);

impl ToDo {
    pub fn new(uid: String, dtstart: String) -> Self {
        let mut todo = ToDo(Component::new(String::from("VTODO")));
        todo.push(Property::new(String::from("UID"), uid));
        todo.push(Property::new(String::from("DTSTART"), dtstart));
        todo
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }

    pub fn add_alarm(&mut self, alarm: Alarm) {
        self.0.add_component(alarm);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Journal(Component);

impl Journal {
    pub fn new(uid: String, dtstart: String) -> Self {
        let mut journal = Journal(Component::new(String::from("VJOURNAL")));
        journal.push(Property::new(String::from("UID"), uid));
        journal.push(Property::new(String::from("DTSTART"), dtstart));
        journal
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FreeBusy(Component);

impl FreeBusy {
    pub fn new(uid: String, dtstart: String) -> Self {
        let mut freebusy = FreeBusy(Component::new(String::from("VFREEBUSY")));
        freebusy.push(Property::new(String::from("UID"), uid));
        freebusy.push(Property::new(String::from("DTSTART"), dtstart));
        freebusy
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeZone(Component);

impl TimeZone {
    pub fn new() -> Self {
        TimeZone(Component::new(String::from("VTIMEZONE")))
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }

    pub fn add_zone_time(&mut self, zone_time: ZoneTime) {
        self.0.add_component(zone_time);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ZoneTime(Component);

impl ZoneTime {
    pub fn standard(dtstart: String, tz_offset_to: String, tz_offset_from: String) -> Self {
        let mut zone_time = ZoneTime(Component::new(String::from("STANDARD")));
        zone_time.push(Property::new(String::from("DTSTART"), dtstart));
        zone_time.push(Property::new(String::from("TZOFFSETTO"), tz_offset_to));
        zone_time.push(Property::new(String::from("TZOFFSETFROM"), tz_offset_from));
        zone_time
    }

    pub fn daylight(dtstart: String, tz_offset_to: String, tz_offset_from: String) -> Self {
        let mut zone_time = ZoneTime(Component::new(String::from("DAYLIGHT")));
        zone_time.push(Property::new(String::from("DTSTART"), dtstart));
        zone_time.push(Property::new(String::from("TZOFFSETTO"), tz_offset_to));
        zone_time.push(Property::new(String::from("TZOFFSETFROM"), tz_offset_from));
        zone_time
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Alarm(Component);

impl Alarm {
    fn new(kind: String) -> Self {
        let mut comp = Component::new(String::from("VALARM"));
        comp.add_property(Property::new(String::from("ACTION"), kind));
        Alarm(comp)
    }

    pub fn audio(trigger: String) -> Self {
        let mut alarm = Alarm::new(String::from("AUDIO"));
        alarm.push(Property::new(String::from("TRIGGER"), trigger));
        alarm
    }

    pub fn display(trigger: String, description: String) -> Self {
        let mut alarm = Alarm::new(String::from("DISPLAY"));
        alarm.push(Property::new(String::from("TRIGGER"), trigger));
        alarm.push(Property::new(String::from("DESCRIPTION"), description));
        alarm
    }

    pub fn email(trigger: String, description: String, summary: String) -> Self {
        let mut alarm = Alarm::new(String::from("EMAIL"));
        alarm.push(Property::new(String::from("TRIGGER"), trigger));
        alarm.push(Property::new(String::from("DESCRIPTION"), description));
        alarm.push(Property::new(String::from("SUMMARY"), summary));
        alarm
    }

    pub fn push<P: Into<Property>>(&mut self, property: P) {
        self.0.add_property(property);
    }
}

macro_rules! impl_display_comps {
    ($type:ty) => {
        impl fmt::Display for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! impl_component_conversion {
    ($component:ty) => {
        impl From<$component> for Component {
            fn from(component: $component) -> Self {
                component.0
            }
        }
    };
}

impl_display_comps!(Event);
impl_display_comps!(ToDo);
impl_display_comps!(Journal);
impl_display_comps!(FreeBusy);
impl_display_comps!(TimeZone);
impl_display_comps!(ZoneTime);
impl_display_comps!(Alarm);

impl_component_conversion!(Event);
impl_component_conversion!(ToDo);
impl_component_conversion!(Journal);
impl_component_conversion!(FreeBusy);
impl_component_conversion!(TimeZone);
impl_component_conversion!(ZoneTime);
impl_component_conversion!(Alarm);
