use clap::Subcommand;

pub mod run;
pub mod upgrade;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run the main functionality
    Run(run::Args),

    /// Upgrade the CLI to the latest version
    Upgrade(upgrade::Args),
}
