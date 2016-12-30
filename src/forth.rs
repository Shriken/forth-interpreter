use std::convert;
use std::error;
use std::error::Error;
use std::fmt;
use std::io;

pub struct State {
}

impl State {
    pub fn new() -> State {
        State {
        }
    }

    pub fn run_line(&mut self, line: String) -> Result<(), ForthError> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct ForthError {
    msg: String,
}

impl error::Error for ForthError {
    fn description(&self) -> &str { self.msg.as_ref() }
    fn cause(&self) -> Option<&error::Error> { None }
}

impl fmt::Display for ForthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "{}", self.description()));
        Ok(())
    }
}

impl convert::From<io::Error> for ForthError {
    fn from(err: io::Error) -> ForthError {
        ForthError {
            msg: format!("io error: {}", err),
        }
    }
}
