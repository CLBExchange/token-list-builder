//! CLI tool for building combined token lists.
use anyhow::Result;

mod commands;
pub mod lists;
pub mod resolvers;

use clap::{Parser, Subcommand};
use lists::Lists;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Updates the `lists/` directory.
    Update,
    /// Builds the token list as defined in `Lists.toml`. Ensure `update` was run first.
    Build,
}

/// Entrypoint to the CLI.
#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let lists_file = std::fs::read_to_string("Lists.toml")?;
    let lists_config: Lists = toml::from_str(lists_file.as_str())?;

    match &cli.command {
        Some(Commands::Update) => {
            commands::update::handler(&lists_config).await?;
        }
        Some(Commands::Build) => {
            commands::build::handler(&lists_config).await?;
        }
        None => {}
    }

    Ok(())
}
