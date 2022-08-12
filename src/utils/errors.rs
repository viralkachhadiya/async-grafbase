use std::fmt;
use std::fmt::{Display, Formatter};

use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Enum, Copy, Debug, Clone)]
pub enum ErrorType {
    Unauthorized,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl Eq for ErrorType {}

impl PartialEq for ErrorType {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ErrorType::Unauthorized => write!(f, "Unauthorized"),
            ErrorType::BadRequest => write!(f, "BadRequest"),
            ErrorType::NotFound => write!(f, "NotFound"),
            ErrorType::InternalServerError => write!(f, "InternalServerError"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, thiserror::Error)]
pub struct Error {
    pub title: String,
    pub status: u16,
    #[serde(rename = "type")]
    pub error_type: ErrorType,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}