use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{Done, FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct PostRequest {
    pub title: String,
    pub body: String,
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
            id,
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

    pub async fn create(post: PostRequest, user_id: Uuid, pool: &PgPool) -> Result<Post> {
        let post_id = Uuid::new_v4();

        let mut tx = pool.begin().await?;

        sqlx::query!(
            r#"
            INSERT INTO posts (id, title, body, user_id)
            VALUES ($1, $2, $3, $4)
            "#,
            post_id,
            post.title,
            post.body,
            user_id,
        )
        .execute(&mut tx)
        .await?;

        let rec = sqlx::query!(
            r#"
            SELECT id, title, body, user_id
            FROM posts
            WHERE id = $1
            "#,
            post_id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            user_id: rec.user_id,
        })
    }

    pub async fn update(
        id: Uuid,
        post: PostRequest,
        user_id: Uuid,
        pool: &PgPool,
    ) -> Result<Option<Post>> {
        let mut tx = pool.begin().await?;

        let n_updated = sqlx::query!(
            r#"
            UPDATE posts 
            SET title = $1, body = $2
            WHERE id = $3 AND user_id = $4
            "#,
            post.title,
            post.body,
            id,
            user_id,
        )
        .execute(&mut tx)
        .await?
        .rows_affected();

        if n_updated == 0 {
            return Ok(None);
        }

        let post = sqlx::query!(
            r#"
            SELECT id, title, body, user_id
            FROM posts
            WHERE id = $1
            "#,
            id,
        )
        .fetch_one(&mut tx)
        .await
        .map(|rec| Post {
            id: rec.id,
            title: rec.title,
            body: rec.body,
            user_id: rec.user_id,
        })?;

        tx.commit().await?;

        Ok(Some(post))
    }

    pub async fn delete(id: Uuid, user_id: Uuid, pool: &PgPool) -> Result<u64> {
        let mut tx = pool.begin().await?;

        let n_deleted = sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id,
        )
        .execute(&mut tx)
        .await?
        .rows_affected();

        tx.commit().await?;

        Ok(n_deleted)
    }

    pub async fn find_by_user(user_id: Uuid, pool: &PgPool) -> Result<Vec<Post>> {
        let posts = sqlx::query!(
            r#"
            SELECT id, title, body, user_id
            FROM posts
            WHERE user_id = $1
            "#,
            user_id,
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
}
