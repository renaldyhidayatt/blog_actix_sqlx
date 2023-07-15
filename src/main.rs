mod abstract_trait;
mod config;
mod domain;
mod middleware;
mod models;
mod repository;
mod service;
mod service_register;

mod handler;

use crate::handler::router_config;
use crate::service_register::ServiceRegister;
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{http::header, App, HttpServer};
use config::{Config, ConnectionManager};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();
    let config = Config::init();

    let db_pool = ConnectionManager::new_pool(&config.database_url, config.run_migrations)
        .await
        .expect("Error initializing database connection pool");

    let port = config.port;
    let service_register = ServiceRegister::new(db_pool, config);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .configure(router_config)
            .app_data(Data::new(service_register.clone()))
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
    .expect("Failed");

    Ok(())
}
