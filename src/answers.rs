// == File for app's answers handling == 

use http::StatusCode;
use serde_json::json;

use axum::{
    body::Body,
    response::{IntoResponse, Response}, 
    Json
};



#[derive(Debug)]
pub enum InitializationErrorEnum {
    DBConnectionError,
    DBURLError,
    OpenSitePortError,
    ClosedSitePortError,
    JWTKeyError,
    RegistrationError,
    PortError1,
    PortError2,
    ServerError1,
    ServerError2
}


#[derive(Debug)]
pub struct InitializationError {
    _code: InitializationErrorEnum,
    pub error_msg: String,
}


impl InitializationError {
    pub fn new(code: InitializationErrorEnum) -> InitializationError {
        let error_type = match code {
            InitializationErrorEnum::DBConnectionError => String::from("[ ERR ] DB connection error. Check parameters."),
            InitializationErrorEnum::DBURLError => String::from("[ ERR ] .env must contains DATABASE_URL"),
            InitializationErrorEnum::JWTKeyError => String::from("[ ERR ] .env must contains JWT_SECRET"),
            InitializationErrorEnum::OpenSitePortError => String::from("[ ERR ] .env must contains OPEN_PORT"),
            InitializationErrorEnum::ClosedSitePortError => String::from("[ ERR ] .env must contains CLOSED_PORT"),
            InitializationErrorEnum::RegistrationError => String::from("[ ERR ] .env must contains REGISTRATON_ALLOWED"),
            InitializationErrorEnum::PortError1 => String::from("[ ERR ] your port number for registration page is already in use"),
            InitializationErrorEnum::PortError2 => String::from("[ ERR ] your port number for public API is already in use"),
            InitializationErrorEnum::ServerError1 => String::from("[ ERR ] could not open private server, check your settings"),
            InitializationErrorEnum::ServerError2 => String::from("[ ERR ] could not open public server, check your settings"),
        };

        InitializationError {
            _code: code,
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
            "AUTH ERROR": self.error_msg,
        }));

        (self.status_code, body).into_response()
    }
}


impl AuthError {
    pub fn new(code: StatusCode, err_type: AuthErrorEnum) -> AuthError {
        let error_type = match err_type {
            AuthErrorEnum::MissingHeaderError => String::from("[ ERR ] 空のヘッダーは許可されていません"),
            AuthErrorEnum::MissingHeaderTokenError => String::from("[ ERR ] ヘッダーにJWTトークンを追加してください"),
            AuthErrorEnum::TokenDecodingError => String::from("[ ERR ] トークンをデコードできません"),
            AuthErrorEnum::TokenExpirationError => String::from("[ ERR ] トークンの有効期限が切れています"),
            AuthErrorEnum::TokenAuthError => String::from("[ ERR ] 存在しないユーザートークンの使用"),
            AuthErrorEnum::SignInError => String::from("[ ERR ] 正しいメールアドレス/パスワードフィールドが指定されていません"),
            AuthErrorEnum::UsersNotFoundError => String::from("[ ERR ] 登録ユーザーが見つかりません"),
        };

        AuthError {
            status_code: code,
            error_msg:error_type
        }
    }
}



pub enum ApiAnswersEnum {
    Answer1,
    Answer2,
    Answer3,
    Answer4,
    Answer5,
    Answer6,
    Answer7,
}


pub struct ApiAnswers {
    _name: ApiAnswersEnum,
    pub code: StatusCode,
    pub message: String,
}


impl ApiAnswers {
    pub fn new(answer_name: ApiAnswersEnum) -> ApiAnswers {
        let answer_type = match answer_name {
            ApiAnswersEnum::Answer1 => String::from("[ INF ] データの登録を完了した"),
            ApiAnswersEnum::Answer2 => String::from("[ ERR ] データが既にある"),
            ApiAnswersEnum::Answer3 => String::from("[ ERR ] データが登録不可能になっている"),
            ApiAnswersEnum::Answer4 => String::from("[ INF ] データがDBに存在している"),
            ApiAnswersEnum::Answer5 => String::from("[ INF ] データが見つからない"),
            ApiAnswersEnum::Answer6 => String::from("[ ERR ] データが登録不可能になっている"),
            ApiAnswersEnum::Answer7 => String::from("[ INF ] データが登録可能になっている"),
            
        };

        let answer_code = match answer_name {
            ApiAnswersEnum::Answer1 => StatusCode::OK,
            ApiAnswersEnum::Answer2 => StatusCode::NOT_ACCEPTABLE,
            ApiAnswersEnum::Answer3 => StatusCode::NOT_IMPLEMENTED,
            ApiAnswersEnum::Answer4 => StatusCode::OK,
            ApiAnswersEnum::Answer5 => StatusCode::NOT_FOUND,
            ApiAnswersEnum::Answer6 => StatusCode::NOT_FOUND,
            ApiAnswersEnum::Answer7 => StatusCode::FOUND,
        };

        ApiAnswers {
            _name: answer_name,
            code: answer_code,
            message:answer_type
        }
    }
}