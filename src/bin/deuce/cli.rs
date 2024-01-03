use clap::Command;
use self::error::{ CliResult, CliError };
use crate::commands;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn cli() -> Command {
    Command::new(NAME)
        .version(VERSION)
        .subcommands(commands::builtin())
        .subcommand_required(true)
}

pub fn exec() -> CliResult {
    let cmd = cli().try_get_matches().map_err(|error| {
        match error.print() {
            Ok(_) => CliError::NoCommand,
            Err(error) => {
                let error = anyhow::Error::new(error).context(
                    "Could note print CLI message."
                );

                CliError::Other { error }
            },
        }
    })?;

    match cmd.subcommand() {
        Some((name, arg_matches)) => {
            // Unwrap should not panic if `subcommand_required` is true.
            let f = commands::buitin_exec(name).unwrap();
            f(arg_matches)
        },
        None => unreachable!("`subcommand_required` should be true."),
    }
}

pub mod error {
    use std::fmt;
    use std::process::ExitCode;

    pub type CliResult = Result<(), CliError>;

    #[derive(Debug)]
    pub enum CliError {
        NoCommand,
        Other { error: anyhow::Error },
    }

    impl CliError {
        pub fn exit_code(&self) -> ExitCode {
            match &self {
                CliError::Other { error: _ } => ExitCode::from(1),
                CliError::NoCommand => ExitCode::from(2),
            }
        }
    }

    impl std::error::Error for CliError {}

    impl fmt::Display for CliError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self {
                CliError::NoCommand => {
                    write!(f, "No command provided.")
                },
                CliError::Other { error } => error.fmt(f),
            }
        }
    }
}
