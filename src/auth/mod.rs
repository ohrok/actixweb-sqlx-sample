use crate::user::User;
use actix_web_httpauth::extractors::basic::BasicAuth;
use anyhow::{anyhow, Result};
use bcrypt::verify;
use sqlx::PgPool;

pub async fn basic_auth_validator(credentials: BasicAuth, pool: &PgPool) -> Result<User> {
    let result = User::find_by_username(credentials.user_id(), pool).await?;
    match result {
        Some(user) => {
            // TODO: unwrap()を使わずに適切なエラーハンドリングをする
            let valid = verify(credentials.password().unwrap().as_bytes(), &user.password);
            if valid.unwrap() {
                Ok(user)
            } else {
                // TODO: actix_web_httpauth::extractors::AuthenticationErrorを使う
                Err(anyhow!("authentication error"))
            }
        }
        None => Err(anyhow!("authentication error")),
    }
}
