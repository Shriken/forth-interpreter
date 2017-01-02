use token::Token;
use token::tokenize;
use forth;
use forth::error::Error;
use super::native_words::NATIVE_WORDS;

pub struct State {
    value_stack: Vec<forth::Number>,
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
            Token::Word(w) => try!(self.run_word(w)),
            _ => return Err(
                Error::from(format!("token {:?} not handled", token))
            ),
        }
        Ok(())
    }

    fn run_word(&mut self, word: String) -> Result<(), Error> {
        for &(name, func) in NATIVE_WORDS {
            if word == name {
                return func(&mut self.value_stack);
            }
        }

        Err(Error::from(format!("word {} not recognized", word)))
    }
}
