use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use anyhow::Result;
use dotenv::dotenv;
use log::info;
use sqlx::PgPool;
use std::env;

mod post;
mod user;

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT")
        .expect("PORT is not set in .env file")
        .parse::<u16>()
        .expect("PORT should be a u16");

    info!("using postgresql database at: {}", &database_url);
    let db_pool = PgPool::connect(&database_url).await?;

    let server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello))
            .configure(user::init) // init user routes
            .configure(post::init)
    })
    .bind((host, port))?;

    info!("Starting server");
    server.run().await?;

    Ok(())
}
