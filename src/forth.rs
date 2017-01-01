use std::error;
use std::fmt;

use token::Token;
use token::tokenize;

pub type Number = i32;

pub struct State {
    value_stack: Vec<Number>,
}

impl State {
    pub fn new() -> State {
        State {
            value_stack: Vec::new(),
        }
    }

    pub fn run_line(&mut self, line: String) -> Result<(), Error> {
        for token in tokenize(line) {
            try!(self.parse_token(token));
        }
        Ok(())
    }

    fn parse_token(&mut self, token: Token) -> Result<(), Error> {
        match token {
            Token::Number(n) => self.value_stack.push(n),
            _ => return Err(
                Error::from(format!("token {:?} not handled", token))
            ),
        }
        Ok(())
    }
}

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
