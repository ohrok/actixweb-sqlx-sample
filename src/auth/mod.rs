use crate::user::User;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use bcrypt::verify;
use sqlx::PgPool;

pub async fn validate_basic_auth(credentials: BasicAuth, pool: &PgPool) -> Result<User, Error> {
    let password = match credentials.password() {
        Some(password) => password,
        None => {
            return Err(AuthenticationError::from(Config::default()).into());
        }
    };
    let result = User::find_by_username(credentials.user_id(), pool).await;
    match result {
        Ok(Some(user)) => {
            let valid = verify(password.as_bytes(), &user.password).unwrap_or(false);
            if valid {
                Ok(user)
            } else {
                Err(AuthenticationError::from(Config::default()).into())
            }
        }
        _ => Err(AuthenticationError::from(Config::default()).into()),
    }
}
