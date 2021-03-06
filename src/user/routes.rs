use crate::auth;
use crate::post::Post;
use crate::token::Token;
use crate::user::{PasswordRequest, User, UserPostRequest, UserPublic, UserPutRequest};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bcrypt::verify;
use log::error;
use sqlx::PgPool;
use uuid::Uuid;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all)
        .service(find)
        .service(create)
        .service(update)
        .service(update_password)
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
async fn create(user: web::Json<UserPostRequest>, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::create(user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(user) => HttpResponse::Ok().json(UserPublic::from(user)),
        Err(err) => {
            error!("error creating user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create new user")
        }
    }
}

#[put("/users")]
async fn update(
    credentials: BearerAuth,
    new_user: web::Json<UserPutRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let user = match auth::validate_bearer_auth(credentials, db_pool.get_ref()).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::from_error(err);
        }
    };

    let result = User::update(user.id, new_user.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(UserPublic::from(user)),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            error!("error updating user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to update user")
        }
    }
}

#[put("/users/password")]
async fn update_password(
    credentials: BearerAuth,
    password: web::Json<PasswordRequest>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let user = match auth::validate_bearer_auth(credentials, db_pool.get_ref()).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::from_error(err);
        }
    };

    let valid = verify(&password.current.as_bytes(), &user.password).unwrap_or(false);
    if !valid {
        return HttpResponse::BadRequest().body("Current password is incorrect");
    }

    let result = User::update_password(user.id, &password.new, db_pool.get_ref()).await;
    match result {
        Ok(Some(user)) => HttpResponse::Ok().json(UserPublic::from(user)),
        Ok(None) => HttpResponse::NotFound().body("User not found"),
        Err(err) => {
            error!("error updating user: {}", err);
            HttpResponse::InternalServerError().body("Error trying to update user")
        }
    }
}

#[delete("/users")]
async fn delete(credentials: BearerAuth, db_pool: web::Data<PgPool>) -> impl Responder {
    let user = match auth::validate_bearer_auth(credentials, db_pool.get_ref()).await {
        Ok(user) => user,
        Err(err) => {
            return HttpResponse::from_error(err);
        }
    };

    let result = User::delete(user.id, db_pool.get_ref()).await;
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
        Err(err) => {
            return HttpResponse::from_error(err);
        }
    };

    let token = Token::create(user.id, db_pool.get_ref()).await;
    match token {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(err) => {
            error!("error creating token: {}", err);
            HttpResponse::InternalServerError().body("Error trying to create new token")
        }
    }
}
