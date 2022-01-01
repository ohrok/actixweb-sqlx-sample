use crate::user::User;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config as BasicConfig};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config as BearerConfig};
use actix_web_httpauth::extractors::AuthenticationError;
use bcrypt::verify;
use sqlx::PgPool;

pub async fn validate_basic_auth(credentials: BasicAuth, pool: &PgPool) -> Result<User, Error> {
    let config = BasicConfig::default();

    let password = match credentials.password() {
        Some(password) => password,
        None => {
            return Err(AuthenticationError::from(config).into());
        }
    };

    let result = User::find_by_username(credentials.user_id(), pool).await;
    match result {
        Ok(Some(user)) => {
            let valid = verify(password.as_bytes(), &user.password).unwrap_or(false);
            if valid {
                Ok(user)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        // TODO: 401エラー以外も返すようにする
        Ok(None) | Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

pub async fn validate_bearer_auth(credentials: BearerAuth, pool: &PgPool) -> Result<User, Error> {
    let config = BearerConfig::default();
    let result = User::find_by_token(credentials.token(), pool).await;
    match result {
        Ok(Some(user)) => Ok(user),
        Ok(None) | Err(_) => Err(AuthenticationError::from(config).into()),
    }
}
