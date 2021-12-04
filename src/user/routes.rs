use actix_web::{get, web, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
}

#[get("/users")]
async fn find_all() -> impl Responder {
    "users"
}
