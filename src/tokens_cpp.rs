use logos::{Logos, SpannedIter};
use std::{convert::Infallible, fmt, ops::Range}; // to implement the Display trait

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(
    error = LexingError,
    skip r"[ \t\n\f]+",
    skip r"/\*.*\*/" // why doesnt this skip?
    skip r"#.*\n?",
    skip r"//.*\n?",)]
pub enum Token {
    #[token("MACRO_CONFIG_INT")]
    MacroConfigInt,
    #[token("MACRO_CONFIG_STR")]
    MacroConfigStr,
    #[token("MACRO_CONFIG_COL")]
    MacroConfigColor,

    #[token("CFGFLAG_SAVE")]
    FlagSave,
    #[token("CFGFLAG_CLIENT")]
    FlagClient,
    #[token("CFGFLAG_SERVER")]
    FlagServer,
    #[token("CFGFLAG_INSENSITIVE")]
    FlagInsensitive,
    #[token("CFGFLAG_NONTEEHISTORIC")]
    FlagNonTeehistoric,
    #[token("CFGFLAG_MASTER")]
    FlagMaster,
    #[token("CFGFLAG_ECON")]
    FlagEcon,
    #[token("CFGFLAG_GAME")]
    FlagGame,

    #[regex(r#""(?:[^"]|\\")*""#, |lex| lex.slice().parse().ok())]
    StringLiteral(String),
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]+", |lex| lex.slice().parse().ok())]
    Identifier(String),
    #[regex(r"-?[0-9][_0-9]*", |lex| lex.slice().parse().ok())]
    Integer(i64),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token("|")]
    Pipe,
    #[token("MAX_CLIENTS")]
    MaxClients,

    #[token("SERVERINFO_LEVEL_MIN")]
    ServerInfoLevelMin,
    #[token("SERVERINFO_LEVEL_MAX")]
    ServerInfoLevelMax,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, PartialEq, Clone, Default)]
pub enum LexingError {
    NumberParseError,
    #[default]
    Other,
}

#[derive(Debug, Clone)]
pub enum LexicalError {
    InvalidToken(LexingError, Range<usize>),
}

impl From<std::num::ParseIntError> for LexingError {
    fn from(_: std::num::ParseIntError) -> Self {
        LexingError::NumberParseError
    }
}

impl From<Infallible> for LexingError {
    fn from(_: Infallible) -> Self {
        LexingError::Other
    }
}

pub struct Lexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| match token {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(err) => Err(LexicalError::InvalidToken(err, span)),
        })
    }
}
