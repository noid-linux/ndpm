use clap::Parser;

use ndpm::{Cli, Command, Xbps, is_root};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if is_root() {
        return Err("Avoid running ndpm as root/sudo.".into());
    }

    let args = Cli::parse();
    let xbps = Xbps::new();

    match args.command {
        Command::Install(args) => xbps.install(args)?,
        Command::Update => xbps.update()?,
        Command::Upgrade(args) => xbps.upgrade(args)?,
        Command::Remove(args) => xbps.remove(args)?,
        Command::Search(args) => xbps.search(args)?,
        Command::AppImage { action } => match action {
            zap_rs::Command::Install(_) => todo!(),
            zap_rs::Command::Remove(_) => todo!(),
            zap_rs::Command::List => todo!(),
        },
    }

    Ok(())
}
