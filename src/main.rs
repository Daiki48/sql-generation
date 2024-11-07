use clap::{command, Parser, Subcommand};

use std::path::PathBuf;

mod consts;
mod cli_init;
mod cli_info;
mod cli_config;
mod cli_add;
mod cli_remove;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init,
    Info,
    Config,
    Add,
    Remove,
}

fn get_home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    dirs::home_dir().ok_or_else(|| "Home directory not found".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args= Args::parse();
    let home_dir = get_home_dir()?;

    match args.cmd {
        Commands::Init => cli_init::init(home_dir, consts::INIT_CONFIG)?,
        Commands::Info => cli_info::info(),
        Commands::Config => cli_config::config(),
        Commands::Add => cli_add::add(),
        Commands::Remove => cli_remove::remove(),
    }
    Ok(())
}
