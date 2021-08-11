pub use crate::contentline::{LineWriter, PropertyWrite};
use crate::properties::{ProdID, Version};
use std::io::{Error, Write};

pub const VCALENDAR: &str = "VCALENDAR";
pub const VEVENT: &str = "VEVENT";
pub const VTODO: &str = "VTODO";
pub const VJOURNAL: &str = "VJOURNAL";
pub const VFREEBUSY: &str = "VFREEBUSY";
pub const VALARM: &str = "VALARM";
pub const VTIMEZONE: &str = "VTIMEZONE";
pub const STANDARD: &str = "STANDARD";
pub const DAYLIGHT: &str = "DAYLIGHT";

#[derive(Debug)]
pub struct ICalendar<'w>(LineWriter<'w>);

impl<'w> ICalendar<'w> {
    pub fn new(
        inner: &'w mut dyn Write,
        version: Version,
        product_id: ProdID
    ) -> Result<Self, Error> {
        let mut writer = LineWriter::new(inner);
        writer.write_begin_unchecked(VCALENDAR)?;
        writer.write_property(&version)?;
        writer.write_property(&product_id)?;
        Ok(Self(writer))
    }
}

impl ICalendar<'_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_component(
        &mut self,
        name: &str,
        body: impl FnOnce(&mut Self) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin(name)?;
        body(self)?;
        self.0.write_end(name)
    }

    pub fn write_event(
        &mut self,
        event: impl FnOnce(&mut EventWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VEVENT)?;
        (event)(&mut EventWriter(&mut self.0))?;
        self.0.write_end_unchecked(VEVENT)
    }

    pub fn write_todo(
        &mut self,
        todo: impl FnOnce(&mut ToDoWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VTODO)?;
        (todo)(&mut ToDoWriter(&mut self.0))?;
        self.0.write_end_unchecked(VTODO)
    }

    pub fn write_journal(
        &mut self,
        journal: impl FnOnce(&mut JournalWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VJOURNAL)?;
        (journal)(&mut JournalWriter(&mut self.0))?;
        self.0.write_end_unchecked(VJOURNAL)
    }

    pub fn write_freebusy(
        &mut self,
        freebusy: impl FnOnce(&mut FreeBusyWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VFREEBUSY)?;
        (freebusy)(&mut FreeBusyWriter(&mut self.0))?;
        self.0.write_end_unchecked(VFREEBUSY)
    }

    pub fn write_timezone(
        &mut self,
        timezone: impl FnOnce(&mut TimeZoneWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VTIMEZONE)?;
        (timezone)(&mut TimeZoneWriter(&mut self.0))?;
        self.0.write_end_unchecked(VTIMEZONE)
    }

    pub fn close(mut self) -> Result<(), Error> {
        self.0.write_end_unchecked(VCALENDAR)
    }
}

#[derive(Debug)]
pub struct EventWriter<'e, 'w>(&'e mut LineWriter<'w>);

impl EventWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_alarm(
        &mut self,
        alarm: impl FnOnce(&mut AlarmWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VALARM)?;
        (alarm)(&mut AlarmWriter(self.0))?;
        self.0.write_end_unchecked(VALARM)
    }
}

#[derive(Debug)]
pub struct ToDoWriter<'t, 'w>(&'t mut LineWriter<'w>);

impl ToDoWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_alarm(
        &mut self,
        alarm: impl FnOnce(&mut AlarmWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(VALARM)?;
        (alarm)(&mut AlarmWriter(self.0))?;
        self.0.write_end_unchecked(VALARM)
    }
}

#[derive(Debug)]
pub struct JournalWriter<'j, 'w>(&'j mut LineWriter<'w>);

impl JournalWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct FreeBusyWriter<'f, 'w>(&'f mut LineWriter<'w>);

impl FreeBusyWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct TimeZoneWriter<'t, 'w>(&'t mut LineWriter<'w>);

impl TimeZoneWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }

    pub fn write_standard(
        &mut self,
        definition: impl FnOnce(&mut StandardWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(STANDARD)?;
        (definition)(&mut StandardWriter(self.0))?;
        self.0.write_end_unchecked(STANDARD)
    }

    pub fn write_daylight(
        &mut self,
        definition: impl FnOnce(&mut DaylightWriter) -> Result<(), Error>
    ) -> Result<(), Error> {
        self.0.write_begin_unchecked(DAYLIGHT)?;
        (definition)(&mut DaylightWriter(self.0))?;
        self.0.write_end_unchecked(DAYLIGHT)
    }
}

#[derive(Debug)]
pub struct AlarmWriter<'a, 'w>(&'a mut LineWriter<'w>);

impl AlarmWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct StandardWriter<'s, 'w>(&'s mut LineWriter<'w>);

impl StandardWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}

#[derive(Debug)]
pub struct DaylightWriter<'d, 'w>(&'d mut LineWriter<'w>);

impl DaylightWriter<'_, '_> {
    pub fn write(&mut self, property: &dyn PropertyWrite) -> Result<(), Error> {
        self.0.write_property(property)
    }
}
