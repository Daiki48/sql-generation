use std::path::PathBuf;
use std::fs;
use std::io::Write;

pub fn init(home_dir: PathBuf, config_content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir = home_dir.join(".sql-generation");
    let config_file = config_dir.join("config.toml");

    if config_file.exists() {
        println!("Already created config.toml");
    } else {
        fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        let mut file = fs::File::create(&config_file)?;
        write!(file, "{}", config_content)?;

        println!("Created config.toml with specified settings");
    }
    Ok(())
}
