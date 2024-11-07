use std::fs;
use std::path::PathBuf;
use std::error::Error;
use toml;

pub fn info(home_dir: PathBuf) -> Result<(), Box<dyn Error>> {
    let config_path = home_dir.join(".sql-generation").join("config.toml");
    if !config_path.exists() {
        return Err("config.toml file does not exist. Run `sql-generation init` command first.".into());
    }

    let config_content = fs::read_to_string(config_path)?;
    let config: toml::Value = toml::from_str(&config_content)?;

    println!("Configuration:");
    println!("{}", toml::to_string_pretty(&config)?);

    Ok(())
}
