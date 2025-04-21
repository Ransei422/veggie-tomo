// == File for JWT processing ==

use std::sync::Arc;
use chrono::Utc;
use tracing::{
    error,
    info
};
use axum::{
    body::Body, extract::{Json, Request, State},
    http::{self, Response, StatusCode},
    middleware::Next,
};


use crate::{
    answers::AuthErrorEnum,
    answers::AuthError,
    state::AppState
};


use super::utils;



pub async fn sign_in_page(
    State(app_state): State<Arc<AppState>>, 
    Json(user_data): Json<utils::SignInData>,
) -> Result<Json<String>, AuthError> {

    let jwt_secret = app_state.jwt_secret.clone();
    let db_users = utils::retrieve_all_users(&app_state).await;

    if db_users.len() == 0 {
        let response_error = AuthError::new(StatusCode::NOT_FOUND,
            AuthErrorEnum::UsersNotFoundError
        );

        error!(
            code = "ERR-1",
            endpoint = "sign_in_page",
            status = ?response_error.status_code,
            answer = response_error.error_msg,
            server_message = "No Database Users registered"
        );
        return Err(response_error);
    }

    let user_data_email = match user_data.email {
        Some(data) => data,
        None => {
            let response_error = AuthError::new(StatusCode::NOT_ACCEPTABLE,
                AuthErrorEnum::SignInError
            );

            error!(
                code = "ERR-2",
                endpoint = "sign_in_page",
                status = ?response_error.status_code,
                answer = response_error.error_msg,
                server_message = "No User email match"
            );
            return Err(response_error)
        }
    };

    let user_data_password = match user_data.password {
        Some(data) => data,
        None => {
            let response_error = AuthError::new(StatusCode::NOT_ACCEPTABLE,
                AuthErrorEnum::SignInError
            );

            error!(
                code = "ERR-3",
                endpoint = "sign_in_page",
                status = ?response_error.status_code,
                answer = response_error.error_msg,
                server_message = "No User password match"
            );
            return Err(response_error)
        }
    };

    let matching_user = db_users
        .iter()
        .filter_map(|user| user.as_ref()) 
        .find(|user| user.email == *user_data_email);

    if let Some(user) = matching_user {
        if utils::verify_password(&user_data_password, &user.password_hash).unwrap_or(false) {

            let token = utils::encode_jwt(user.email.clone(), jwt_secret)
                .map_err(|_| {
                    let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                        AuthErrorEnum::TokenAuthError
                    );
                    
                    error!(
                        code = "ERR-4",
                        endpoint = "sign_in_page",
                        status = ?response_error.status_code,
                        answer = response_error.error_msg,
                        server_message = "Could not encode JWT token"
                    );
                    response_error
                })?;

            info!(
                code = "INF",
                endpoint = "sign_in_page",
                status = ?StatusCode::OK,
                answer = "sign_in_page responded",
                server_message = "OK"
            );
            return Ok(Json(token))
        } else {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                AuthErrorEnum::TokenAuthError
            );
            
            error!(
                code = "ERR-5",
                endpoint = "sign_in_page",
                status = ?response_error.status_code,
                answer = response_error.error_msg,
                server_message = "Password is wrong"
            );
            return Err(response_error);
        }
    } else {
        let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
            AuthErrorEnum::TokenAuthError
        );

        error!(
            code = "ERR-6",
            endpoint = "sign_in_page",
            status = ?response_error.status_code,
            answer = response_error.error_msg,
            server_message = "No matching user"
        );
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
                AuthErrorEnum::MissingHeaderError
            );

            error!(
                code = "ERR-1",
                endpoint = "authorize",
                status = ?response_error.status_code,
                answer = response_error.error_msg,
                server_message = "Error occurred during header authorization - No Header"
            );
            response_error
        })?,
        
        None => {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                AuthErrorEnum::MissingHeaderTokenError
            );
            
            error!(
                code = "ERR-2",
                endpoint = "authorize",
                status = ?response_error.status_code,
                answer = response_error.error_msg,
                server_message = "Error occurred during header authorization - No Token"
            );
            return Err(response_error)},
    };

    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    let token_data = match utils::decode_jwt(token.unwrap().to_string(), jwt_secret) {
        Ok(data) => data,
        Err(_) => {
            let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
                AuthErrorEnum::TokenDecodingError
            );
            
            error!(
                code = "ERR-3",
                endpoint = "authorize",
                status = ?response_error.status_code,
                answer = response_error.error_msg,
                server_message = "Could not decode JWT token"
            );
            return Err(response_error);},
    };
    
    let token_user = token_data.claims;

    if token_user.exp < Utc::now().timestamp() as usize {
        let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
            AuthErrorEnum::TokenExpirationError
        );
        
        error!(
            code = "ERR-4",
            endpoint = "authorize",
            status = ?response_error.status_code,
            answer = response_error.error_msg,
            server_message = "Using expired token"
        );
        return Err(response_error);
    }

    let db_users = utils::retrieve_all_users(&app_state).await;

    let matching_user = db_users
        .iter()
        .filter_map(|user| user.as_ref()) 
        .find(|user| user.email == *token_user.email);

    if let Some(user) = matching_user {
        req.extensions_mut().insert(user.clone());

        info!(
            code = "INF",
            endpoint = "authorize",
            status = ?StatusCode::OK,
            answer = "authorize responded",
            server_message = "OK"
        );
        return Ok(next.run(req).await);
    } else {
        let response_error = AuthError::new(StatusCode::UNAUTHORIZED,
            AuthErrorEnum::TokenAuthError
        );
        
        error!(
            code = "ERR-5",
            endpoint = "authorize",
            status = ?response_error.status_code,
            answer = response_error.error_msg,
            server_message = "No matching user"
        );
        return Err(response_error);
    }

}