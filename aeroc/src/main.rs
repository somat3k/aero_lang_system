use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod commands;

/// AERO language compiler and toolchain
#[derive(Parser, Debug)]
#[command(name = "aeroc")]
#[command(version, about, long_about = None)]
#[command(author = "AERO Research Consortium")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new AERO project
    New {
        /// Project name
        name: String,
        /// Project directory (defaults to current directory + name)
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Check the project for errors without building
    Check {
        /// Path to the project directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
    },
    /// Build the AERO project to AVM bytecode
    Build {
        /// Path to the project directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
        /// Build in release mode with optimizations
        #[arg(long)]
        release: bool,
    },
    /// Run the AERO project
    Run {
        /// Path to the project directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
        /// Build in release mode with optimizations
        #[arg(long)]
        release: bool,
        /// Arguments to pass to the program
        args: Vec<String>,
    },
    /// Run tests in the project
    Test {
        /// Path to the project directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
        /// Filter tests by name
        filter: Option<String>,
    },
    /// Format AERO source files
    Fmt {
        /// Path to the project directory
        #[arg(long, default_value = ".")]
        path: PathBuf,
        /// Check formatting without making changes
        #[arg(long)]
        check: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, path } => commands::new::execute(name, path),
        Commands::Check { path } => commands::check::execute(path),
        Commands::Build { path, release } => commands::build::execute(path, release),
        Commands::Run { path, release, args } => commands::run::execute(path, release, args),
        Commands::Test { path, filter } => commands::test::execute(path, filter),
        Commands::Fmt { path, check } => commands::fmt::execute(path, check),
    }
}
