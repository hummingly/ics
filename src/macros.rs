/// Macro to create several parameters at once.
///
/// # Example
/// ```
/// # #[macro_use] extern crate ics;
/// use ics::components::Property;
/// use ics::properties::DtStart;
///
/// # fn main() {
/// let mut date = DtStart::new("20180906");
/// date.append(parameters!("TZID", "America/New_York"; "VALUE", "DATE"));
/// assert_eq!(
///     Property::from(date).to_string(),
///     "DTSTART;TZID=America/New_York;VALUE=DATE:20180906\r\n"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! parameters {
    ($($key:expr, $value:expr);*) => {
        {
            use std::collections::BTreeMap;
            use $crate::components::Parameters;
            let mut parameters: Parameters = BTreeMap::new();
            $(
                parameters.insert($key.into(), $value.into());
            )*
            parameters
        }
    };
}

#[cfg(test)]
mod test {
    use components::Parameters;
    use std::collections::BTreeMap;

    #[test]
    fn parameters_btreemap() {
        let mut b_map: Parameters = BTreeMap::new();
        b_map.insert("VALUE".into(), "BOOLEAN".into());
        b_map.insert("CUTYPE".into(), "GROUP".into());
        let param = parameters!("VALUE", "BOOLEAN"; "CUTYPE", "GROUP");
        assert_eq!(b_map, param);
    }
}

macro_rules! write_crlf {
    ($dst:expr) => (
        write!($dst, "\r\n")
    );
    ($dst:expr, $fmt:expr) => (
        write!($dst, concat!($fmt, "\r\n"))
    );
    ($dst:expr, $fmt:expr, $($arg:tt)*) => (
        write!($dst, concat!($fmt, "\r\n"), $($arg)*)
    );
}

macro_rules! property_builder {
    ($builder:ident, $name:expr) => {
        #[allow(missing_docs)]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $builder<'a> {
            value: Cow<'a, str>,
            parameters: Parameters<'a>
        }
        
        impl<'a> $builder<'a> {
            /// Creates the property with the given value.
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                $builder {
                    value: value.into(),
                    parameters: BTreeMap::new()
                }
            }
        
            /// Adds a parameter to the property.
            pub fn add<P>(&mut self, parameter: P)
            where
                P: Into<Parameter<'a>>
            {
                let param = parameter.into();
                self.parameters.insert(param.key, param.value);
            }
        
            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the `parameters!` macro.
            pub fn append(&mut self, mut parameter: Parameters<'a>) {
                self.parameters.append(&mut parameter);
            }
        }
        
        impl<'a> From<$builder<'a>> for Property<'a> {
            fn from(builder: $builder<'a>) -> Self {
                Property {
                    key: $name.into(),
                    value: builder.value,
                    parameters: builder.parameters
                }
            }
        }
    };
}

// Creation and conversion from builder types to Parameter
macro_rules! parameter_builder {
    ($builder:ident, $name:expr) => {
        #[allow(missing_docs)]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $builder<'a> {
            value: Cow<'a, str>
        }
        
        impl<'a> $builder<'a> {
            /// Creates the parameter with the given value.
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                $builder {
                    value: value.into()
                }
            }
        }
        
        impl<'a> From<$builder<'a>> for Parameter<'a> {
            fn from(builder: $builder<'a>) -> Self {
                Parameter {
                    key: $name.into(),
                    value: builder.value
                }
            }
        }
    };
}

// Some properties/parameters have default values.
// The default value is implemented for the builder types!
macro_rules! impl_default_property {
    ($builder:ident, $default:expr) => {
        impl<'a> Default for $builder<'a> {
            fn default() -> Self {
                $builder {
                    value: $default.into(),
                    parameters: BTreeMap::new()
                }
            }
        }
    };
    ($builder:ident) => {
        impl<'a> Default for $builder<'a> {
            fn default() -> Self {
                $builder {
                    value: Cow::default(),
                    parameters: BTreeMap::new()
                }
            }
        }
    };
}

macro_rules! impl_default_parameter {
    ($builder:ident, $default:expr) => {
        impl<'a> Default for $builder<'a> {
            fn default() -> Self {
                $builder {
                    value: $default.into()
                }
            }
        }
    };
    ($builder:ident) => {
        impl<'a> Default for $builder<'a> {
            fn default() -> Self {
                $builder {
                    value: Cow::default()
                }
            }
        }
    };
}

macro_rules! impl_display_comps {
    ($type:ident) => {
        impl<'a> fmt::Display for $type<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! impl_component_conversion {
    ($component:ident) => {
        impl<'a> From<$component<'a>> for Component<'a> {
            fn from(component: $component<'a>) -> Self {
                component.0
            }
        }
    };
}
