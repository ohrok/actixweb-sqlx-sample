use crate::post::{Post, PostRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all)
        .service(find)
        .service(create)
        .service(update)
        .service(delete);
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

#[post("/posts")]
async fn create(post: web::Json<PostRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Post::create(post.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => {
            error!("error creating post: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create new post")
        }
    }
}

#[put("/posts/{id}")]
async fn update(
    id: web::Path<Uuid>,
    post: web::Json<PostRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = Post::update(id.into_inner(), post.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(post)) => HttpResponse::Ok().json(post),
        Ok(None) => HttpResponse::NotFound().body("Post not found"),
        Err(err) => {
            error!("error updating post: {}", err);
            HttpResponse::InternalServerError().body("Error trying to update post")
        }
    }
}

#[delete("/posts/{id}")]
async fn delete(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Post::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                let msg = format!("Successfully deleted {} record(s)", rows_deleted);
                HttpResponse::Ok().body(msg)
            } else {
                HttpResponse::NotFound().body("Post not found")
            }
        }
        Err(err) => {
            error!("error deleting post: {}", err);
            HttpResponse::InternalServerError().body("Error trying to delete post")
        }
    }
}
