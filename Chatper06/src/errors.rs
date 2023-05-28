use std::fmt;
use std::io;
use std::num;

#[derive(Debug)]
pub struct StatsError {
    pub message: String,
}

impl fmt::Display for StatsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<&str> for StatsError {
    fn from(s: &str) -> Self {
        Self {
            message: s.to_string(),
        }
    }
}

impl From<io::Error> for StatsError {
    fn from(e: io::Error) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}
impl From<num::TryFromIntError> for StatsError {
    fn from(_e: num::TryFromIntError) -> Self {
        Self {
            message: "Number conversion error".to_string(),
        }
    }
}
