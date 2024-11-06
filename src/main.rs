use clap::{command, Parser};

use std::path::PathBuf;

mod consts;
mod cli_init;

#[derive(Parser, Debug)]
#[command(name = "sql-generation", author, version, about, long_about)]
struct Cli {
    #[arg(short, long)]
    init: bool
}

fn get_home_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    dirs::home_dir().ok_or_else(|| "Home directory not found".into())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let home_dir = get_home_dir()?;

    if cli.init {
        cli_init::init(home_dir, consts::INIT_CONFIG)?;
    }
    Ok(())
}
