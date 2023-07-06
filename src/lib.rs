use bitflags::bitflags;
use lalrpop_util::{lalrpop_mod, ParseError};
use tokens_cpp::{LexicalError, Token};

use crate::tokens_cpp::Lexer;

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
        const COLALPHA =  0b100000000;
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConfigEntry {
    /// The description of this config entry.
    description: String,
    /// The type of the entry.
    entry_type: EntryType,
    /// The flags of the entry.
    flags: CFGFlags,
    /// The name of the entry, used in the config file.
    name: String,
    /// The symbol of the entry as used in the source code.
    symbol: String,
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
