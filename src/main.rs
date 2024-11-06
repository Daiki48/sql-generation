use clap::{command, Parser, Subcommand};

use std::path::PathBuf;

mod consts;
mod cli_init;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Init,
}

fn get_home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    dirs::home_dir().ok_or_else(|| "Home directory not found".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args= Args::parse();
    let home_dir = get_home_dir()?;

    // println!("args is {:?}", args);

    match args.cmd {
        Commands::Init => cli_init::init(home_dir, consts::INIT_CONFIG)?,
    }
    Ok(())
}
