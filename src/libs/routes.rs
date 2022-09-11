use actix_web::{
    error, get, post,
    web::{self},
    Error, HttpResponse,
};
use chrono::Utc;
use futures_util::StreamExt as _;
use log::debug;

use crate::libs::structs::{AppState, Item, Meta, WebError, WebHealth};

const MAX_PAYLOAD_SIZE: usize = 262_144; // Max size of 256k

fn push_new_item(data: web::Data<AppState>, payload_item: Item) {
    let mut states = data.item_queue.lock().unwrap();
    // TODO: This needs validation
    states.push(payload_item);
}

fn generate_metadata() -> Meta {
    Meta {
        received_epoch: Utc::now().timestamp(),
    }
}

#[get("/health")]
async fn health(data: web::Data<AppState>) -> HttpResponse {
    debug!("Health request received");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}

#[post("/item")]
async fn add_item(
    data: web::Data<AppState>,
    mut payload: web::Payload,
) -> Result<HttpResponse, Error> {
    debug!("Item create/ingest request received");

    // Convert payload stream into useful object
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        if (body.len() + chunk.len()) > MAX_PAYLOAD_SIZE {
            return Err(error::ErrorBadRequest("payload overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    let mut item = match serde_json::from_slice::<Item>(&body) {
        Ok(n) => n,
        Err(e) => {
            return Ok(HttpResponse::BadRequest()
                .content_type("application/json")
                .json(WebError {
                    timestamp: Utc::now().to_rfc3339(),
                    error: format!("failed to parse json. {}", e),
                }));
        }
    };

    item.meta = Some(generate_metadata());
    push_new_item(data, item);

    Ok(HttpResponse::NoContent().finish())
}

#[get("/items/preview/{recipient_secret}")]
async fn get_items(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    debug!("Item get all request received");

    let rs_query = path.into_inner();

    let items = data.item_queue.lock().unwrap();
    let mut filtered_items: Vec<Item> = Vec::new();

    for item in items.iter() {
        if item.recipient_secret == rs_query {
            filtered_items.push(item.clone());
        }
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(filtered_items))
}

#[get("/items/{recipient_secret}")]
async fn fetch_items(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    debug!("Item fetch request received");

    let rs_query = path.into_inner();

    let mut items_cleanup = Vec::<usize>::new();
    let mut items = data.item_queue.lock().unwrap();
    let mut return_items: Vec<Item> = Vec::new();

    // Get items to return and a list of items to cleanup from shared queue
    for (i, item) in items.iter().enumerate() {
        if item.recipient_secret == rs_query {
            items_cleanup.push(i);
            return_items.push(item.clone());
        }
    }

    // Reverse state_cleanup list to resolve indexing issues due to shifting indexes
    items_cleanup.reverse();
    // If state found, cleanup and respond
    if !items_cleanup.is_empty() && !return_items.is_empty() {
        for target in items_cleanup {
            items.remove(target);
        }
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .json(return_items))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}
