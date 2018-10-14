/// The low-level interface of the library.
/// Components for building calendar objects are defined here.
use std::collections::BTreeMap;
use std::fmt;
use std::fmt::Write;
use util::fold_line;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Component {
    pub(crate) name: String,
    pub(crate) properties: BTreeMap<String, Vec<Property>>,
    pub(crate) subcomponents: Vec<Component>,
}

impl Component {
    pub fn new(name: String) -> Self {
        Component {
            name,
            ..Default::default()
        }
    }

    pub fn add_property<P: Into<Property>>(&mut self, property: P) {
        let property = property.into();
        self.properties
            .entry(property.key.clone())
            .or_insert_with(Vec::new)
            .push(property);
    }

    pub fn add_component<C: Into<Component>>(&mut self, component: C) {
        self.subcomponents.push(component.into());
    }
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_crlf!(f, "BEGIN:{})", self.name)?;
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
pub struct Property {
    pub(crate) key: String,
    pub(crate) value: String,
    pub(crate) parameters: Parameters,
}

impl Property {
    pub fn new(key: String, value: String) -> Self {
        Property {
            key,
            value,
            parameters: BTreeMap::new(),
        }
    }

    pub fn add<P: Into<Parameter>>(&mut self, parameter: P) {
        let parameter = parameter.into();
        self.parameters.insert(parameter.key, parameter.value);
    }

    pub fn append(&mut self, mut parameter: Parameters) {
        self.parameters.append(&mut parameter);
    }
}

impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut line = String::with_capacity(150);
        write_crlf!(line, "{})", self.key)?;
        for (key, value) in &self.parameters {
            write!(line, ";{}={}", key, value)?;
        }
        write!(line, ":{}", self.value)?;
        line.shrink_to_fit();
        write_crlf!(f, "{}", fold_line(&line))
    }
}

// TODO: What to do with multiple values?
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Parameter {
    pub(crate) key: String,
    pub(crate) value: String,
}

impl Parameter {
    pub fn new(key: String, value: String) -> Parameter {
        Parameter { key, value }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, ";{}={}", self.key, self.value)
    }
}

pub type Parameters = BTreeMap<String, String>;
