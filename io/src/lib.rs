#[cfg(test)]
mod tests;
use anyhow::anyhow;
pub type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

// Flag used for representing an argument that is just a -v or -l and requires no value
// If you require a flag with a value otherwise use Option<T>
pub struct Flag {
    // found represents if the given argument was found in the command string.
    pub found: bool,
}

pub trait Executable {
    fn execute(&self) -> anyhow::Result<()>;
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
            "true" | "1" => true,
            "false" | "0" => false,
            _ => {
                return Err(anyhow!("Invalid boolean in command!"));
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
            None => Err(anyhow!("Too little arguments passed!"))
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
                    None => Err(anyhow!("Too little arguments passed!"))
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

            fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self, anyhow::Error> {
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
            pub fn execute(&self) -> anyhow::Result<()> {
                match self {
                    $(
                        Self::$var(value) => value.execute(),
                    )*
                }
            }

            pub fn name(&self) -> String {
                match self {
                    $(
                        Self::$var(_) => $disc.to_string(),
                    )*
                }
            }

            pub fn description(&self) -> String {
                match self {
                    $(
                        Self::$var(_) => <$var>::description().to_string(),
                    )*
                }
            }

            pub fn usage(&self) -> String {
                match self {
                    $(
                        Self::$var(_) => <$var>::usage().to_string(),
                    )*
                }
            }

            pub fn help(&self) -> anyhow::Result<String> {
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

            fn get_cmd<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self, anyhow::Error> {
                match &*<$disty as crate::CommandIo>::read(iter)? {
                    $(
                        $disc => Ok(Self::$var($var::default())),
                    )*
                    _ =>  Err(anyhow!("Unknown command!")),
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

            fn read<'a>(iter: &mut impl Iterator<Item = regex::Match<'a>>) -> Result<Self, anyhow::Error> {
                match &*<$disty as crate::CommandIo>::read(iter)? {
                    $(
                        $disc => Ok(Self::$var(crate::CommandIo::read(iter)?)),
                    )*
                    _ =>  Err(anyhow!("Unknown command!")),
                }
            }
        }
    };
}