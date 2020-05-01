/// Macro to create several parameters at once.
///
/// # Example
/// ```
/// # #[macro_use] extern crate ics;
/// use ics::components::Property;
/// use ics::properties::DtStart;
/// use ics::values::{DateTime, Month};
///
/// # fn main() {
/// let mut date = DtStart::utc(DateTime::ymd(2018, Month::September, 6).unwrap());
/// date.append(parameters!("TZID" => "America/New_York"; "VALUE" => "DATE-TIME"));
/// assert_eq!(
///     Property::from(date).to_string(),
///     "DTSTART;TZID=America/New_York;VALUE=DATE-TIME:20180906T000000Z\r\n"
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
    use components::Parameters;
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
    ($(#[$outer:meta])* $type:ident, $name:expr) => {
        #[doc=$name]#[doc = " Property"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Text<'a>,
            parameters: Parameters<'a>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new "]
            #[doc=$name]
            #[doc = " Property with the given value."]
            pub fn new<T>(value: T) -> Self
            where
                T: Into<Text<'a>>
            {
                Self {
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
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl_from_prop!($type, $name);
    };
}

macro_rules! property_with_constructor {
    (
        $(#[$outer:meta])* $type:ident, $name:expr,
        $($(#[$inner:meta])* fn $const_ident:ident() { $value:expr });*
    ) => {
        #[doc=$name]#[doc = " Property"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Text<'a>,
            parameters: Parameters<'a>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new "]
            #[doc=$name]
            #[doc = " Property with the given value."]
            pub fn new<T>(value: T) -> Self
            where
                T: Into<Text<'a>>
            {
                Self {
                    value: value.into(),
                    parameters: BTreeMap::new()
                }
            }

            $(
                $(#[$inner])*
                ///
                #[doc = "Property Value: "]#[doc = $value]
                pub fn $const_ident() -> Self {
                    Self::new($value)
                }
            )*

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
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl_from_prop!($type, $name);
    };
}

// Creation and conversion from builder types to Property with default value
// types as parameter
// This matters right now only for the newer properties from RFC7986.
#[cfg(feature = "rfc7986")]
macro_rules! property_with_parameter {
    ($type:ident, $name:expr, $value:expr) => {
        #[doc=$name]#[doc = " Property\n\n"]
        #[doc = "Newer properties that have a different value type than TEXT have to include the \"VALUE\" parameter. This property already contains \"VALUE:"]
        #[doc=$value]#[doc=", do not add this parameter manually."]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Text<'a>,
            parameters: Parameters<'a>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new "]
            #[doc=$name]
            #[doc = " Property with the given value."]
            pub fn new<T>(value: T) -> Self
            where
                T: Into<Text<'a>>
            {
                Self {
                    value: value.into(),
                    parameters: parameters!("VALUE" => $value)
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
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl_from_prop!($type, $name);
    };
}

// Creation and conversion from builder types to Parameter
macro_rules! parameter {
    ($(#[$outer:meta])* $type:ident, $name:expr) => {
        #[doc=$name]#[doc = " Parameter"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new "]
            #[doc=$name]
            #[doc = " Parameter with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                Self {
                    value: value.into()
                }
            }
        }

        impl_from_param!($type, $name);
    };
}

macro_rules! parameter_with_const {
    (
        $(#[$outer:meta])* $type:ident, $name:expr,
        $($(#[$inner:meta])* const $const_ident:ident = $value:expr);*
    ) => {
        #[doc=$name]#[doc = " Parameter"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Cow<'a, str>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new "]
            #[doc=$name]
            #[doc = " Parameter with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                Self {
                    value: value.into()
                }
            }

            $(
                $(#[$inner])*
                ///
                #[doc = "Parameter Value: "]#[doc = $value]
                pub const $const_ident: Self = Self {
                    value: Cow::Borrowed($value)
                };
            )*
        }

        impl_from_param!($type, $name);
    };
}

macro_rules! impl_from_prop {
    ($type:ident, $name:expr) => {
        impl<'a> From<$type<'a>> for Property<'a> {
            fn from(builder: $type<'a>) -> Self {
                Property {
                    key: $name.into(),
                    value: builder.value.to_string().into(),
                    parameters: builder.parameters
                }
            }
        }
    };
}

macro_rules! impl_from_param {
    ($type:ident, $name:expr) => {
        impl<'a> From<$type<'a>> for Parameter<'a> {
            fn from(builder: $type<'a>) -> Self {
                Parameter {
                    key: $name.into(),
                    value: builder.value
                }
            }
        }
    };
}
