#![allow(
    clippy::all,
    missing_debug_implementations,
)]

mod cli;
mod commands;

use std::process::ExitCode;

fn main() -> ExitCode {
    match cli::exec() {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => error.exit_code(),
    }
}
