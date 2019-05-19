# Changelog

## Version 0.4.0

### Breaking Changes

- Changed `parameters!` input syntax from `,` to `=>` to separate key and value
- Properties aren't ordered anymore but shown in the order they were added. This change was made to reduce the unnecessary complexity to store properties but mainly also because it is very annoying to compare the input code with the generated iCalendar file. The previous implementation would prevent duplicated properties but this won't generate erroneous files.
- `Parameter`s are now formatted without a semicolon: "KEY=VALUE".

### API

- Format defintions (constants) for the following parameters were added:
  - `CUType`
  - `FBType`
  - `RelType`
  - `Role`
  - `Value`
  - `Display`
  - `Feature`
  - `PartStat`

### Misc

- `BTreeMap` was changed to `Vec` in `Property` to remove overhead/complexity

## Version 0.3.2

### Bug Fix:

- `NAME` property was not properly capitalized

## Version 0.3.1

### Documentation:

- Improved method descriptions of `ICalendar struct`

### Misc:

- Simplified fold algorithm

## Version 0.3.0

### API:

- Added convenience method for saving `ICalendar` object in a writer (i.e. files)
- Removed `fast_text` feature

### Documentation:

- Added minimum rustc version to README
- Fixed typos
- Added the `?` operator in examples to make it more idiomatic

### Misc:

- Refactored a lot of macros
- Improved fold algorithm (more robust)
- Changed escape_text to be similar to regex version
- Added keyword `ical`
- Separated `components` module

## Version 0.2.3

### Misc:

- Moved content line methods to components into module
- Replaced is_char_boundary with next_boundary
- Dropping Windows support in Travis due to incredible long build times for the size of this project.

## Version 0.2.2

### Documentation:

- Fixes inconsistency
- Adds note about RFC7986 support

## Version 0.2.1

### Documentation:

- Finishes the documentation on features
- Improves/extends the main example

## Version 0.2.0

### API:

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

### Misc:

- Fixes badges on `README` and documentation

## Version 0.1.0

- Release!
