// SPDX-FileCopyrightText: 2023 Jonathan Haigh <jonathanhaigh@gmail.com>
//
// SPDX-License-Identifier: MIT

//! Compile a lang0 translation unit

use std::fs;

use lang0::cli;
use lang0::error;

fn run_lang0c(_input: &str, _args: &cli::Cli) -> error::Result<()> {
    Ok(())
}

fn main() -> miette::Result<()> {
    let args = cli::parse();
    let source = fs::read_to_string(&args.input).map_err(|e| {
        miette::Report::new_boxed(Box::new(error::Error::ReadSourceCode {
            input: args.input.clone(),
            source: e.into(),
        }))
    })?;
    run_lang0c(&source, &args).map_err(|e| miette::Report::new_boxed(e).with_source_code(source))
}
