use std::collections::hash_map::HashMap;

use token::Token;
use token::tokenize;
use forth;
use forth::error::Error;
use super::native_words::NATIVE_WORDS;

pub struct State {
    value_stack: Vec<forth::Number>,
    dictionary: HashMap<String, Vec<Token>>,
}

impl State {
    pub fn new() -> State {
        State {
            value_stack: Vec::new(),
            dictionary: HashMap::new(),
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

        // find correct defined word
        if let Some(body) = self.get_defined_word(&word) {
            for token in body {
                try!(self.parse_token(token));
            }

            return Ok(())
        }

        return Err(Error::from(format!("word {} not recognized", word)));
    }

    fn get_defined_word(&self, word: &String) -> Option<Vec<Token>> {
        match self.dictionary.keys().find(|w| word == *w) {
            Some(key) => Some(self.dictionary[key].clone()),
            None => None,
        }
    }
}

#[test]
fn test_defined_words() {
    let mut state = State::new();
    state.dictionary.insert("blah".to_string(), vec![
        Token::Number(1),
        Token::Number(2),
        Token::Word("+".to_string())
    ]);

    let blah_word = Token::Word("blah".to_string());
    assert!(state.parse_token(blah_word).is_ok());
    assert_eq!(state.value_stack, vec![3]);
}
