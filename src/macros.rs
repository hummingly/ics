/// Macros to create data types and implement traits.
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
        pub struct $builder {
            value: String,
            parameters: Parameters,
        }
        
        impl $builder {
            pub fn new(value: String) -> Self {
                $builder {
                    value,
                    parameters: BTreeMap::new(),
                }
            }
        
            pub fn add<P: Into<Parameter>>(&mut self, parameter: P) {
                let param = parameter.into();
                self.parameters.insert(param.key, param.value);
            }
        
            pub fn append(&mut self, mut parameter: Parameters) {
                self.parameters.append(&mut parameter);
            }
        }
        
        impl From<$builder> for Property {
            fn from(builder: $builder) -> Self {
                Property {
                    key: $name,
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
        pub struct $builder {
            value: String,
        }
        
        impl $builder {
            pub fn new(value: String) -> Self {
                $builder { value }
            }
        }
        
        impl From<$builder> for Parameter {
            fn from(builder: $builder) -> Self {
                Parameter {
                    key: $name,
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
        impl Default for $builder {
            fn default() -> Self {
                $builder {
                    value: $default,
                    parameters: BTreeMap::new(),
                }
            }
        }
    };
    ($builder:ident) => {
        impl Default for $builder {
            fn default() -> Self {
                $builder {
                    value: String::new(),
                    parameters: BTreeMap::new(),
                }
            }
        }
    };
}

macro_rules! impl_default_parameter {
    ($builder:ident, $default:expr) => {
        impl Default for $builder {
            fn default() -> Self {
                $builder { value: $default }
            }
        }
    };
    ($builder:ident) => {
        impl Default for $builder {
            fn default() -> Self {
                $builder {
                    value: String::new(),
                }
            }
        }
    };
}

// TODO: parameters!
