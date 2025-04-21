// == File to get values from .env file ==

use std::env;
use dotenv::dotenv;

use crate::answers;

pub type InitErrors = answers::InitializationError;


#[warn(dead_code)]
pub struct Enviroment {
    pub database_url: String,
    pub jwt_secret: String,
    pub open_host: String,
    pub closed_host: String,
    pub registration_allowed: String,
}


impl Enviroment {
    pub fn new() -> Enviroment {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| InitErrors::new(answers::InitializationErrorEnum::DBURLError)).unwrap();
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| InitErrors::new(answers::InitializationErrorEnum::JWTKeyError)).unwrap();

        let open_url = String::from("0.0.0.0");
        let open_port = env::var("OPEN_PORT")
            .map_err(|_| InitErrors::new(answers::InitializationErrorEnum::OpenSitePortError)).unwrap();
        let open_host = open_url + ":" + &open_port;

        let closed_url = String::from("127.0.0.1");
        let closed_port = env::var("CLOSED_PORT")
            .map_err(|_| InitErrors::new(answers::InitializationErrorEnum::ClosedSitePortError)).unwrap();
        let closed_host = closed_url + ":" + &closed_port;

        let registration_allowed = env::var("REGISTRATON_ALLOWED")
        .map_err(|_| InitErrors::new(answers::InitializationErrorEnum::RegistrationError)).unwrap();

        Enviroment {
            database_url,
            jwt_secret,
            open_host,
            closed_host,
            registration_allowed
        }
    }
}