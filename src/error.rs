use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    ModuleNotFound,
    InvalidAttribute,
    MissingAttribute,
    X11NotSupported,
    InvalidValueForAttribute(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ModuleNotFound => write!(f, "Module not found"),
            Error::InvalidAttribute => write!(f, "Invalid attribute"),
            Error::MissingAttribute => write!(f, "Missing attribute"),
            Error::X11NotSupported => write!(f, "X11 not supported for shell layer windows"),
            Error::InvalidValueForAttribute(attribute) => {
                write!(f, "Invalid value for attribute {attribute}")
            }
        }
    }
}
