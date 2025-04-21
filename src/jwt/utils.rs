// == File for JWT helpers ==

use std::sync::Arc;
use serde::{
    Serialize,
    Deserialize
};
use bcrypt::{
    hash,
    verify,
    DEFAULT_COST
};
use chrono::{
    Duration,
    Utc
};
use jsonwebtoken::{
    decode,
    encode,
    DecodingKey,
    EncodingKey,
    Header,
    TokenData,
    Validation
};


use crate::state::AppState;



#[derive(Serialize, Deserialize)]
pub struct Cliams {
    pub exp: usize,
    pub email: String,
}



#[derive(Clone)]
pub struct DBUser {
    pub email: String,
    pub password_hash: String
}



#[derive(Deserialize)]
pub struct SignInData {
    pub email: Option<String>,
    pub password: Option<String>,
}



pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}



pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    let hash = hash(password, DEFAULT_COST)?;
    Ok(hash)
}



pub fn encode_jwt(email: String, jwt_secret: String) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::hours(24);
    let exp: usize = (now + expire).timestamp() as usize;
    let claim = Cliams { exp, email };

    encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}



pub fn decode_jwt(jwt: String, jwt_secret: String) -> Result<TokenData<Cliams>, jsonwebtoken::errors::Error> {
    let result = decode(
        &jwt,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    );
    result
}



pub async fn retrieve_all_users(app_state: &Arc<AppState>) -> Vec<Option<DBUser>> {
    let pool = &app_state.db_pool;

    let rows = sqlx::query!("SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap_or_else(|_| vec![]);

    let mut all_users: Vec<Option<DBUser>> = vec![];

    for user in rows {
        let current_user: DBUser = DBUser {
            email: user.email,
            password_hash: user.password_hash
        };
        all_users.push(Some(current_user))
    }

    all_users
}