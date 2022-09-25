/// Macro to create several `Parameter`s at once.
#[macro_export]
macro_rules! parameters {
    ($($key:expr => $value:expr);*) => {
        {
            use $crate::parameters::Parameter;
            let mut parameters = Vec::new();
            $(
                parameters.push(Parameter::new($key, $value));
            )*
            parameters
        }
    };
}

#[cfg(test)]
mod test {
    use crate::parameters::Parameter;

    #[test]
    fn parameters() {
        let mut expected = Vec::new();
        expected.push(Parameter::new("VALUE", "BOOLEAN"));
        expected.push(Parameter::new("CUTYPE", "GROUP"));
        let parameters = parameters!("VALUE" => "BOOLEAN"; "CUTYPE" => "GROUP");
        assert_eq!(expected, parameters);
    }
}

// Creation and conversion from builder types to Property
macro_rules! property {
    ($type:ident, $name:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property"]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>,
            parameters: Parameters<'a>,
        }

        impl $type<'_> {
            /// The associated specification name of the property in upper case.
            pub const NAME: &'static str = $name;
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]
            #[doc=$name]
            #[doc = "` Property with the given value."]
            pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
                Self {
                    value: value.into(),
                    parameters: Vec::new(),
                }
            }
        }

        impl_add_parameters!($type);

        impl_property_write!($type, $name);
    };
}

macro_rules! property_text {
    ($type:ident, $name:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property"]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>,
            parameters: Parameters<'a>,
        }

        impl $type<'_> {
            /// The associated specification name of the property in upper case.
            pub const NAME: &'static str = $name;
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]
            #[doc=$name]
            #[doc = "` Property with the given value."]
            pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
                Self {
                    value: value.into(),
                    parameters: Vec::new(),
                }
            }
        }

        impl_add_parameters!($type);

        impl PropertyWrite for $type<'_> {
            fn write(&self, w: &mut LineWriter<'_>) -> Result<(), Error> {
                w.write_name_unchecked(Self::NAME);
                for parameter in &self.parameters {
                    w.write_parameter(&parameter.name, &parameter.value)?;
                }
                w.write_text_value(&self.value)
            }
        }
    };
}

// Creation and conversion from builder types to Property with default value
// types as parameter
// This matters right now only for the newer properties from RFC7986.
#[cfg(feature = "rfc7986")]
macro_rules! property_with_parameter {
    ($type:ident, $name:expr, $value:expr) => {
        #[doc = "`"]#[doc=$name]#[doc = "` Property"]
        ///
        /// Newer properties that have a different value type than `TEXT` have to include the `VALUE` parameter.
        #[doc = "The `VALUE` parameter is set to `"]#[doc=$value]#[doc = "`. Do not add this parameter manually."]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>,
            parameters: Parameters<'a>
        }

        impl $type<'_> {
            /// The associated specification name of the property in upper case.
            pub const NAME: &'static str = $name;
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]#[doc=$name]#[doc = "` Property with the given value."]
            pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
                Self {
                    value: value.into(),
                    parameters: parameters!("VALUE" => $value)
                }
            }
        }

        impl_add_parameters!($type);

        impl_property_write!($type, $name);
    };
}

// Creation and conversion from builder types to Property
macro_rules! property_integer {
    ($type:ident, $name:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Property"]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Integer,
            parameters: Parameters<'a>,
        }

        impl $type<'_> {
            /// The associated specification name of the property in upper case.
            pub const NAME: &'static str = $name;

            #[doc = "Creates a new `"]
            #[doc=$name]
            #[doc = "` Property with the given value."]
            pub const fn new(value: Integer) -> Self {
                Self {
                    value,
                    parameters: Vec::new(),
                }
            }
        }

        impl_add_parameters!($type);

        impl_property_write!($type, $name);
    };
}

// Creation and conversion from builder types to Parameter
macro_rules! parameter {
    ($type:ident, $name:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Parameter"]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a>(Cow<'a, str>);

        impl $type<'_> {
            /// The associated specification name of the parameter in upper case.
            pub const NAME: &'static str = $name;
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]
            #[doc=$name]
            #[doc = "` Parameter with the given value."]
            pub fn new(value: impl Into<Cow<'a, str>>) -> Self {
                Self(value.into())
            }
        }

        impl<'a> From<$type<'a>> for Parameter<'a> {
            fn from(builder: $type<'a>) -> Self {
                Parameter {
                    name: Cow::Borrowed($type::NAME),
                    value: builder.0,
                }
            }
        }
    };
}

macro_rules! impl_add_parameters {
    ($type:ident) => {
        impl<'a> $type<'a> {
            /// Adds a parameter to the property.
            pub fn add(&mut self, parameter: impl Into<Parameter<'a>>) {
                self.parameters.push(parameter.into())
            }

            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the [`parameters!`] macro.
            pub fn append(&mut self, parameters: &mut Parameters<'a>) {
                self.parameters.append(parameters)
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_property_write {
    ($type:ident, $name:expr) => {
        impl PropertyWrite for $type<'_> {
            fn write(&self, w: &mut LineWriter<'_>) -> Result<(), Error> {
                w.write_name_unchecked($name);
                for parameter in &self.parameters {
                    w.write_parameter(&parameter.name, &parameter.value)?;
                }
                w.write_value(&self.value)
            }
        }
    };
}
