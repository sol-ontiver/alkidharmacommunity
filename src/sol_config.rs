/// App configuration

/// Get the main application directory in the user's home directory.
pub fn get_app_dir() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_dir = home_dir.join(".sol_cli");
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).expect("Could not create config directory");
    }

    std::path::PathBuf::from(config_dir)
}