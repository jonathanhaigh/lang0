// SPDX-FileCopyrightText: 2023 Jonathan Haigh <jonathanhaigh@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Types and methods related to parsing the command line.

use std::path::PathBuf;

use clap::Parser;

/// Command line arguments passed to lang0c.
#[derive(Parser)]
#[command(author, version, about, long_about=None)]
#[must_use]
pub struct Cli {
    /// The file to compile
    pub input: PathBuf,
}

/// Parse the command line.
///
/// # Returns
/// - a `Cli` struct containing the command line arguments.
pub fn parse() -> Cli {
    Cli::parse()
}
