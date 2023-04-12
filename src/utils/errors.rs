use actix_web::{error::HttpError, HttpResponse, ResponseError};
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("email or username already in use")]
    EmailAlreadyUsed(),
    #[error("data store disconnected")]
    NoUserFinde,
    #[error("invalid token")]
    InvalidToken,
    #[error("invalid password")]
    InvalidPassword(#[from] argon2::Error),
    #[error("decode")]
    DecoderError(#[from] base64::DecodeError),
    #[error("invalid token")]
    InvalidJwt(#[from] jwt::Error),
    #[error("invalide base64")]
    InvalideBase64(#[from] std::str::Utf8Error),
    #[error("database connection error")]
    ConnectionErrors(#[from] sqlx::Error),
    #[error("database migration errors")]
    MigrateErrors(#[from] sqlx::migrate::MigrateError),
    #[error("invalid secret lenght")]
    InvalidSecretLenght(#[from] sha2::digest::InvalidLength),
    #[error("http errors")]
    HttpError(#[from] HttpError),
    #[error("config loading error")]
    ConfigError,
    #[error("invalid request {0} not found")]
    NotFound(String),
    #[error("invalid request {0} already exists")]
    AlreadyExists(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("orm errors")]
    OrmError(#[from] DbErr),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppErrorsResponse {
    pub status_code: u16,
    pub types: String,
    pub message: String,
    pub trace: Option<String>,
}

impl AppErrorsResponse {
    pub fn new(error: &AppErrors) -> Self {
        let types = format!("{:?}", error);
        let message = error.to_string();
        let trace = None;
        match error {
            AppErrors::EmailAlreadyUsed() => Self {
                status_code: 400,
                types,
                message,
                trace,
            },
            AppErrors::NoUserFinde => Self {
                status_code: 400,
                types,
                message,
                trace,
            },
            AppErrors::InvalidToken => Self {
                status_code: 400,
                types,
                message,
                trace,
            },
            AppErrors::InvalidPassword(err) => Self {
                status_code: 400,
                types: "InvalidPassword".to_string(),
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::DecoderError(err) => Self {
                status_code: 400,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::InvalidJwt(err) => Self {
                status_code: 400,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::InvalideBase64(err) => Self {
                status_code: 400,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::ConnectionErrors(err) => Self {
                status_code: 500,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::MigrateErrors(err) => Self {
                status_code: 500,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::InvalidSecretLenght(err) => Self {
                status_code: 500,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::HttpError(err) => Self {
                status_code: 500,
                types,
                message,
                trace: if cfg!(debug_assertions) {
                    Some(format!("{:?}", err))
                } else {
                    None
                },
            },
            AppErrors::ConfigError => Self {
                status_code: 500,
                types,
                message,
                trace,
            },
            AppErrors::NotFound(_) => Self {
                status_code: 404,
                types,
                message,
                trace,
            },
            AppErrors::AlreadyExists(_) => Self {
                status_code: 400,
                types,
                message,
                trace,
            },
            AppErrors::Unauthorized => Self {
                status_code: 401,
                types,
                message,
                trace,
            },
            AppErrors::OrmError(_) => Self {
                status_code: 500,
                types,
                message,
                trace,
            },
        }
    }
}

impl ResponseError for AppErrors {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let response = AppErrorsResponse::new(self);
        HttpResponse::build(actix_web::http::StatusCode::from_u16(response.status_code).unwrap())
            .json(response)
    }
}
