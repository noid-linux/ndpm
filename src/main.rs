use clap::Parser;
use std::path::PathBuf;
use zap_rs::{AppImage, PackageManager, Source, SourceMetadata};

use ndpm::{Cli, Command, Xbps, is_root};

async fn run() -> Result<(), Box<dyn std::error::Error>> {
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
        Command::AppImage { action } => {
            let pm = PackageManager::new();
            match action {
                zap_rs::Command::Install(args) => {
                    let mut options = AppImage {
                        file_path: PathBuf::new(),
                        executable: args.executable.unwrap_or(args.appname.clone()),
                        source: Source {
                            identifier: "raw_url".to_string(),
                            meta: SourceMetadata { url: args.from },
                        },
                    };

                    pm.install(&mut options, &args.appname).await?;
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
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
