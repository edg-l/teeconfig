use logos::{FilterResult, Logos, SpannedIter};
use std::{convert::Infallible, fmt, ops::Range}; // to implement the Display trait

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(
    error = LexingError,
    skip r"[ \t\n\f]+",
    skip r"#[^\n]*\n?",
    skip r"//[^\n]*\n?",
)]
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
    #[token("CFGFLAG_COLALPHA")]
    FlagColAlpha,
    #[token("CFGFLAG_COLLIGHT")]
    FlagColLight,

    #[regex(r#""(?:[^"]|\\")*""#, |lex| {
        let slice = lex.slice();
        slice[1..(slice.len()-1)].to_string()
    })]
    StringLiteral(String),
    #[regex(r"[_a-zA-Z][_0-9a-zA-Z]+", |lex| lex.slice().to_string())]
    Identifier(String),
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", |lex| i64::from_str_radix(&lex.slice()[2..], 16))]
    #[regex(r"-?[0-9][_0-9]*", |lex| lex.slice().parse())]
    Integer(i64),

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token(",")]
    Comma,
    #[token("|")]
    Pipe,
    #[token(";")]
    Semicolon,
    #[token("MAX_CLIENTS")]
    MaxClients,

    #[token("SERVERINFO_LEVEL_MIN")]
    ServerInfoLevelMin,
    #[token("SERVERINFO_LEVEL_MAX")]
    ServerInfoLevelMax,

    #[token("/*", |lex| {
        let len = lex.remainder().find("*/").unwrap();
        lex.bump(len + 2); // include len of `*/`

        FilterResult::Skip::<(), Infallible>
    })]
    BlockComment,
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

pub struct VarLexer<'input> {
    // instead of an iterator over characters, we have a token iterator
    token_stream: SpannedIter<'input, Token>,
}

impl<'input> VarLexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self {
            token_stream: Token::lexer(input).spanned(),
        }
    }
}

impl<'input> Iterator for VarLexer<'input> {
    type Item = Spanned<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream.next().map(|(token, span)| match token {
            Ok(token) => Ok((span.start, token, span.end)),
            Err(err) => Err(LexicalError::InvalidToken(err, span)),
        })
    }
}
