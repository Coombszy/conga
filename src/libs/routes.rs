use actix_web::{
    error, get, post,
    web::{self},
    Error, HttpResponse,
};
use chrono::Utc;
use futures_util::StreamExt as _;
use log::debug;

use crate::libs::{
    middleware::Auth,
    structs::{AppState, Item, Meta, WebError, WebHealth},
};

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

/// Check health of service 
/// 
/// Checks the health of the service as well as include uptime
#[utoipa::path(
    responses(
        (status = 200, description = "Contains service uptime", body = WebHealth)
    )
)]
#[get("/health")]
async fn health(data: web::Data<AppState>) -> HttpResponse {
    debug!("Health request received");
    HttpResponse::Ok()
        .content_type("application/json")
        .json(WebHealth {
            uptime: data.uptime(),
        })
}

/// Validate auth
///
/// Allows checking if an API key is authorized
#[utoipa::path(
    responses(
        (status = 204, description = "API is valid"),
        (status = 401, description = "API is not valid")
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/auth", wrap = "Auth")]
async fn auth() -> Result<HttpResponse, Error> {
    debug!("Auth request received");
    Ok(HttpResponse::NoContent().finish())
}

/// Add item
/// 
/// Add item to a target queue
#[utoipa::path(
    responses(
        (status = 204, description = "Successfully added item to queue"),
        (status = 401, description = "Not authorized"),
        (status = 400, description = "Bad request")
    ),
    security(
        ("api_key" = [])
    )
    
)]
#[post("/item", wrap = "Auth")]
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

/// Preview item queue
/// 
/// Preview items in a queue, without ingesting them
#[utoipa::path(
    responses(
        (status = 200, description = "Items currently in queue", body = [Item]),
        (status = 401, description = "Not authorized"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("queue" = String, Path, description = "Target queue")
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/items/preview/{queue}", wrap = "Auth")]
async fn get_items(
    data: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<HttpResponse, Error> {
    debug!("Item get all request received");

    let rs_query = path.into_inner();

    let items = data.item_queue.lock().unwrap();
    let mut filtered_items: Vec<Item> = Vec::new();

    for item in items.iter() {
        if item.queue == rs_query {
            filtered_items.push(item.clone());
        }
    }

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(filtered_items))
}

/// Fetch item queue
/// 
/// Fetch items from a queue. This will ingest them
#[utoipa::path(
    responses(
        (status = 200, description = "Items fetched from queue", body = [Item]),
        (status = 401, description = "Not authorized"),
        (status = 400, description = "Bad request")
    ),
    params(
        ("queue" = String, Path, description = "Target queue")
    ),
    security(
        ("api_key" = [])
    )
)]
#[get("/items/{queue}", wrap = "Auth")]
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
        if item.queue == rs_query {
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
