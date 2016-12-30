use std::error;
use std::fmt;

pub struct State {
}

impl State {
    pub fn new() -> State {
        State {
        }
    }

    pub fn run_line(&mut self, line: String) -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Error {
    msg: String,
}

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
