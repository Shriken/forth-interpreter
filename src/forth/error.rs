use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {}

impl error::Error for Error {
    fn description(&self) -> &str { self.msg.as_ref() }
    fn cause(&self) -> Option<&error::Error> { None }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{}", error::Error::description(self)));
        Ok(())
    }
}

impl<'a> From<&'a str> for Error {
    fn from(msg: &str) -> Error { Error { msg: msg.to_string() } }
}

impl From<String> for Error {
    fn from(msg: String) -> Error { Error { msg: msg } }
}
