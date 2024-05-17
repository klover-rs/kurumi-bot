use std::path::PathBuf;

use lazy_static::lazy_static;
const CONFIG_DIR_STD: &str = "kurumi-bot";
lazy_static::lazy_static! {
    pub static ref CONFIG_DIR: PathBuf = get_config_dir();
}

lazy_static! {
    pub static ref CONFIG_FILE: PathBuf = get_config_file();
}

pub fn mk_config_file() -> Result<(), std::io::Error> {
    let config_dir = get_config_dir();

    let config_file = config_dir.join("config.toml");
    mk_config_dir()?;
    if !config_file.exists() {
        std::fs::File::create(config_file)?;
    }
    Ok(())
}

pub fn get_config_file() -> PathBuf {
    get_config_dir().join("config.toml")
}
pub fn mk_config_dir() -> Result<(), std::io::Error> {
    let config_dir = get_config_dir();

    std::fs::create_dir_all(config_dir)
}

pub fn get_config_dir() -> PathBuf {
    let config_dir = dirs::config_dir();

    config_dir.unwrap().join(CONFIG_DIR_STD)
}
