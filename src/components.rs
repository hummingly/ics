/// The low-level interface of the library.
/// Components for building calendar objects are defined here.
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write;
use util::{escape_cow, fold_line};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Component<'a> {
    pub(crate) name: String,
    pub(crate) properties: BTreeMap<String, Vec<Property<'a>>>,
    pub(crate) subcomponents: Vec<Component<'a>>,
}

impl<'a> Component<'a> {
    pub fn new(name: &str) -> Self {
        Component {
            name: name.to_owned(),
            ..Default::default()
        }
    }

    pub fn add_property<P>(&mut self, property: P)
    where
        P: Into<Property<'a>>,
    {
        let property = property.into();
        self.properties
            .entry(property.key.clone())
            .or_insert_with(Vec::new)
            .push(property);
    }

    pub fn add_component<C>(&mut self, component: C)
    where
        C: Into<Component<'a>>,
    {
        self.subcomponents.push(component.into());
    }
}

impl<'a> fmt::Display for Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_crlf!(f, "BEGIN:{}", self.name)?;
        for properties in self.properties.values() {
            for property in properties {
                write!(f, "{}", property)?;
            }
        }
        for component in &self.subcomponents {
            write!(f, "{}", component)?;
        }
        write_crlf!(f, "END:{}", self.name)
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Property<'a> {
    pub(crate) key: String,
    pub(crate) value: Cow<'a, str>,
    pub(crate) parameters: Parameters<'a>,
}

impl<'a> Property<'a> {
    pub fn new<S>(key: &str, value: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Property {
            key: key.to_owned(),
            value: escape_cow(value),
            parameters: BTreeMap::new(),
        }
    }

    pub fn add<P>(&mut self, parameter: P)
    where
        P: Into<Parameter<'a>>,
    {
        let parameter = parameter.into();
        self.parameters.insert(parameter.key, parameter.value);
    }

    pub fn append(&mut self, mut parameter: Parameters<'a>) {
        self.parameters.append(&mut parameter);
    }

    fn len(&self) -> usize {
        // + 1 for the : in the property
        let mut len = self.value.len() + self.key.len() + 1;
        for (key, value) in &self.parameters {
            // + 2 for the ; and = in the parameter
            len = len + key.len() + value.len() + 2;
        }
        len
    }
}

impl<'a> fmt::Display for Property<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line = String::with_capacity(self.len());
        write!(line, "{}", self.key)?;
        for (key, value) in &self.parameters {
            write!(line, ";{}={}", key, value)?;
        }
        write!(line, ":{}", self.value)?;
        line.shrink_to_fit();
        write_crlf!(f, "{}", fold_line(line))
    }
}

// TODO: What to do with multiple values?
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Parameter<'a> {
    pub(crate) key: String,
    pub(crate) value: Cow<'a, str>,
}

impl<'a> Parameter<'a> {
    pub fn new<S>(key: &str, value: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Parameter {
            key: key.to_owned(),
            value: escape_cow(value),
        }
    }
}

impl<'a> fmt::Display for Parameter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ";{}={}", self.key, self.value)
    }
}

pub type Parameters<'a> = BTreeMap<String, Cow<'a, str>>;
