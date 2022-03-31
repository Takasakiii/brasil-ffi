use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidYear,
    NotHoliday,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidYear => write!(f, "Invalid year, must be between 1900 and 2199"),
            Error::NotHoliday => write!(f, "Not a holiday"),
        }
    }
}

impl std::error::Error for Error {}
