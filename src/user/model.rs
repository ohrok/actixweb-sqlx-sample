use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{Done, FromRow, PgPool};
use uuid::Uuid;

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub username: String,
    pub password: String,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub password: String,
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
            SELECT id, name, username, password
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
            password: rec.password,
        })
        .collect();

        Ok(users)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Option<User>> {
        let rec = sqlx::query!(
            r#"
            SELECT id, name, username, password
            FROM users
            WHERE id = $1
            "#,
            id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|rec| User {
            id: rec.id,
            name: rec.name,
            username: rec.username,
            password: rec.password,
        }))
    }

    pub async fn create(user: UserRequest, pool: &PgPool) -> Result<User> {
        let mut tx = pool.begin().await?;
        let user_id = Uuid::new_v4();
        let hashed_password = hash(user.password, DEFAULT_COST)?;

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, username, password)
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            user.name,
            user.username,
            hashed_password,
        )
        .execute(&mut tx)
        .await?;

        let rec = sqlx::query!(
            r#"
            SELECT id, name, username, password
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
            password: rec.password,
        })
    }

    pub async fn update(id: Uuid, user: UserRequest, pool: &PgPool) -> Result<Option<User>> {
        let mut tx = pool.begin().await?;
        let hashed_password = hash(user.password, DEFAULT_COST)?;

        let n_updated = sqlx::query!(
            r#"
            UPDATE users 
            SET name = $1, username = $2, password = $3
            WHERE id = $4
            "#,
            user.name,
            user.username,
            hashed_password,
            id,
        )
        .execute(&mut tx)
        .await?
        .rows_affected();

        if n_updated == 0 {
            return Ok(None);
        }

        let user = sqlx::query!(
            r#"
            SELECT id, name, username, password
            FROM users
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&mut tx)
        .await
        .map(|rec| User {
            id: rec.id,
            name: rec.name,
            username: rec.username,
            password: rec.password,
        })?;

        tx.commit().await?;
        Ok(Some(user))
    }

    pub async fn delete(id: Uuid, pool: &PgPool) -> Result<u64> {
        let mut tx = pool.begin().await?;

        let n_deleted = sqlx::query!(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
            id,
        )
        .execute(&mut tx)
        .await?
        .rows_affected();

        tx.commit().await?;

        Ok(n_deleted)
    }

    pub async fn find_by_post(post_id: Uuid, pool: &PgPool) -> Result<Option<User>> {
        let rec = sqlx::query!(
            r#"
            SELECT users.id, users.name, users.username, users.password
            FROM posts inner join users
            ON posts.user_id = users.id
            WHERE posts.id = $1
            "#,
            post_id,
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|rec| User {
            id: rec.id,
            name: rec.name,
            username: rec.username,
            password: rec.password,
        }))
    }
}
