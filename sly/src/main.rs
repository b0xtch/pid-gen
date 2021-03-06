use anyhow::Result;
use clap::Parser as Clap;
use human_panic::setup_panic;
use log::LevelFilter;
use pretty_env_logger::formatted_builder;

use crate::lib::command::Command;
use crate::lib::env::Env;

mod commands;
mod lib;

fn main() -> Result<()> {
    setup_panic!();

    let opts: commands::App = commands::App::parse();

    let level = if opts.verbose >= 2 {
        LevelFilter::Trace
    } else if opts.verbose == 1 {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    formatted_builder().filter(None, level).init();

    let identity = opts.identity.as_deref();
    let config_path = opts.config;
    let mut env = Env::new(opts.network, identity, config_path)?;

    opts.sub.exec(&mut env)
}
