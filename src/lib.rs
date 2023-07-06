#![doc = include_str!("../README.md")]

use bitflags::bitflags;
use lalrpop_util::{lalrpop_mod, ParseError};
use tokens_cpp::{LexicalError, Token};

use crate::tokens_cpp::Lexer;

mod tokens_cfg;
mod tokens_cpp;

bitflags! {
    /// Config option flags.
    #[rustfmt::skip]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct CFGFlags: u16 {
        const SAVE =            1 << 0;
        const CLIENT =          1 << 1;
        const SERVER =          1 << 2;
        const INSENSITIVE =     1 << 3;
        const NON_TEEHISTORIC = 1 << 4;
        const MASTER =          1 << 5;
        const ECON =            1 << 6;
        const GAME =            1 << 7;
        const COLALPHA =        1 << 8;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfigEntry {
    /// The description of this config entry.
    pub description: String,
    /// The type of the entry.
    pub entry_type: EntryType,
    /// The flags of the entry.
    pub flags: CFGFlags,
    /// The name of the entry, used in the config file.
    pub name: String,
    /// The symbol of the entry as used in the source code.
    pub symbol: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EntryType {
    Str {
        /// The max length of this string value config.
        max_length: usize,
        /// The default string.
        default: String,
        /// The value if used/filled by the config file.
        value: Option<String>,
    },
    Int {
        /// The max value of this int config.
        max: i64,
        /// The min value of this int config.
        min: i64,
        /// The default value of this int config.
        default: i64,
        /// The value if used/filled by the config file.
        value: Option<i64>,
    },
    Color {
        /// The default value of this color config.
        default: i64,
        /// The value if used/filled by the config file.
        value: Option<i64>,
    },
}

lalrpop_mod!(pub(crate) grammar_cpp);

/// Parses the given header file containing the MACRO_CONFIG_XXX options.
///
/// Usually at `src/engine/shared/config_variables.h`
pub fn parse_config_variables(
    header_source: &str,
) -> Result<Vec<ConfigEntry>, ParseError<usize, Token, LexicalError>> {
    let lexer = Lexer::new(header_source);
    let parser = grammar_cpp::ConfigsParser::new();
    parser.parse(lexer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses() {
        let header_source = include_str!("../config_variables.h");
        let vars = parse_config_variables(header_source).unwrap();
        assert!(!vars.is_empty())
    }
}
