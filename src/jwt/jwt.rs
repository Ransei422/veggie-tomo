use std::sync::Arc;
use axum::{
    body::Body, extract::{Json, Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
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
use tracing::{error, info};

use crate::{
    answers::AuthErrorEnum,
    answers::AuthError,
    state::AppState};

use super::services::*;



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


pub async fn sign_in_page(
    State(app_state): State<Arc<AppState>>, 
    Json(user_data): Json<SignInData>,
) -> Result<Json<String>, AuthError> {

    let jwt_secret = app_state.jwt_secret.clone();
    let db_users = retrieve_all_users(&app_state).await;

    if db_users.len() == 0 {
        let response_error = AuthError::new(StatusCode::NOT_FOUND,
            AuthErrorEnum::UsersNotFoundError);

        error!("[ ERR-1 ] `sign_in_page` response ERR: {}", response_error.error_msg);
        return Err(response_error);
    }

    let user_data_email = match user_data.email {
        Some(d) => d,
        None => {
            let response_error = AuthError::new(StatusCode::NOT_ACCEPTABLE,
                AuthErrorEnum::SignInError);

            error!("[ ERR-2 ] `sign_in_page` response ERR: {}", response_error.error_msg);
            return Err(response_error)
        }
    };

    let user_data_password = match user_data.password {
        Some(d) => d,
        None => {
            let response_error = AuthError::new(StatusCode::NOT_ACCEPTABLE,
                AuthErrorEnum::SignInError);

            error!("[ ERR-3 ] `sign_in_page` response ERR: {}", response_error.error_msg);
            return Err(response_error)
        }
    };

    let matching_user = db_users
        .iter()
        .filter_map(|user| user.as_ref()) 
        .find(|user| user.email == *user_data_email);

    if let Some(user) = matching_user {
        if verify_password(&user_data_password, &user.password_hash).unwrap_or(false) {

            let token = encode_jwt(user.email.clone(), jwt_secret)
                .map_err(|_| {
                    let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                        AuthErrorEnum::TokenAuthError);
                    
                    error!("[ ERR-4 ] `sign_in_page` response ERR: {}", response_error.error_msg);
                    response_error
                    })?;

            info!("[ INF ] `sign_in_page` response OK");
            return Ok(Json(token))
        } else {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                AuthErrorEnum::TokenAuthError);
            
            error!("[ ERR-5 ] `sign_in_page` response ERR: {}", response_error.error_msg);
            return Err(response_error);
        }
    } else {
        let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
            AuthErrorEnum::TokenAuthError);

        error!("[ ERR-6 ] `sign_in_page` response ERR: {}", response_error.error_msg); 
        return Err(response_error);
    }
}


pub async fn authorize(
    State(app_state): State<Arc<AppState>>,
    mut req: Request,
    next: Next
) -> Result<Response<Body>, AuthError> {

    let jwt_secret = app_state.jwt_secret.clone();

    let auth_header = req.headers_mut().get(http::header::AUTHORIZATION);

    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED, 
                AuthErrorEnum::MissingHeaderError);
            
            error!("[ ERR-1 ] `authorize` response ERR: {}", response_error.error_msg);
            response_error
        })?,
        
        None => {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                AuthErrorEnum::MissingHeaderTokenError);
            
            error!("[ ERR-2 ] `authorize` response ERR: {}", response_error.error_msg);
            return Err(response_error)},
    };

    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    let token_data = match decode_jwt(token.unwrap().to_string(), jwt_secret) {
        Ok(data) => data,
        Err(_) => {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                AuthErrorEnum::TokenDecodingError);
            
            error!("[ ERR-3 ] `authorize` response ERR: {}", response_error.error_msg);
            return Err(response_error);},
    };
    
    let token_user = token_data.claims;

    if token_user.exp < Utc::now().timestamp() as usize {
        let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
            AuthErrorEnum::TokenExpirationError);
        
        error!("[ ERR-4 ] `authorize` response ERR: {}", response_error.error_msg);
        return Err(response_error);
    }

    let db_users = retrieve_all_users(&app_state).await;

    let matching_user = db_users
        .iter()
        .filter_map(|user| user.as_ref()) 
        .find(|user| user.email == *token_user.email);

    if let Some(user) = matching_user {
        req.extensions_mut().insert(user.clone());

        info!("[ INF ] `authorize` response OK");
        return Ok(next.run(req).await);
    } else {
        let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
            AuthErrorEnum::TokenAuthError);
        
        error!("[ ERR-5 ] `authorize` response ERR: {}", response_error.error_msg);
        return Err(response_error);
    }

}


async fn retrieve_all_users(app_state: &Arc<AppState>) -> Vec<Option<DBUser>> {
    // Reason for getting all users from DB is to not let users create malicious SQL queries.
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