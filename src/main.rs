use std::{error::Error, path::PathBuf};

use clap::Parser;
use config::ConfigurationHolder;
use console::{set_colors_enabled, Style};

mod cli;
mod config;
mod model;
mod repository;
mod secret_key;
mod utils;

#[derive(Debug, Parser)]
#[clap(name = "dotium", version = clap::crate_version!())]
pub struct DotiumOptions {
    #[clap(short, long, help = "Config file to use")]
    config: Option<PathBuf>,

    #[clap(short, long, help = "Secret age keys file to use")]
    keys: Option<PathBuf>,

    #[clap(long, help = "Do not use ansi colors")]
    no_colors: bool,

    #[clap(subcommand)]
    sub_command: cli::Subcommand,
}

fn main() {
    let opts = DotiumOptions::parse();

    if opts.no_colors {
        set_colors_enabled(false);
    }

    let config = match ConfigurationHolder::read_config(&opts.config, &opts.keys) {
        Ok(config) => config,
        Err(err) => exit_on_error(err),
    };

    if let Err(err) = opts.sub_command.run(config) {
        exit_on_error(err);
    }
}

fn exit_on_error(err: Box<dyn Error>) -> ! {
    let style = Style::new().bold().red();
    println!("{}", style.apply_to(format!("{}", err)));
    std::process::exit(1);
}
