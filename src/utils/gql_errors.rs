use std::fmt;
use std::fmt::{Display, Formatter};

use async_graphql::Error;
use async_graphql::ErrorExtensions;

use super::errors::ErrorType;

#[derive(Debug, thiserror::Error)]
pub enum GqlError {
    NotFound(String),
    InternalServerError(String),
    BadRequest(String),
    Unauthorized(String),
}

impl Display for GqlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ErrorExtensions for GqlError {
    // lets define our base extensions
    fn extend(&self) -> Error {
        match self {
            GqlError::NotFound(reason) => Error::new(format!("{}", reason))
                .extend_with(|_err, e| e.set("code", "404"))
                .extend_with(|_err, e| e.set("type", ErrorType::NotFound)),
            GqlError::InternalServerError(reason) => Error::new(format!("{}", reason))
                .extend_with(|_err, e| e.set("code", "500"))
                .extend_with(|_err, e| e.set("type", ErrorType::InternalServerError)),
            GqlError::BadRequest(reason) => Error::new(format!("{}", reason))
                .extend_with(|_err, e| e.set("code", "400"))
                .extend_with(|_err, e| e.set("type", ErrorType::BadRequest)),
            GqlError::Unauthorized(reason) => Error::new(format!("{}", reason))
                .extend_with(|_err, e| e.set("code", "403"))
                .extend_with(|_err, e| e.set("type", ErrorType::Unauthorized)),
        }
    }
}

/// Returns respective async_gql::Error of given Error.
pub fn get_gql_error(e: super::errors::Error) -> async_graphql::Error {
    return match e.status {
        403 => GqlError::Unauthorized(e.title).extend(),
        400 => GqlError::BadRequest(e.title).extend(),
        404 => GqlError::NotFound(e.title).extend(),
        // Rest of the status codes will be returned as InternalServerError.
        _ => GqlError::InternalServerError("Internal server error".to_string()).extend(),
    };
}
