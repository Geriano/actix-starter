#[macro_use]
extern crate actix_web;

use sea_orm::Database;

mod api;
mod app;
mod common;
mod controllers;
mod dao;
mod middlewares;
mod models;
mod requests;
mod responses;
mod route;
mod services;
mod types;

#[cfg(not(feature = "shuttle"))]
#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    dotenv::dotenv().ok();

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let db = Database::connect(std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    HttpServer::new(move || App::new().configure(app::configure(db.clone())))
        .workers(4)
        .bind((host, port.parse().unwrap()))?
        .run()
        .await
}

#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] store: shuttle_secrets::SecretStore,
) -> shuttle_actix_web::ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let db = Database::connect(store.get("DATABASE_URL").unwrap())
        .await
        .unwrap();

    Ok(app::configure(db).into())
}
