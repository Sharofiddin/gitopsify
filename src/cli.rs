use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitopsify")]
#[command(about = "Convert k8s components to FluxCD GitOps manifests")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Detect Helm releases in ns
    Detect {
        #[arg(short, long)]
        namespace: String,
    },

    /// Convert Helm release into GitOps manifests
    Convert {
        #[arg(short, long)]
        namespace: String,
        #[arg(short, long)]
        chart: String,
        #[arg(short, long)]
        release: String,
        #[arg(short, long)]
        output: String,
        #[arg(short, long)]
        url: String,
    }
}

