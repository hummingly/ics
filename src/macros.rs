/// Macro to create several `Parameter`s at once.
///
/// # Example
/// ```
/// use ics::parameters;
/// use ics::components::Property;
/// use ics::properties::DtStart;
///
/// # fn main() {
/// let mut date = DtStart::new("20180906");
/// date.append(parameters!("TZID" => "America/New_York"; "VALUE" => "DATE"));
/// assert_eq!(
///     Property::from(date).to_string(),
///     "DTSTART;TZID=America/New_York;VALUE=DATE:20180906\r\n"
/// );
/// # }
/// ```
#[macro_export]
macro_rules! parameters {
    ($($key:expr => $value:expr);*) => {
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
    use crate::components::Parameters;
    use std::collections::BTreeMap;

    #[test]
    fn parameters() {
        let mut b_map: Parameters = BTreeMap::new();
        b_map.insert("VALUE".into(), "BOOLEAN".into());
        b_map.insert("CUTYPE".into(), "GROUP".into());
        let param = parameters!("VALUE" => "BOOLEAN"; "CUTYPE" => "GROUP");
        assert_eq!(b_map, param);
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

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]
            #[doc=$name]
            #[doc = "` Property with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>,
            {
                Self {
                    value: value.into(),
                    parameters: BTreeMap::new(),
                }
            }
        }

        impl_add_parameters!($type);

        impl_from_prop!($type, $name);
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
        #[doc = "This property already contains `VALUE:"]#[doc=$value]#[doc="`. Do not add this parameter manually."]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>,
            parameters: Parameters<'a>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]#[doc=$name]#[doc = "` Property with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                Self {
                    value: value.into(),
                    parameters: parameters!("VALUE" => $value)
                }
            }
        }

        impl_add_parameters!($type);

        impl_from_prop!($type, $name);
    };
}

// Creation and conversion from builder types to Parameter
macro_rules! parameter {
    ($type:ident, $name:expr) => {
        #[doc = "`"]
        #[doc=$name]
        #[doc = "` Parameter"]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>,
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]
            #[doc=$name]
            #[doc = "` Parameter with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>,
            {
                Self {
                    value: value.into(),
                }
            }
        }

        impl<'a> From<$type<'a>> for Parameter<'a> {
            fn from(builder: $type<'a>) -> Self {
                Parameter {
                    key: Cow::Borrowed($name),
                    value: builder.value,
                }
            }
        }
    };
}

macro_rules! impl_add_parameters {
    ($type:ident) => {
        impl<'a> $type<'a> {
            /// Adds a parameter to the property.
            pub fn add<P>(&mut self, parameter: P)
            where
                P: Into<Parameter<'a>>,
            {
                let parameter = parameter.into();
                self.parameters.insert(parameter.key, parameter.value);
            }

            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the [`parameters!`] macro.
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }
    };
}

macro_rules! impl_from_prop {
    ($type:ident, $name:expr) => {
        impl<'a> From<$type<'a>> for Property<'a> {
            fn from(builder: $type<'a>) -> Self {
                Property {
                    key: Cow::Borrowed($name),
                    value: builder.value,
                    parameters: builder.parameters,
                }
            }
        }
    };
}
