// SPDX-FileCopyrightText: 2023 Jonathan Haigh <jonathanhaigh@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Error types.

use std::path::PathBuf;

use miette::{Diagnostic, SourceSpan};
use thiserror::Error as ThisError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    ReadSourceCode,
    Lex,
}

/// Type of error used throughout SQ infrastructure code.
///
/// Errors have `span: SourceSpan` members where possible to point to the part of the input source
/// that is invalid or led to an error.
#[derive(Debug, Diagnostic, ThisError)]
#[must_use]
pub enum Error {
    /// Failure to read source code
    #[error("failed to read source code from file {input}")]
    ReadSourceCode {
        input: PathBuf,

        #[source]
        source: anyhow::Error,
    },

    /// A lexing error.
    #[error("unrecognized token")]
    Lex {
        #[label("unrecognized token")]
        span: SourceSpan,
    },
}

impl Error {
    pub fn kind(&self) -> ErrorKind {
        match self {
            Error::ReadSourceCode { .. } => ErrorKind::ReadSourceCode,
            Error::Lex { .. } => ErrorKind::Lex,
        }
    }
}

/// A value or an `Error`
pub type Result<T> = std::result::Result<T, Box<Error>>;

/// A value, `None` or an `Error`
pub type OptResult<T> = Result<Option<T>>;
