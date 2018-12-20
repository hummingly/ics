# Version 0.2.3
- Refactoring
  - moved content line methods to components into module
  - replaced is_char_boundary with next_boundary
- Dropping Windows support in Travis due to incredible long build times for the size of this project.

# Version 0.2.2
- Fixes inconsistency in the documentation.
- Add note about RFC7986 support in the documentation.

# Version 0.2.1
- Finishes the documentation on features.
- Improves/extends the main example in the documentation.

# Version 0.2.0
- Removes the Into<Cow<str>> implementation from Parameter enums.
- Properties and Parameter as defined in [RFC7986](https://tools.ietf.org/html/rfc7986) were added under a feature flag `rfc7986` which is enabled by default.
  ### New Properties
  - `NAME` Property
  - `REFRESH-INTERVAL` Property
  - `SOURCE` Property
  - `COLOR` Property
  - `IMAGE` Property
  - `CONFERENCE` Property
  ### New Parameters
  - `DISPLAY` Property Parameter
  - `EMAIL` Property Parameter
  - `FEATURE` Property Parameter
  - `LABEL` Property Parameter

# Version 0.1.1
- Fixes badges on `README` and documentation.

# Version 0.1.0
- Release!