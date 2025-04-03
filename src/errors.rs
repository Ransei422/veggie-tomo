use axum::{body::Body,
    response::{IntoResponse, Response}, 
    Json
};
use http::StatusCode;
use serde_json::json;

#[allow(dead_code)]
#[derive(Debug)]
pub enum InitializationErrorEnum {
    DBConnectionError,
    DBURLError,
    OpenSitePortError,
    ClosedSitePortError,
    JWTKeyError,
    RegistrationError,
}


#[allow(dead_code)]
#[derive(Debug)]
pub struct InitializationError {
    code: InitializationErrorEnum,
    pub error_msg: String,
}


#[allow(dead_code)]
impl InitializationError {
    pub fn new(code: InitializationErrorEnum) -> InitializationError {
        let error_type = match code {
            InitializationErrorEnum::DBConnectionError => String::from("[ ERR ] DB connection error. Check parameters."),
            InitializationErrorEnum::DBURLError => String::from("[ ERR ] .env must contains DATABASE_URL"),
            InitializationErrorEnum::JWTKeyError => String::from("[ ERR ] .env must contains JWT_SECRET"),
            InitializationErrorEnum::OpenSitePortError => String::from("[ ERR ] .env must contains OPEN_PORT"),
            InitializationErrorEnum::ClosedSitePortError => String::from("[ ERR ] .env must contains CLOSED_PORT"),
            InitializationErrorEnum::RegistrationError => String::from("[ ERR ] .env must contains REGISTRATON_ALLOWED"),
        };

        InitializationError {
            code,
            error_msg:error_type
        }
    }
}


impl std::fmt::Display for InitializationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.error_msg)
    }
}



pub enum AuthErrorEnum {
    MissingHeaderError,
    MissingHeaderTokenError,
    TokenDecodingError,
    TokenExpirationError,
    TokenAuthError,
    SignInError,
    UsersNotFoundError
}


pub struct AuthError {
    pub status_code: StatusCode,
    pub error_msg: String,
}


impl IntoResponse for AuthError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "API ERROR": self.error_msg,
        }));

        (self.status_code, body).into_response()
    }
}


#[allow(dead_code)]
impl AuthError {
    pub fn new(code: StatusCode, err_type: AuthErrorEnum) -> AuthError {
        let error_type = match err_type {
            AuthErrorEnum::MissingHeaderError => String::from("Empty header is not allowed"),
            AuthErrorEnum::MissingHeaderTokenError => String::from("Please add the JWT token to the header"),
            AuthErrorEnum::TokenDecodingError => String::from("Unable to decode token"),
            AuthErrorEnum::TokenExpirationError => String::from("Token is expired"),
            AuthErrorEnum::TokenAuthError => String::from("Using non-existing user token"),
            AuthErrorEnum::SignInError => String::from("No right email/password field provided"),
            AuthErrorEnum::UsersNotFoundError => String::from("No registered user found"),
        };

        AuthError {
            status_code: code,
            error_msg:error_type
        }
    }
}
