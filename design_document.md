# Purpose
The ics crate should provide a meaningful way to create correct iCalender files without being difficult or verbose. At the same time the crate should be easy to integrate into other projects by having few dependencies.

# Description of the Specification
An iCalender file is one calendar component containing other components such as: event, todo, journal, freebusy or timezone. Those components can contain components and/or properties which is a key-value pair that can optionally have parameters which are also a key-value pairs. Some components can contain subcomponents. Properties can reappear depending on the specification.

After 75 bytes a newline and whitespace is inserted (CRLF ) and ',', ';' and '/' need to be escaped properly. Newlines are /n and text containg qoutes need to be quoted.

The sytax for components, properties and parameters are as follows:
- Component:
BEGIN COMPONENT
// properties and subcomponents
END COMPONENT
- Property:
PROPERTY_KEY(;PARAMETER)*:PROPERTY_VALUE
- Parameter:
PARAMETER_KEY=PARAMETER_VALUE

For now ics only supports RFC 5545.

# Structure
The crate contains a "low-level" and "high-level" interface. The basic components in the components module can be used to create new custom iCalender components, properties and parameters. This is much more low-level which most users won't need. The high-level components are specified by the RFC 5545. However, if more control is needed, custom properties and custom components can still be added. The ICalendar component is the calendar object which contains a stream of properties and components. Those components are also just a stream of properties and subcomponents.

## Basic Structure/Hierarchy
ICalendar (VCALENDAR)
|__VERSION
|__PRODID
|__Event (VEVENT)
   |__UID
   |__DTSTAMP
   |__Other Properties
   |__Alarm(s) (VALARM)
      |__AUDIO
      |__DISPLAY
      |__EMAIL
   |__Other Components
|__ToDO (VTODO)
   |__UID
   |__DTSTAMP
   |__Other Properties
   |__Alarm(s) (VALARM)
      |__AUDIO
      |__DISPLAY
      |__EMAIL
   |__Other Components
|__Journal (VJOURNAL)
   |__UID
   |__DTSTAMP
   |__Other Properties
   |__Other Components
|__FreeBusy (VFREEBUSY)
   |__UID
   |__DTSTAMP
   |__Other Properties
   |__Other Components
|__TimeZone (VTIMEZONE)
   |__TZID
   |__ZoneTime(s) (STANDARD | DAYLIGHT)
   |__Other Properties
   |__Other Components
|__Other Properties
|__Other Components

# No Error Checks
Error Checks should be done on top of the library because some require external dependencies which goes against the goal of lightness. Error checking is also twofold. There is on one hand the defined value of a property and the value itself. The CLASS property, for example, takes in a text value and has some defined values (plus custom ones too) like PUBLIC. This means there is an associated type and then a (associated) value range. Could this come to the builder types? Probably but right now the focus is on documentation and testing.

# No Builder Pattern
The builder pattern tries to solve the problem of creating a flexible constructor. In Rust, function overloading does not exist. One option would be to create many constructor methods but that would end in a enormous amount of with_x_y_z methods, another is to create a method or big struct with many Options but that is just as much work and with major version updates those would become even bigger. This is where the builder pattern shines when there are many (optional) configurations. However, it becomes useless when there are few configurations.
ICalendar objects only have properties and components which are very similar in structure and defined. There are some convenience methods but they are not necessary. So, instead of having to call several methods to add properties or components, they can be just added generally. Additionally, there are common parameters which could be added to almost every property which would mean even more methods or parameters.
As a maintainer that is actually doable, however, it is not future-proof design as specifications change and adding or changing methods would be always a breaking change. Most importantly it would mean testing a lot of methods that actually do the same.
The interface should guide the user on how to use the library but not hinder using it.

# Cow vs. String
One problem that the library faces is that everything is text (file format) which means everything could be a String. A String, however, is on the Heap, which means memory can be dynamically allocated but many allocations are performance killer and not every device has 16 GB or needs it. Furthermore, a String is good for mutations which is good since we need to fold and escape characters in content lines.
However, not every text value needs to be escaped, nor do key values ever change. This is where Cow comes into play. We can keep a reference (&str) as long as we want and do not need to clone if there are no mutations. However, we can get an owned value if necessary and that also removes many life time issues in theory. The crate was not tested properly yet which means this assumption might change.

# BTreeMap vs. Vec & HashMap
This is a design decision copied from the vobject crate. The pros are that we get an ordered view and searching is easy. For future implementations that is great. Right now it is sometimes an unnecessary allocation. It makes sense that many parameters are stored as a map since a Parameter has only a key and value. HashMap would be an option if that would not mean that every property and component could not implement the Hash trait which ironically means that those could not be used in a HashMap. Another thing is that not every property appears only once. In a way BTreeMap wins by not offering too much.

# Almost Rich Type System
All types implement eagerly all kinds of Traits. There are types for each defined property and parameter created with macros. However, there is no actual type checking because all values are text. The problem is that some types like BOOLEAN have a different text representation in iCalendar. That needs some work since that conflicts with the goal of having few but meaningful dependencies.

# Regex as Feature
I wanted to have no dependencies but using regex for text escaping is probably faster than using the methods from the standard library. Therefore, I will try to maintain it as a feature and test how much of an impact it has.