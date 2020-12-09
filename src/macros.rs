/// Macro to create several `Parameter`s at once.
///
/// # Example
/// ```
/// # #[macro_use] extern crate ics;
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
            use $crate::components::Parameter;
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
    use crate::components::Parameter;

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
    ($(#[$outer:meta])* $type:ident, $name:expr) => {
        #[doc = "`"]#[doc=$name]#[doc = "` Property"]
        ///
        $(#[$outer])*
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
                    parameters: Vec::new()
                }
            }

            /// Adds a parameter to the property.
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

            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the [`parameters!`] macro.
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl_from_prop!($type, $name);

        impl_property_write!($type, $name);
    };
}

macro_rules! property_with_constructor {
    (
        $(#[$outer:meta])* $type:ident, $name:expr,
        $($(#[$inner:meta])* fn $const_ident:ident() { $value:expr });*
    ) => {
        #[doc = "`"]#[doc=$name]#[doc = "` Property"]
        ///
        $(#[$outer])*
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
                    parameters: Vec::new()
                }
            }

            $(
                $(#[$inner])*
                ///
                #[doc = "Property Value: "]#[doc = $value]
                pub const fn $const_ident() -> Self {
                    Self {
                        value: Cow::Borrowed($value),
                        parameters: Vec::new()
                    }
                }
            )*

            /// Adds a parameter to the property.
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

            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the [`parameters!`] macro.
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl_from_prop!($type, $name);

        impl_property_write!($type, $name);
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

            /// Adds a parameter to the property.
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

            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the [`parameters!`] macro.
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl_from_prop!($type, $name);

        impl_property_write!($type, $name);
    };
}

// Creation and conversion from builder types to Property
#[allow(unused_macros)]
macro_rules! property_integer {
    ($(#[$outer:meta])* $type:ident, $name:expr) => {
        #[doc = "`"]#[doc=$name]#[doc = "` Property"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a> {
            value: Integer,
            parameters: Parameters<'a>
        }

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]#[doc=$name]#[doc = "` Property with the given value."]
            pub const fn new(value: Integer) -> Self {
                Self {
                    value,
                    parameters: Vec::new()
                }
            }

            /// Adds a parameter to the property.
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

            /// Adds several parameters at once to the property. For creating
            /// several parameters at once, consult the documentation of
            /// the [`parameters!`] macro.
            pub fn append(&mut self, mut parameters: Parameters<'a>) {
                self.parameters.append(&mut parameters);
            }
        }

        impl<'a> From<$type<'a>> for Property<'a> {
            fn from(builder: $type<'a>) -> Self {
                Property {
                    key: Cow::Borrowed($name),
                    value: Cow::Owned(builder.value.to_string()),
                    parameters: builder.parameters
                }
            }
        }

        impl_property_write!($type, $name);
    };
}

// Creation and conversion from builder types to Parameter
macro_rules! parameter {
    ($(#[$outer:meta])* $type:ident, $name:expr) => {
        #[doc = "`"]#[doc=$name]#[doc = "` Parameter"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a>(Cow<'a, str>);

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]#[doc=$name]#[doc = "` Parameter with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                Self(value.into())
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
        #[doc = "`"]#[doc=$name]#[doc = "` Parameter"]
        ///
        $(#[$outer])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $type<'a>(Cow<'a, str>);

        impl<'a> $type<'a> {
            #[doc = "Creates a new `"]#[doc=$name]#[doc = "` Parameter with the given value."]
            pub fn new<S>(value: S) -> Self
            where
                S: Into<Cow<'a, str>>
            {
                Self(value.into())
            }

            $(
                $(#[$inner])*
                ///
                #[doc = "Parameter Value: "]#[doc = $value]
                pub const $const_ident: Self = Self(Cow::Borrowed($value));
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
                    key: Cow::Borrowed($name),
                    value: builder.value,
                    parameters: builder.parameters
                }
            }
        }
    };
}

#[allow(unused_macros)]
macro_rules! impl_property_write {
    ($type:ident, $name:expr) => {
        impl PropertyWrite for $type<'_> {
            fn write<W: io::Write>(&self, line: &mut ContentLine<'_, W>) -> Result<(), io::Error> {
                line.write_name_unchecked($name);
                for parameter in &self.parameters {
                    line.write_parameter_pair(&parameter.key, &parameter.value)?;
                }
                line.write_value(&self.value)
            }
        }
    };
}

macro_rules! impl_from_param {
    ($type:ident, $name:expr) => {
        impl<'a> From<$type<'a>> for Parameter<'a> {
            fn from(builder: $type<'a>) -> Self {
                Parameter {
                    key: Cow::Borrowed($name),
                    value: builder.0
                }
            }
        }
    };
}
