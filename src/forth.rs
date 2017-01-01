use std::error;
use std::fmt;

use token::Token;
use token::tokenize;

pub type Number = i32;

pub struct State {
}

impl State {
    pub fn new() -> State {
        State {
        }
    }

    pub fn run_line(&mut self, line: String) -> Result<(), Error> {
        for token in tokenize(line) {
            try!(self.parse_token(token));
        }
        Ok(())
    }

    fn parse_token(&mut self, token: Token) -> Result<(), Error> {
        println!("{:?}", token);
        Ok(())
    }
}

#[derive(Debug)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn from(msg: &str) -> Error {
        Error { msg: msg.to_string() }
    }
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
