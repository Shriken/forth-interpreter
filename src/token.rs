use forth;

#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Number(forth::Number),
    OpenWordDef,
    CloseWordDef,
}

macro_rules! word   { ($x:expr) => { Token::Word($x) } }
macro_rules! number { ($x:expr) => { Token::Number($x) } }

const OPEN_WORD_DEF_TOKEN: &'static str = ":";
const CLOSE_WORD_DEF_TOKEN: &'static str = ";";

pub fn tokenize(line: String) -> Vec<Token> {
    line.split_whitespace()
        .map(|w| {
            if let Ok(n) = w.parse::<forth::Number>() {
                return number!(n);
            }

            match w {
                OPEN_WORD_DEF_TOKEN => Token::OpenWordDef,
                CLOSE_WORD_DEF_TOKEN => Token::CloseWordDef,
                _ => word!(w.to_string()),
            }
        }).collect::<Vec<_>>()
}

#[test]
fn test_tokenize() {
    assert_eq!(
        tokenize("abc   def".to_string()),
        vec![
            word!("abc".to_string()),
            word!("def".to_string()),
        ]
    );

    assert_eq!(
        tokenize("ab1 2c 3".to_string()),
        vec![
            word!("ab1".to_string()),
            word!("2c".to_string()),
            number!(3),
        ]
    );

    assert_eq!(
        tokenize(": a 1 b 2 ;".to_string()),
        vec![
            Token::OpenWordDef,
            word!("a".to_string()),
            number!(1),
            word!("b".to_string()),
            number!(2),
            Token::CloseWordDef,
        ]
    );
}
