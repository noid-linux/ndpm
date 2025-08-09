use clap::Parser;
use colored::Colorize;
use zap_rs::{AppImage, PackageManager};

use ndpm::{Cli, Command, Error, Result, Xbps, is_root};

async fn run() -> Result<()> {
    if is_root() {
        return Err(Error::PermissionDenied);
    }

    let args = Cli::parse();
    let xbps = Xbps::new();

    match args.command {
        Command::Install(args) => xbps.install(args)?,
        Command::Update => xbps.update()?,
        Command::Upgrade(args) => xbps.upgrade(args)?,
        Command::Remove(args) => xbps.remove(args)?,
        Command::Search(args) => xbps.search(args)?,
        Command::AppImage { action } => {
            let pm = PackageManager::new();
            match action {
                zap_rs::Command::Install(args) => {
                    let mut appimage = AppImage::new(&args);

                    pm.install(&mut appimage, &args.appname).await?;
                }
                zap_rs::Command::Remove(args) => pm.remove(&args.appname).await?,
                zap_rs::Command::List => pm.list().await?,
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
