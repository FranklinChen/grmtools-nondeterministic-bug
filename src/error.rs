use thiserror::Error;

// Do not use serde because DefaultLexeme is not ours.
#[derive(Debug, Error, PartialEq)]
pub enum TermError<'input> {
    #[error("number too big for u32: {0}")]
    NumberTooBig(&'input str),

    #[error("lexeme error: {0}")]
    LexemeError(#[from] lrlex::DefaultLexeme),

    #[error("unknown token: {0}")]
    UnknownToken(&'input str),
}
