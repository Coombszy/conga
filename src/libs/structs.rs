use std::sync::{Arc, Mutex};

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub struct CargoPkgInfo {
    pub name: String,
    pub version: String,
    pub authors: String,
}

// TOML Data on loaded on startup
#[derive(Deserialize, Clone, Debug)]
pub struct TOMLData {
    pub config: Config,
}

// Config data stored within TOML Data
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub web_host: String,
    pub web_port: u16,
    pub write_logs: bool,
    pub write_logs_file: String,
    pub api_keys: Option<Vec<String>>,
}

////////////////////////////////////////////////////////////////////////////////////////
// Web server structs

// Actix Application global state
pub struct AppState {
    pub start_time: DateTime<Utc>,
    pub item_queue: Arc<Mutex<Vec<Item>>>,
    pub api_keys: Vec<String>,
}
// Global state impls
impl AppState {
    // Returns current uptime using `start_time`
    pub fn uptime(&self) -> String {
        let duration: Duration = Utc::now() - self.start_time;

        let days = duration.num_days();
        let hours = duration.num_hours() % 24;
        let minutes = duration.num_minutes() % 60;
        let seconds = duration.num_seconds() % 60;

        return format!("{days:02} {hours:02}:{minutes:02}:{seconds:02}",);
    }
}

// Reponse error
#[derive(Serialize, ToSchema)]
pub struct WebError {
    pub timestamp: String,
    pub error: String,
}

// Web route 'health' response body
#[derive(Serialize, ToSchema)]
pub struct WebHealth {
    pub uptime: String,
}

// Web route 'health' response body
#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct Meta {
    pub received_epoch: i64,
}

// Item to be queued
#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct Item {
    pub queue: String,
    pub content: serde_json::Value,
    pub meta: Option<Meta>,
}
