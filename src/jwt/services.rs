use serde::{Serialize, Deserialize};



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