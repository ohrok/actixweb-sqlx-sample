use crate::user::User;
use actix_web::{get, web, HttpResponse, Responder};
use log::error;
use sqlx::PgPool;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
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
