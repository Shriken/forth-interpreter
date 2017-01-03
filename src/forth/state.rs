use std::collections::hash_map::HashMap;

use forth;
use forth::Error;
use forth::native_words::NATIVE_WORDS;
use forth::token::Token;
use forth::token::tokenize;

pub struct State {
    value_stack: Vec<forth::Number>,
    dictionary: HashMap<String, Vec<Token>>,
    mode: Mode,
}

enum Mode {
    Normal,
    WordDefName, // just saw OpenWordDef, looking for name
    WordDefBody(String, Vec<Token>),
}

impl State {
    pub fn new() -> State {
        State {
            value_stack: Vec::new(),
            dictionary: HashMap::new(),
            mode: Mode::Normal,
        }
    }

    pub fn run_line(&mut self, line: String) -> Result<(), Error> {
        for token in tokenize(line) {
            try!(self.parse_token(token));
        }
        Ok(())
    }

    fn parse_token(&mut self, token: Token) -> Result<(), Error> {
        let mut change_mode = false;
        let mut new_mode: Mode = Mode::Normal;

        match self.mode {
            Mode::Normal => match token {
                Token::Number(n) => self.value_stack.push(n),
                Token::Word(w) => try!(self.run_word(w)),
                Token::OpenWordDef => {
                    change_mode = true;
                    new_mode = Mode::WordDefName;
                },
                _ => return Err(Error::from(
                    format!("token {:?} not handled", token)
                )),
            },

            Mode::WordDefName => match token {
                Token::Word(w) => {
                    change_mode = true;
                    new_mode = Mode::WordDefBody(w, Vec::new());
                },
                _ => return Err(Error::from(
                    format!("expected word name, not {:?}", token)
                )),
            },

            Mode::WordDefBody(ref name, ref mut body) => {
                match token {
                    Token::OpenWordDef => return Err(Error::from(
                        format!("already in word definition")
                    )),

                    Token::CloseWordDef => {
                        self.dictionary.insert(
                            name.clone(),
                            body.clone()
                        );
                        change_mode = true;
                        new_mode = Mode::Normal;
                    },

                    _ => body.push(token.clone()),
                }
            },
        }

        if change_mode {
            self.mode = new_mode;
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
