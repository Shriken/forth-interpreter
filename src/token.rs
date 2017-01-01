#[derive(Debug)]
pub enum Token {
    Word(String),
    Number(i32),
    OpenWordDef,
    CloseWordDef,
}

const OPEN_WORD_DEF_TOKEN: &'static str = ":";
const CLOSE_WORD_DEF_TOKEN: &'static str = ";";

pub fn tokenize(line: String) -> Vec<Token> {
    line.split_whitespace()
        .map(|w| {
            if let Ok(n) = w.parse::<i32>() {
                return Token::Number(n);
            }

            match w {
                OPEN_WORD_DEF_TOKEN => Token::OpenWordDef,
                CLOSE_WORD_DEF_TOKEN => Token::CloseWordDef,
                _ => Token::Word(w.to_string()),
            }
        }).collect::<Vec<_>>()
}
