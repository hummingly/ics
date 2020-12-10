//! Basic components for building custom calendar objects.
//!
//! To create new custom (IANA/non-standard) calendar
//! components/properties/parameters, the [Display](https://doc.rust-lang.org/std/fmt/trait.Display.html) implementation of the base
//! components should be used to avoid conflicting formatting.
//!
//! # Example
//! ```
//! // Implementing Display for new component
//! use ics::components::Component;
//! use std::fmt;
//!
//! pub struct MyCustomComponent<'a>(Component<'a>);
//!
//! impl<'a> fmt::Display for MyCustomComponent<'a> {
//!     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//!         write!(f, "{}", self.0)
//!     }
//! }
//! ```
use std::borrow::Cow;
use std::fmt;

/// A `Component` contains properties and sometimes sub-components.
///
/// This can be used to create a new calendar component by either creating a
/// wrapper type or just use it as it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Component<'a> {
    pub(crate) name: Cow<'a, str>,
    pub(crate) properties: Vec<Property<'a>>,
    pub(crate) subcomponents: Vec<Component<'a>>
}

impl<'a> Component<'a> {
    /// Creates a new component with the given name.
    pub fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>
    {
        Component {
            name: name.into(),
            properties: Vec::new(),
            subcomponents: Vec::new()
        }
    }

    /// Adds a property to a component. Some properties can be added multiple
    /// times. Each occurrence will be shown as single content line.
    pub fn add_property<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>
    {
        self.properties.push(property.into());
    }

    /// Adds a sub-component to this component.
    pub fn add_component<C>(&mut self, component: C)
    where
        C: Into<Component<'a>>
    {
        self.subcomponents.push(component.into());
    }
}

impl fmt::Display for Component<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "BEGIN:{}\r", self.name)?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for component in &self.subcomponents {
            write!(f, "{}", component)?;
        }
        writeln!(f, "END:{}\r", self.name)
    }
}

/// A `Property` contains a key-value pair which can have optionally several
/// parameters.
///
/// They are part of a component and define it. This can be used to create a
/// new calendar property by either creating a wrapper type or just use it as
/// it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Property<'a> {
    pub(crate) key: Cow<'a, str>,
    pub(crate) value: Cow<'a, str>,
    pub(crate) parameters: Parameters<'a>
}

impl<'a> Property<'a> {
    /// Creates a new property with the given key and value.
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>
    {
        Property {
            key: key.into(),
            value: value.into(),
            parameters: Vec::new()
        }
    }

    /// Adds a parameter to a property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let parameter = parameter.into();
        match self.parameters.iter_mut().find(|p| p.key == parameter.key) {
            Some(p) => *p = parameter,
            None => self.parameters.push(parameter)
        }
    }

    /// Adds several parameters at once to a property. For creating several
    /// parameters at once, consult the documentation of the `parameters!`
    /// macro.
    pub fn append(&mut self, parameters: &mut Parameters<'a>) {
        for parameter in parameters.drain(..) {
            self.add(parameter);
        }
    }

    fn content_len(&self) -> usize {
        // + 1 for the : in the property
        // + 2 for the ; and = in the parameter
        self.parameters.iter().fold(
            self.value.len() + self.key.len() + 1,
            |len, Parameter { key, value }| len + key.len() + value.len() + 2
        )
    }

    fn format<W: fmt::Write>(&self, writer: &mut W) -> fmt::Result {
        write!(writer, "{}", self.key)?;
        for parameter in &self.parameters {
            write!(writer, ";{}", parameter)?;
        }
        write!(writer, ":{}", self.value)
    }
}

impl fmt::Display for Property<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.content_len();
        if len <= contentline::LIMIT {
            self.format(f)?;
        } else {
            let mut content = String::with_capacity(contentline::size(len));
            self.format(&mut content)?;
            contentline::fold(f, &content)?;
        }
        writeln!(f, "\r")
    }
}

