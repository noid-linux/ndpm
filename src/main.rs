use clap::Parser;

use ndpm::{Cli, Command, Xbps};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let xbps = Xbps::new();

    match args.command {
        Command::Install(args) => xbps.install(args)?,
        Command::Update => xbps.update()?,
        Command::Upgrade(args) => xbps.upgrade(args)?,
        Command::Remove(args) => xbps.remove(args)?,
        Command::Search(args) => xbps.search(args)?,
    }

    Ok(())
}
