use crate::user::{User, UserRequest};
use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all).service(find).service(create);
}

#[get("/users")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            error!("error fetching users: {}", err);
            HttpResponse::InternalServerError().body("Error trying to read all users from database")
        }
    }
}

#[get("/users/{id}")]
async fn find(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            error!("error fetching user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to read user from database")
        }
    }
}

#[post("/users")]
async fn create(user: web::Json<UserRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            error!("error creating user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create new user")
        }
    }
}