/// A `Parameter` is a key-value that can be added to a property to specify it
/// more.
///
/// This can be used to create a new calendar parameter by either creating a
/// wrapper type or just use it as it is.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Parameter<'a> {
    pub(crate) key: Cow<'a, str>,
    pub(crate) value: Cow<'a, str>
}

impl<'a> Parameter<'a> {
    /// Creates a new property with the given key and value.
    pub fn new<K, V>(key: K, value: V) -> Self
    where
        K: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>
    {
        Parameter {
            key: key.into(),
            value: value.into()
        }
    }
}

impl fmt::Display for Parameter<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

/// `Parameters` is a collection of `Parameter`s. It can be created with the
/// `parameters!` macro.
pub type Parameters<'p> = Vec<Parameter<'p>>;

#[cfg(test)]
mod tests {
    use super::{Parameter, Property};

    #[test]
    fn simple() {
        let property = Property::new("SUMMARY", "Simple");
        let expected = 14;
        assert_eq!(property.content_len(), expected);
    }

    #[test]
    fn with_parameter() {
        let mut property = Property::new("SUMMARY", "Simple");
        property.add(Parameter::new("VALUE", "TEXT"));
        let expected = 25;
        assert_eq!(property.content_len(), expected);
    }
}

mod contentline {
    //! Algorithms for content lines.
    use std::fmt;

    // Content lines must be folded after around 75 bytes by inserting a carriage
    // return and line feed followed by whitespace. This crate uses a space
    // character as white space but it could also be a horizontal tab.
    pub const LIMIT: usize = 75;
    const LINE_BREAK: &str = "\r\n ";

    pub fn fold<W: fmt::Write>(writer: &mut W, mut content: &str) -> fmt::Result {
        let mut boundary = next_boundary(&content);
        writer.write_str(&content[..boundary])?;

        while boundary < content.len() {
            content = &content[boundary..];
            writer.write_str(LINE_BREAK)?;
            let next_boundary = next_boundary(&content);
            writer.write_str(&content[..next_boundary])?;
            boundary = next_boundary;
        }
        Ok(())
    }

    // TODO: unfold algorithm

    fn next_boundary(input: &str) -> usize {
        if LIMIT >= input.len() {
            return input.len();
        }
        match input[..=LIMIT].bytes().rposition(|i| i < 128 || i >= 192) {
            Some(0) | None => input.len(),
            Some(boundary) => boundary
        }
    }

    // Calculates the new estimated text length after inserting line breaks
    pub fn size(len: usize) -> usize {
        if len % LIMIT == 0 {
            len + (len / LIMIT - 1) * 3
        } else {
            len + (len / LIMIT) * 3
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{fold, size};

        #[test]
        fn no_linebreak() {
            let content = "No line break today.";
            let mut line = String::with_capacity(size(content.len()));
            fold(&mut line, content).unwrap();

            assert_eq!(line, content);
        }

        #[test]
        fn over_limit() {
            let content = "Content lines that have a fixed length over 75 bytes should be line folded with CRLF and whitespace.";
            let mut line = String::with_capacity(size(content.len()));
            fold(&mut line, content).unwrap();
            let expected = "Content lines that have a fixed length over 75 bytes should be line folded \r\n with CRLF and whitespace.";

            assert_eq!(line, expected);
        }

        #[test]
        fn multibytes() {
            let content =
                "Content lines shouldn't be folded in the middle of a UTF-8 character! 老虎.";
            let mut line = String::with_capacity(size(content.len()));
            fold(&mut line, content).unwrap();
            let expected =
                "Content lines shouldn't be folded in the middle of a UTF-8 character! 老\r\n 虎.";

            assert_eq!(line, expected);
        }

        #[test]
        fn multi_lines() {
            let content = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";
            let mut line = String::with_capacity(size(content.len()));
            fold(&mut line, content).unwrap();
            let expected = "The quick brown fox jumps over the lazy dog. The quick brown fox jumps over\r\n  the lazy dog. The quick brown fox jumps over the lazy dog. The quick brown\r\n  fox jumps over the lazy dog. The quick brown fox jumps over the lazy dog. ";

            assert_eq!(line, expected);
        }
    }
}
