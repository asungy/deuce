use clap::{ArgMatches, Command};
use crate::cli::error::CliResult;

pub(super) const NAME: &str = "start";

pub fn cli() -> Command {
    clap::Command::new(NAME)
        .about("Start the server.")
}

pub fn exec(_args: &ArgMatches) -> CliResult {
    deuce::greet();
    Ok(())
}
