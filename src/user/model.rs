use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub username: String,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}

// implementation of Actix Responder for User struct so we can return User from action handler
impl Responder for User {
    type Error = Error;
    type Future = HttpResponse;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        HttpResponse::Ok().json(&self)
    }
}

impl User {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<User>> {
        let users = sqlx::query!(
            r#"
            SELECT id, name, username
            FROM users
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| User {
            id: rec.id,
            name: rec.name,
            username: rec.username,
        })
        .collect();

        Ok(users)
    }

    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<User> {
        let mut tx = pool.begin().await?;
        let user_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, username)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            user.name,
            user.username,
        )
        .execute(&mut tx)
        .await?;

        let rec = sqlx::query!(
            r#"
            SELECT id, name, username
            FROM users
            WHERE id = $1
            "#,
            user_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(User {
            id: rec.id,
            name: rec.name,
            username: rec.username,
        })
    }
}
