use crate::post::Post;
use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all).service(find);
}

#[get("/posts")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Post::find_all(db_pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => {
            error!("error fetching posts: {}", err);
            HttpResponse::InternalServerError().body("Error trying to read all posts from database")
        }
    }
}

#[get("/posts/{id}")]
async fn find(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Post::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().body("Post not found"),
        Err(err) => {
            error!("error fetching post: {}", err);
            HttpResponse::InternalServerError().body("Error trying to read post from database")
        }
    }
}
