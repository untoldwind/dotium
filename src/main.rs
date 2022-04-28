use clap::Parser;
use log::error;

mod cli;


#[derive(Debug, Parser)]
#[clap(name = "dotium", version = clap::crate_version!())]
struct DotiumOptions {
    #[clap(subcommand)]
    sub_command: cli::Subcommand,
}

fn main() {
    env_logger::builder()
        .format_timestamp(None)
        .filter_level(log::LevelFilter::Off)
        .parse_default_env()
        .init();

    let opts = DotiumOptions::parse();

    if let Err(err) = opts.sub_command.run() {
        error!("{}", err)
    }
}
