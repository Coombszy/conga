mod libs;
use actix_cors::Cors;
use libs::{
    structs::{CargoPkgInfo, Item, TOMLData},
    utils::draw_start_screen,
};

use actix_web::{http, web, App, HttpServer};
use chrono::Utc;
use dotenv::dotenv;
use log::{debug, info, LevelFilter};
use simplelog::*;

use std::fs::File;
use std::sync::{Arc, Mutex};
use std::vec;
use std::{env, str::FromStr};

use crate::libs::{structs::AppState, utils::load_config_toml, middleware::{ Auth}};

const DATA_FOLDER: &str = "config/";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let toml_data: TOMLData = startup();

    let queue = Arc::new(Mutex::new(Vec::<Item>::new()));

    // Start Web
    let host: String = toml_data.clone().config.web_host;
    let port: u16 = toml_data.clone().config.web_port;
    info!("Starting web server, listening on {host}:{port}");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            // .allowed_origin("http://localhost:5500/") // For local development
            // .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b".coombszy.com")) // coombszy.com deployments
            .allowed_methods(vec!["POST", "GET"])
            .allowed_headers(vec![
                http::header::AUTHORIZATION,
                http::header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Auth)
            .wrap(cors)
            .app_data(web::Data::new(AppState {
                start_time: Utc::now(),
                item_queue: queue.clone(),
            }))
            .service(libs::routes::health)
            .service(libs::routes::add_item)
            .service(libs::routes::get_items)
            .service(libs::routes::fetch_items)
    })
    .bind((host, port))?
    .run()
    .await
}

fn startup() -> TOMLData {
    draw_start_screen(&CargoPkgInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        authors: env!("CARGO_PKG_AUTHORS").to_string(),
    });

    // Init environment vars from .env file
    dotenv().ok();

    // Load TOML Data for config
    let toml_data: TOMLData = load_config_toml(format!("{}conga.toml", &DATA_FOLDER));

    // Init logging
    // Is HYPNOS_LOG_LEVEL in environment vars
    let level: LevelFilter = if env::var("CONGA_LOG_LEVEL").is_err() {
        LevelFilter::Info
    } else {
        LevelFilter::from_str(env::var("CONGA_LOG_LEVEL").unwrap().as_str()).unwrap()
    };
    // Create custom config
    let mut config: ConfigBuilder = simplelog::ConfigBuilder::default();
    config.set_time_format_custom(format_description!(
        "[hour]:[minute]:[second] [day]/[month]/[year]"
    ));
    if toml_data.config.write_logs {
        CombinedLogger::init(vec![
            TermLogger::new(
                level,
                config.build(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            WriteLogger::new(
                level,
                config.build(),
                File::create(toml_data.config.write_logs_file.clone()).unwrap(),
            ),
        ])
        .unwrap();
    } else {
        CombinedLogger::init(vec![TermLogger::new(
            level,
            config.build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )])
        .unwrap();
    }

    // Config validation
    debug!("Config loaded:\n{:?}", toml_data.config);

    toml_data
}
