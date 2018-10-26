/// Macros to create data types and implement traits.
#[macro_export]
macro_rules! parameters {
    ($($key:expr, $value:expr);*) => {
        {
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
        let para = parameters!("VALUE", "BOOLEAN"; "CUTYPE", "GROUP");
        assert_eq!(b_map, para);
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
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $builder<'a> {
            value: Cow<'a, str>,
            parameters: Parameters<'a>,
        }
        
        impl<'a> $builder<'a> {
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>,
            {
                $builder {
                    value: value.into(),
                    parameters: BTreeMap::new(),
                }
            }
        
            pub fn add<P>(&mut self, parameter: P)
            where
                P: Into<Parameter<'a>>,
            {
                let param = parameter.into();
                self.parameters.insert(param.key, param.value);
            }
        
            pub fn append(&mut self, mut parameter: Parameters<'a>) {
                self.parameters.append(&mut parameter);
            }
        }
        
        impl<'a> From<$builder<'a>> for Property<'a> {
            fn from(builder: $builder<'a>) -> Self {
                Property {
                    key: $name.into(),
                    value: builder.value,
                    parameters: builder.parameters,
                }
            }
        }
    };
}

// Creation and conversion from builder types to Parameter
macro_rules! parameter_builder {
    ($builder:ident, $name:expr) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $builder<'a> {
            value: Cow<'a, str>,
        }
        
        impl<'a> $builder<'a> {
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>,
            {
                $builder {
                    value: value.into(),
                }
            }
        }
        
        impl<'a> From<$builder<'a>> for Parameter<'a> {
            fn from(builder: $builder<'a>) -> Self {
                Parameter {
                    key: $name.into(),
                    value: builder.value,
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
                    parameters: BTreeMap::new(),
                }
            }
        }
    };
    ($builder:ident) => {
        impl<'a> Default for $builder<'a> {
            fn default() -> Self {
                $builder {
                    value: Cow::default(),
                    parameters: BTreeMap::new(),
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
                    value: $default.into(),
                }
            }
        }
    };
    ($builder:ident) => {
        impl<'a> Default for $builder<'a> {
            fn default() -> Self {
                $builder {
                    value: Cow::default(),
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
