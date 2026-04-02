use std::{io::Write, process::Command};

use crate::{Error, InstallArgs, RemoveArgs, Result, SearchArgs, UpgradeArgs};

const REPO: &str = "repository=https://github.com/noid-linux/noid-packages/releases/latest/current";

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

        let status = Command::new("sudo")
            .arg("xbps-install")
            .args(options)
            .args(args.packages)
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(Error::XbpsFailed(status.code().unwrap_or(-1)));
        }

        Ok(())
    }
    pub fn update(&self) -> Result<()> {
        let status = Command::new("sudo")
            .arg("xbps-install")
            .arg("-S")
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(Error::XbpsFailed(status.code().unwrap_or(-1)));
        }

        Ok(())
    }
    pub fn upgrade(&self, args: UpgradeArgs) -> Result<()> {
        let mut options: Vec<&str> = vec![];

        if args.yes {
            options.push("-y");
        }

        // Upgrade xbps itself
        let status = Command::new("sudo")
            .arg("xbps-install")
            .arg("-u")
            .args(&options)
            .arg("xbps")
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(Error::XbpsFailed(status.code().unwrap_or(-1)));
        }

        let status = Command::new("sudo")
            .arg("xbps-install")
            .arg("-u")
            .args(options)
            .args(args.packages)
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(Error::XbpsFailed(status.code().unwrap_or(-1)));
        }

        Ok(())
    }
    pub fn remove(&self, args: RemoveArgs) -> Result<()> {
        let mut options: Vec<&str> = vec![];

        if args.yes {
            options.push("-y");
        }

        let status = Command::new("sudo")
            .arg("xbps-remove")
            .arg("-R")
            .args(options)
            .args(args.packages)
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(Error::XbpsFailed(status.code().unwrap_or(-1)));
        }

        Ok(())
    }
    pub fn search(&self, args: SearchArgs) -> Result<()> {
        let status = Command::new("xbps-query")
            .arg("-Rs")
            .arg(args.package)
            .spawn()?
            .wait()?;

        if !status.success() {
            return Err(Error::XbpsFailed(status.code().unwrap_or(-1)));
        }

        Ok(())
    }
    pub fn repo(&self) -> Result<()> {
        let mut child = Command::new("sudo")
            .arg("tee")
            .arg("/etc/xbps.d/00-repository-noid.conf")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(REPO.as_bytes())?;
        }

        child.wait()?;

        Ok(())
    }
}
