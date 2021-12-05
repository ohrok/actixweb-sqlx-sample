use crate::user::{User, UserRequest};
use actix_web::{get, post, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all).service(create);
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

#[post("/user")]
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
