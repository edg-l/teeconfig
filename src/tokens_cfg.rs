use logos::{Logos, SpannedIter};
use std::fmt;

use crate::tokens_cpp::{LexicalError, LexingError, Spanned}; // to implement the Display trait

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(
    error = LexingError,
    skip r"[ \t\n\f]+"
)]
pub enum ConfigToken {
    #[regex(r#""(?:[^"]|\\")*""#, |lex| {
        let slice = lex.slice();
        slice[1..(slice.len()-1)].to_string()
    })]
    StringLiteral(String),
    #[regex(r"[a-zA-Z_][a-zA-Z_0-9]*", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", |lex| i64::from_str_radix(&lex.slice()[2..], 16))]
    #[regex(r"-?[0-9][_0-9]*", |lex| lex.slice().parse())]
    Integer(i64),
    #[regex(r"[0-9]{1,3}.[0-9]{1,3}.[0-9]{1,3}.[0-9]{1,3}:[0-9]+", |lex| lex.slice().to_string())]
    IP(String),

    #[regex("\r?\n")]
    Endline,
}

impl fmt::Display for ConfigToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct ConfigLexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, ConfigToken>,
}

impl<'input> ConfigLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: ConfigToken::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for ConfigLexer<'input> {
    type Item = Spanned<ConfigToken, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| match token {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(err) => Err(LexicalError::InvalidToken(err, span)),
        })
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    String(String),
    IP(String),
    Key(String),
}

#[derive(Debug, Clone)]
pub struct ConfigLine {
    pub name: String,
    pub values: Vec<Value>,
}
