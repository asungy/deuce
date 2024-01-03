mod start;

use clap::{ArgMatches, Command};
use crate::cli::error::CliResult;

pub fn builtin() -> Vec<Command> {
    vec![
        start::cli(),
    ]
}

pub type Exec = fn(&ArgMatches) -> CliResult;

pub fn buitin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        start::NAME => start::exec,
        _ => return None,
    };

    Some(f)
}
