use clap::{Args, Parser, Subcommand};

/// A user-friendly package manager wrapper for XBPS with AppImage support
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Install one or more packages
    #[command(name = "install")]
    Install(InstallArgs),

    /// Update package database
    #[command(name = "update")]
    Update,

    /// Upgrade installed packages
    #[command(name = "upgrade")]
    Upgrade(UpgradeArgs),

    /// Remove one or more packages
    #[command(name = "remove")]
    Remove(RemoveArgs),

    /// Search for packages in repositories
    #[command(name = "search")]
    Search(SearchArgs),

    /// Manage AppImages (alias: a, ai)
    #[command(name = "appimage", aliases = ["a", "ai"])]
    AppImage {
        #[command(subcommand)]
        action: zap_rs::Command,
    },
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    /// Package names to install
    pub packages: Vec<String>,

    /// Assume yes for all prompts
    #[arg(long, short = 'y', default_value_t = false)]
    pub yes: bool,
}

#[derive(Debug, Args)]
pub struct UpgradeArgs {
    /// Specific packages to upgrade (all if empty)
    pub packages: Vec<String>,

    /// Assume yes for all prompts
    #[arg(long, short = 'y', default_value_t = false)]
    pub yes: bool,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// Package names to remove
    pub packages: Vec<String>,

    /// Assume yes for all prompts
    #[arg(long, short = 'y', default_value_t = false)]
    pub yes: bool,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// Package name or pattern to search for
    pub package: String,
}
