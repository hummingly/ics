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
use contentline;
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write;

/// A `Component` contains properties and sometimes sub-components.
///
/// This can be used to create a new calendar component by either creating a
/// wrapper type or just use it as it is.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
            ..Default::default()
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

impl<'a> fmt::Display for Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_crlf!(f, "BEGIN:{}", self.name)?;
        for property in &self.properties {
            write!(f, "{}", property)?;
        }
        for component in &self.subcomponents {
            write!(f, "{}", component)?;
        }
        write_crlf!(f, "END:{}", self.name)
    }
}

/// A `Property` contains a key-value pair which can have optionally several
/// parameters.
///
/// They are part of a component and define it. This can be used to create a
/// new calendar property by either creating a wrapper type or just use it as
/// it is.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
            ..Default::default()
        }
    }

    /// Adds a parameter to a property.
    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>
    {
        let parameter = parameter.into();
        self.parameters.insert(parameter.key, parameter.value);
    }

    /// Adds several parameters at once to a property. For creating several
    /// parameters at once, consult the documentation of the `parameters!`
    /// macro.
    pub fn append(&mut self, mut parameters: Parameters<'a>) {
        self.parameters.append(&mut parameters);
    }

    fn len(&self) -> usize {
        // + 1 for the : in the property
        // + 2 for the ; and = in the parameter
        self.parameters
            .iter()
            .fold(self.value.len() + self.key.len() + 1, |len, (k, v)| {
                len + k.len() + v.len() + 2
            })
    }

    fn write_content<W: Write>(&self, writer: &mut W) -> fmt::Result {
        write!(writer, "{}", self.key)?;
        for (key, value) in &self.parameters {
            write!(writer, ";{}={}", key, value)?;
        }
        write!(writer, ":{}", self.value)
    }
}

impl<'a> fmt::Display for Property<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let len = self.len();
        if len <= contentline::LIMIT {
            self.write_content(f)?;
            write_crlf!(f)
        } else {
            let mut line = String::with_capacity(contentline::size(len));
            self.write_content(&mut line)?;
            contentline::fold(&mut line);
            write_crlf!(f, "{}", line)
        }
    }
}

/// A `Parameter` is a key-value that can be added to a property to specify it
/// more.
///
/// This can be used to create a new calendar parameter by either creating a
/// wrapper type or just use it as it is.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}={}", self.key, self.value)
    }
}

/// `Parameters` is a collection of `Parameter`s. It can be created with the
/// `parameters!` macro.
pub type Parameters<'a> = BTreeMap<Cow<'a, str>, Cow<'a, str>>;

#[cfg(test)]
mod tests {
    use super::{Parameter, Property};

    #[test]
    fn simple() {
        let property = Property::new("SUMMARY", "Simple");
        let expected = 14;
        assert_eq!(property.len(), expected);
    }

    #[test]
    fn with_parameter() {
        let mut property = Property::new("SUMMARY", "Simple");
        property.add(Parameter::new("VALUE", "TEXT"));
        let expected = 25;
        assert_eq!(property.len(), expected);
    }
}
