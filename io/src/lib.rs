#[cfg(test)]
mod tests;
pub type Result<T = (), E = std::io::Error> = std::result::Result<T, E>;

pub trait Executable {
    fn execute(&self) -> std::io::Result<()>;
}

pub trait CommandIo: Sized {
    fn write(&self, s: &mut String);

    fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self>;
}

impl CommandIo for bool {
    fn write(&self, s: &mut String) {
        s.push_str(&*self.to_string())          
    }

    fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self>{
        Ok(match iter.next().unwrap().as_str() {
            "true" => true,
            "false" => false,
            "0" => false,
            "1" => true,
            _ => {
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid boolean in command!"));
            }
        })
    }
}

impl CommandIo for String {
    fn write(&self, s: &mut String) {
        s.push_str(self.as_str())          
    }

    fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self> {
        match iter.next() {
            Some(s) => Ok(s.as_str().to_string()),
            None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Too little arguments passed!"))
        }
    }
}

impl CommandIo for Option<String> {
    fn write(&self, s: &mut String) {
        match self {
            Some(ss) => {
                s.push_str(ss.as_str())
            }
            _ => ()
        }          
    }

    fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self> {
        match iter.next() {
            Some(s) => Ok(Some(s.as_str().to_string())),
            None => Ok(None)
        }
    }
}

macro_rules! impl_primitive {
    ($ty:ty) => {
        impl CommandIo for $ty {
            fn write(&self, s: &mut String) {
                s.push_str(&*self.to_string())          
            }
        
            fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self> {
                match iter.next() {
                    Some(s) => Ok(s.as_str().parse::<$ty>().unwrap()),
                    None => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Too little arguments passed!"))
                }
            }
        }

        impl CommandIo for Option<$ty> {
            fn write(&self, s: &mut String) {
                match self {
                    Some(ss) => {
                        s.push_str(&*ss.to_string())
                    }
                    _ => ()
                }         
            }
        
            fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self> {
                match iter.next() {
                    Some(s) => Ok(Some(s.as_str().parse::<$ty>().unwrap())),
                    None => Ok(None)
                }
            }
        }
    };
}

impl_primitive!(u8);
impl_primitive!(u16);
impl_primitive!(u32);
impl_primitive!(u64);
impl_primitive!(i8);
impl_primitive!(i16);
impl_primitive!(i32);
impl_primitive!(i64);
impl_primitive!(f32);
impl_primitive!(f64);



#[macro_export]
macro_rules! command_io {
    (struct $ident:ident : $description:literal, $usage:literal {$(
        $vis:vis $field:ident : $ty:ty, $disc:literal,
    )*}) => {
        #[derive(Debug, Clone, Default)]
        pub struct $ident {
            $(
                $vis $field: $ty,
            )*
        }

        #[allow(unused_variables)]
        impl crate::CommandIo for $ident {
            fn write(&self, s: &mut String) {
                $(
                    crate::CommandIo::write(&self.$field, &mut *s);
                )*
            }

            fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self, std::io::Error> {
                Ok(Self {
                    $(
                        $field: crate::CommandIo::read(iter)?,
                    )*
                })
            }
        }

        #[allow(dead_code)]
        impl $ident {

            pub fn description() -> String {
                $description.to_string()
            }

            pub fn usage() -> String {
                $usage.to_string()
            }

            pub fn args_usage() -> String {
                let mut s = String::new();
                $(
                    s.push_str(stringify!($field));
                    s.push_str(&*format!("::<{}>::", stringify!($ty)));
                    s.push_str(&*format!("({})\n", stringify!($disc)));
                )*
                s
            }
        }
    };
    (enum $ident:ident : $disty:ty {$(
        $var:ident = $disc:literal,
    )*}) => {
        #[derive(Debug, Clone)]
        pub enum $ident {
            $(
                $var($var),
            )*
        }

        #[allow(dead_code)]
        impl $ident {
            pub fn execute(&self) -> std::io::Result<()> {
                match self {
                    $(
                        Self::$var(value) => value.execute(),
                    )*
                }
            }

            pub fn help(&self) -> std::io::Result<String> {
                match self {
                    $(
                        Self::$var(_) => {
                            let mut s = format!("Name: {}", $disc);
                            s.push_str(&*format!("\nDescription: {}\n", <$var>::description()));
                            s.push_str(&*format!("Usage: {}\n", <$var>::usage()));
                            s.push_str(&*format!("Arguments:\n{}", <$var>::args_usage()));
                            Ok(s)
                        },
                    )*
                }
            }

            fn get_cmd<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self, std::io::Error> {
                match &*<$disty as crate::CommandIo>::read(iter)? {
                    $(
                        $disc => Ok(Self::$var($var::default())),
                    )*
                    _ =>  Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown command!")),
                }
            }

            fn get_all() -> Vec<Self> {
                vec![
                    $(
                        Self::$var($var::default()),
                    )*
                ]
            }
        }

        impl crate::CommandIo for $ident {
            fn write(&self, s: &mut String) {
                match self {
                    $(
                        Self::$var(value) => {
                            <$disty as crate::CommandIo>::write(&$disc.to_string(), &mut *s);
                            crate::CommandIo::write(value, &mut *s);
                        },
                    )*
                }
            }

            fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self, std::io::Error> {
                match &*<$disty as crate::CommandIo>::read(iter)? {
                    $(
                        $disc => Ok(Self::$var(crate::CommandIo::read(iter)?)),
                    )*
                    _ =>  Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unknown command!")),
                }
            }
        }
    };
}