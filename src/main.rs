use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use sqlx::postgres::PgPoolOptions;
use std::env;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<(), sqlx::Error> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        // TODO: use environment variables.
        .connect("postgres://actix_username:actix_password@localhost:5432/actix_database")
        .await?;

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?; // TODO: use environment variables.

    info!("Starting server");
    server.run().await?;

    Ok(())
}
