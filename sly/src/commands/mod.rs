use anyhow::Result;
use clap::Parser as Clap;
use std::path::PathBuf;

use crate::lib::{env::Env, command::Command};

mod principal;

/// Psychedelic's CLI for the Internet Computer.
#[derive(Clap)]
#[clap(version = "0.1", author = "Psychedelic Team")]
pub struct App {
    /// The network to use when making calls to the I.C.
    #[clap(short, long, default_value = "local")]
    pub network: String,
    /// The identity that should be used. This overwrites the
    /// default identity.
    #[clap(long)]
    pub identity: Option<String>,
    /// Optional path to the sly.json file.
    #[clap(long)]
    pub config: Option<PathBuf>,
    /// A level of verbosity, can be used multiple times.
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: i32,
    #[clap(subcommand)]
    pub sub: AppSubCommands,
}

#[derive(Clap)]
pub enum AppSubCommands {

    /// Search for a given principal id.
    PrincipalGen(principal::PrincipalOpts),
}

impl Command for AppSubCommands {
    fn exec(self, env: &mut Env) -> Result<()> {
        match self {
            AppSubCommands::PrincipalGen(opts) => opts.exec(env),
        }
    }
}
