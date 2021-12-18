use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
}

#[derive(Serialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
}

impl Responder for Post {
    type Error = Error;
    type Future = HttpResponse;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        HttpResponse::Ok().json(&self)
    }
}

impl Post {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Post>> {
        let posts = sqlx::query!(
            r#"
            SELECT id, title, body, user_id
            FROM posts
            "#
        )
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|rec| Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            user_id: rec.user_id,
        })
        .collect();

        Ok(posts)
    }

    pub async fn find_by_id(id: Uuid, pool: &PgPool) -> Result<Option<Post>> {
        let rec = sqlx::query!(
            r#"
            SELECT id, title, body, user_id
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(rec.map(|rec| Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            user_id: rec.user_id,
        }))
    }
}
