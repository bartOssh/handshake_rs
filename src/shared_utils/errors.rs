use crate::{fmt, Error, LambdaErrorExt};

#[derive(Debug)]
pub struct ErrorStr {
    pub value: String,
}

impl LambdaErrorExt for ErrorStr {
    fn error_type(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for ErrorStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Error for ErrorStr {
    fn description(&self) -> &str {
        &self.value
    }
}
