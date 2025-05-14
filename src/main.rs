use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::{info, warn};
use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok",
        "message": "Hello from Rust on Cloud Foundry!"
    }))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    }))
}

#[get("/env")]
async fn environment() -> impl Responder {
    let vcap_application = env::var("VCAP_APPLICATION").unwrap_or_else(|_| "Not found".to_string());
    let vcap_services = env::var("VCAP_SERVICES").unwrap_or_else(|_| "Not found".to_string());
    
    HttpResponse::Ok().json(serde_json::json!({
        "vcap_application": vcap_application,
        "vcap_services": vcap_services,
        "environment_variables": env::vars()
            .filter(|(k, _)| k.starts_with("CF_") || k == "PORT" || k == "HOME")
            .collect::<Vec<(String, String)>>()
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    // Get port from environment variable or use default
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT must be a number");
    
    info!("Starting server on port {}", port);
    
    // Create the server and bind to all interfaces
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(health)
            .service(environment)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}