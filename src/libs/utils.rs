use log::debug;

use crate::libs::structs::TOMLData;
use std::{fs, process::exit};

use super::structs::{AppState, CargoPkgInfo};

// Loads TOMLData struct from filename
pub fn load_config_toml(filename: String) -> TOMLData {
    // Load in raw string from config toml
    let toml_raw = match fs::read_to_string(&filename) {
        Ok(c) => c,
        // Failed to read file
        Err(e) => {
            println!("Could not read TOML file '{}'", &filename);
            println!("Error: {}", e);
            exit(1);
        }
    };
    // Convert to TOML struct
    let config_data: TOMLData = match toml::from_str(&toml_raw) {
        Ok(d) => d,
        // Failed to parse from String to TOMLData Struct
        Err(e) => {
            println!("Unable to load data from {}", &filename);
            println!("Error: {}", e);
            exit(1);
        }
    };
    config_data
}

// Function that returns true if an api key is valid, else false
pub fn validate_api_key(app_state: &AppState, key: &String) -> bool {
    debug!(
        "API key in - \"{}\" vs Accepted keys: {:?}",
        key, app_state.api_keys
    );
    app_state.api_keys.contains(key)
}

// Draws start screen containing app version and ascii
pub fn draw_start_screen(package_info: &CargoPkgInfo) {
    let ascii_name = r#"     ____                        
    / ___|___  _ __   __ _  __ _ 
   | |   / _ \| '_ \ / _` |/ _` |
   | |__| (_) | | | | (_| | (_| |
    \____\___/|_| |_|\__, |\__,_|
                     |___/       "#;

    println!("{} v{}", &ascii_name, package_info.version);
    println!("\n   Created by {}", package_info.authors);
    println!("==================================================")
}
