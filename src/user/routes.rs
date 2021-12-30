use crate::auth;
use crate::post::Post;
use crate::token::Token;
use crate::user::{User, UserPublic, UserRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use log::error;
use sqlx::PgPool;
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all)
        .service(find)
        .service(create)
        .service(update)
        .service(delete)
        .service(find_posts)
        .service(login);
}

#[get("/users")]
async fn find_all(db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_all(db_pool.get_ref()).await;
    match result {
        Ok(users) => {
            let users = users
                .into_iter()
                .map(|user| UserPublic::from(user))
                .collect::<Vec<UserPublic>>();
            HttpResponse::Ok().json(users)
        }
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
        Ok(Some(user)) => HttpResponse::Ok().json(UserPublic::from(user)),
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
        Ok(user) => HttpResponse::Ok().json(UserPublic::from(user)),
        Err(err) => {
            error!("error creating user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create new user")
        }
    }
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<Uuid>,
    user: web::Json<UserRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let result = User::update(id.into_inner(), user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(UserPublic::from(user)),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            error!("error updating user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to update user")
        }
    }
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::delete(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(rows_deleted) => {
            if rows_deleted > 0 {
                let msg = format!("Successfully deleted {} record(s)", rows_deleted);
                HttpResponse::Ok().body(msg)
            } else {
                HttpResponse::NotFound().body("User not found")
            }
        }
        Err(err) => {
            error!("error deleting user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to delete user")
        }
    }
}

#[get("/users/{id}/posts")]
async fn find_posts(id: web::Path<Uuid>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = Post::find_by_user(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => {
            error!("error fetching posts by this user: {}", err);
            HttpResponse::InternalServerError()
                .body("Error trying to read posts by this user from database")
        }
    }
}

#[post("/users/login")]
async fn login(credentials: BasicAuth, db_pool: web::Data<PgPool>) -> impl Responder {
    let user = match auth::validate_basic_auth(credentials, db_pool.get_ref()).await {
        Ok(user) => user,
        // TODO: 401エラー以外も返すようにする
        Err(err) => {
            error!("Unauthorized error: {}", err);
            return HttpResponse::from_error(err);
        }
    };

    let token = Token::create(&user, db_pool.get_ref()).await;
    match token {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => {
            error!("error creating token: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create new token")
        }
    }
}
