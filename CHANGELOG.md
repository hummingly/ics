# Changelog

## Version 0.5.1

### Documentation

- Fix dependency version in README

## Version 0.5.0

### Breaking Changes

- Removed `ZoneTime` struct
- Removed `TimeZone::new()` and `TimeZone::add_zone_time()`

### API

- Added `Standard` and `Daylight` component
- Replaced `ZoneTime` with `Standard` and `Daylight` in `TimeZone` API
  - `TimeZone::new()` -> `TimeZone::standard()` and `TimeZone::daylight()`
  - `TimeZone::add_zonetime()` -> `TimeZone::add_standard()` and `TimeZone::add_daylight()`

### Documentation

- Some linking and nicer formatting!

## Version 0.4.4

### Bug Fix

- A single carriage return character (the old macOS new line character) was not properly converted to a new line character (line feed).

### Misc

- Improved memory consumption and reduced some iterations in escape_text
- Flattened source folder structure

## Version 0.4.3 (yanked)

## Version 0.4.2

### Misc

- Internal changes only.

## Version 0.4.1

### Misc

- Package crate only contains needed source files and license.

## Version 0.4.0

### Breaking Changes

- Changed `parameters!` input syntax from `,` to `=>` to separate key and value
- Properties are now written in the order they were added.
- `Parameter`s are now formatted without a semicolon: `KEY=VALUE`.
- Derived `Default` implementations were removed from properties, parameters and components because the key or value would be empty which makes sematically no sense.

### API

- Format definitions (constructors) for the following properties were added:
  - `Class`
  - `Status`
  - `Transp`
  - `Action`
- Format definitions (constants) for the following parameters were added:
  - `CUType`
  - `FBType`
  - `RelType`
  - `Role`
  - `Value`
  - `Display`
  - `Feature`
  - `PartStat`

### Misc

- `BTreeMap` was changed to `Vec` in `Property` to remove unnecessary overhead/complexity. Additionally, it did not actually prevent duplicates due to mutliple content lines with the same property being allowed.
- Tests are now only on linux. The crate does only use the std library, so it does not require platform specific features.

## Version 0.3.2

### Bug Fix

- `NAME` property was not properly capitalized

## Version 0.3.1

### Documentation

- Improved method descriptions of `ICalendar struct`

### Misc

- Simplified fold algorithm

## Version 0.3.0

### API

- Added convenience method for saving `ICalendar` object in a writer (i.e. files)
- Removed `fast_text` feature

### Documentation

- Added minimum rustc version to README
- Fixed typos
- Added the `?` operator in examples to make it more idiomatic

### Misc

- Refactored a lot of macros
- Improved fold algorithm (more robust)
- Changed escape_text to be similar to regex version
- Added keyword `ical`
- Separated `components` module

## Version 0.2.3

### Misc

- Moved content line methods to components into module
- Replaced is_char_boundary with next_boundary
- Dropping Windows support in Travis due to incredible long build times for the size of this project.

## Version 0.2.2

### Documentation

- Fixes inconsistency
- Adds note about RFC7986 support

## Version 0.2.1

### Documentation

- Finishes the documentation on features
- Improves/extends the main example

## Version 0.2.0

### API

- Removes the `Into<Cow<str>>` implementation from Parameter enums
- Properties and Parameters as defined in [RFC7986](https://tools.ietf.org/html/rfc7986) were added under a feature flag `rfc7986` which is enabled by default.
  #### New Properties!
  - `NAME`
  - `REFRESH-INTERVAL`
  - `SOURCE`
  - `COLOR`
  - `IMAGE`
  - `CONFERENCE`
  #### New Parameters!
  - `DISPLAY`
  - `EMAIL`
  - `FEATURE`
  - `LABEL`

## Version 0.1.1

### Misc

- Fixes badges on `README` and documentation

## Version 0.1.0

- Release!
