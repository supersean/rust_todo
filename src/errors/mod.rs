use actix_web::HttpResponse;

#[derive(Debug)]
pub enum ServerError {
    SeanError,
    DieselError(diesel::result::Error),
    UserError(String),
    SerializationError(serde_json::Error),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::SeanError => write!(f, "A sean error occurred"),
            ServerError::DieselError(error) => {
                write!(f, "A diesel error occurred: {:#?}", error.to_string())
            }
            ServerError::UserError(data) => {
                write!(f, "A user error occurred for {}", data)
            }
            ServerError::SerializationError(e) => {
                write!(f, "A serialization error occurred: {:#?}", e)
            }
        }
    }
}


impl actix_web::error::ResponseError for ServerError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        match self {
            ServerError::SeanError => {
                HttpResponse::InternalServerError().json("Sean error")
            }
            ServerError::DieselError(error) => match error {
                diesel::result::Error::NotFound => {
                    HttpResponse::NotFound().json(error.to_string())
                }
                _ => {
                    HttpResponse::InternalServerError().json(error.to_string())
                }
            },
            ServerError::UserError(data) => {
                HttpResponse::InternalServerError().json(data)
            }
            ServerError::SerializationError(_) => {
                HttpResponse::InternalServerError().json("Serialization Error")
            }
        }
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            _ => ServerError::DieselError(err),
        }
    }
}

impl From<serde_json::Error> for ServerError {
    fn from(value: serde_json::Error) -> Self {
        ServerError::SerializationError(value)
    }
}
