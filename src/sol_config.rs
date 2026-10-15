/// App configuration - keeps the various subcommand configs consistent across all
/// apps/sub-apps.
use std::fs;

use serde::{self, Deserialize, Serialize, de::DeserializeOwned};
use toml;

/// Get the main application directory in the user's home directory.
pub fn get_cli_root() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_dir = home_dir.join(".sol_cli");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }

    std::path::PathBuf::from(config_dir)
}

pub fn get_default_downloads_dir() -> std::path::PathBuf {
    dirs::download_dir().expect("Could not find downloads directory")
}

pub fn get_app_dir(app_name: &str) -> std::path::PathBuf {
    let cli_root = get_cli_root();
    let notes_path = cli_root.join(app_name);
    if !notes_path.exists() {
        std::fs::create_dir_all(&notes_path).expect("Failed to create directory");
    }
    notes_path
}

fn get_app_config_path(app_name: &str) -> std::path::PathBuf {
    let app_dir = get_app_dir(app_name);
    let filename = format!("{}_config.toml", app_name);
    let config_name = app_dir.join(filename);
    config_name
}

pub trait Config: DeserializeOwned + Default + Serialize {
    fn app_name(&self) -> &str;

    fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let cfg_filename = get_app_config_path(&self.app_name());

        // create app directory if it doesn't exist.
        if let Some(parent) = cfg_filename.parent() {
            fs::create_dir_all(parent)?;
        }

        let toml_content = toml::ser::to_string_pretty(self)?;
        fs::write(&cfg_filename, toml_content)?;

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(default)]
pub struct BaseConfig {
    pub app: String,
    pub log_level: String,
}

impl Default for BaseConfig {
    fn default() -> Self {
        Self {
            app: "Base_Config".into(),
            log_level: "info".to_string(),
        }
    }
}

impl Config for BaseConfig {
    fn app_name(&self) -> &str {
        &self.app
    }
}

/// Base config loading functionaliity - handles filepaths in accordance with
/// the config file structure, should be derived in the child struct to
/// implement the actual config fields
pub fn load_config<T>(app_name: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: Config,
{
    let cfg_filename = get_app_config_path(app_name);
    // create the default config, if no file exists
    if !cfg_filename.exists() {
        let default_config = T::default();
        let _ = default_config.save();
    }

    let content = fs::read_to_string(&cfg_filename)?;
    let parsed_toml: T = toml::from_str(content.as_str())?;
    Ok(parsed_toml)
}
