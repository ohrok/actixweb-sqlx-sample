use crate::user::User;
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use serde::Serialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Token {
    pub id: Uuid,
    pub value: String,
    pub user_id: Uuid,
}

impl Responder for Token {
    type Error = Error;
    type Future = HttpResponse;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        HttpResponse::Ok().json(&self)
    }
}

impl Token {
    pub fn new(user_id: Uuid) -> Token {
        // TODO: トークンとしてのvalue生成
        Token {
            id: Uuid::new_v4(),
            value: String::from("token value"),
            user_id: user_id,
        }
    }

    pub async fn create(user: &User, pool: &PgPool) -> Result<Token> {
        let token = Token::new(user.id);

        let mut tx = pool.begin().await?;
        sqlx::query!(
            r#"
            INSERT INTO tokens (id, value, user_id)
            VALUES ($1, $2, $3)
            "#,
            token.id,
            token.value,
            token.user_id,
        )
        .execute(&mut tx)
        .await?;

        let rec = sqlx::query!(
            r#"
            SELECT id, value, user_id
            FROM tokens
            WHERE id = $1
            "#,
            token.id,
        )
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;

        Ok(Token {
            id: rec.id,
            value: rec.value,
            user_id: rec.user_id,
        })
    }
}