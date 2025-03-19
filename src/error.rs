use crate::lexer::error::InvalidCharacterError;

pub enum CompilerError {
    InvalidCharacter(InvalidCharacterError),
}