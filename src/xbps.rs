use std::process::Command;

use crate::{InstallArgs, RemoveArgs, Result, SearchArgs, UpgradeArgs};

#[derive(Debug, Default)]
pub struct Xbps {}

impl Xbps {
    pub fn new() -> Self {
        Self {}
    }
    pub fn install(&self, args: InstallArgs) -> Result<()> {
        let mut options: Vec<&str> = vec![];

        if args.yes {
            options.push("-y");
        }

        Command::new("sudo")
            .arg("xbps-install")
            .args(options)
            .args(args.packages)
            .spawn()?
            .wait()?;

        Ok(())
    }
    pub fn update(&self) -> Result<()> {
        Command::new("sudo")
            .arg("xbps-install")
            .arg("-S")
            .spawn()?
            .wait()?;

        Ok(())
    }
    pub fn upgrade(&self, args: UpgradeArgs) -> Result<()> {
        let mut options: Vec<&str> = vec![];

        if args.yes {
            options.push("-y");
        }

        // Upgrade xbps itself
        Command::new("sudo")
            .arg("xbps-install")
            .arg("-u")
            .args(&options)
            .arg("xbps")
            .spawn()?
            .wait()?;

        Command::new("sudo")
            .arg("xbps-install")
            .arg("-u")
            .args(options)
            .args(args.packages)
            .spawn()?
            .wait()?;

        Ok(())
    }
    pub fn remove(&self, args: RemoveArgs) -> Result<()> {
        let mut options: Vec<&str> = vec![];

        if args.yes {
            options.push("-y");
        }

        Command::new("sudo")
            .arg("xbps-remove")
            .arg("-R")
            .args(options)
            .args(args.packages)
            .spawn()?
            .wait()?;

        Ok(())
    }
    pub fn search(&self, args: SearchArgs) -> Result<()> {
        Command::new("xbps-query")
            .arg("-Rs")
            .arg(args.package)
            .spawn()?
            .wait()?;

        Ok(())
    }
}
