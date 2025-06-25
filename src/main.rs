mod cli;
mod detect;
mod convert;
mod fluxgen;

use clap::Parser;
use cli::{Cli, Commands};


fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Detect { namespace } => {
            detect::run(&namespace)?;
        }
        Commands::Convert {namespace, chart,release, output, url } => {
            convert::run(&namespace, &chart, &release, &output, &url)?;
        }
    }

    Ok(())
}
