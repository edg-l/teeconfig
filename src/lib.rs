use bitflags::bitflags;
use lalrpop_util::lalrpop_mod;

mod tokens_cfg;
mod tokens_cpp;

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    struct CFGFlags: u32 {
        const SAVE =            0b00000001;
        const CLIENT =          0b00000010;
        const SERVER =          0b00000100;
        const INSENSITIVE =     0b00001000;
        const NON_TEEHISTORIC =  0b00010000;
        const MASTER =  0b00100000;
        const ECON =  0b01000000;
        const GAME =  0b10000000;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfigEntry {
    description: String,
    entry_type: EntryType,
    flags: CFGFlags,
    name: String,
    symbol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntryType {
    Str {
        max_length: usize,
        default: String,
        value: Option<String>,
    },
    Int {
        max: i64,
        min: i64,
        default: i64,
        value: Option<i64>,
    },
    Color {
        default: i64,
        value: Option<i64>,
    },
}

lalrpop_mod!(pub(crate) grammar_cpp);

#[cfg(test)]
mod tests {
    use lalrpop_util::ParseError;
    use logos::Source;

    use crate::tokens_cpp::{Lexer, LexicalError, LexingError, Token};

    use super::*;

    #[test]
    fn parses() {
        let header_file = include_str!("../config_variables.h");
        let lexer = Lexer::new(header_file);
        let parser = grammar_cpp::ConfigsParser::new();

        match parser.parse(lexer) {
            Ok(entries) => {
                dbg!(&entries);
            }
            Err(e) => {
                let x: ParseError<usize, Token, LexicalError> = e;
                match x {
                    ParseError::InvalidToken { location } => todo!(),
                    ParseError::UnrecognizedEof { location, expected } => todo!(),
                    ParseError::UnrecognizedToken { token, expected } => {
                        dbg!("unrecognized token");
                        dbg!(&token);
                        dbg!(header_file.slice(token.0 - 20..token.2 + 20));
                        dbg!(expected);
                    }
                    ParseError::ExtraToken { token } => todo!(),
                    ParseError::User { error } => {
                        match error {
                            LexicalError::InvalidToken(_, r) => {
                                dbg!("invalid token");
                                println!("{:?}", header_file.slice(r).unwrap())
                            }
                        };
                    }
                };
            }
        };
    }
}
