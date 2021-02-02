use crate::error::Kind::WrongCredentials;
use crate::rest::rest_error::OAuthError;
use crate::types::PleasantResult;
use serde::export::Formatter;
use std::error::Error;
use std::fmt::Result as FmtResult;
use std::string::ToString;
use strum_macros::Display as StrumDisplay;

#[derive(Debug, PartialEq)]
pub struct PleasantError {
    pub kind: Kind,
    pub message: String,
    pub context: String,
    pub hint: Option<String>,
    pub cause: Option<Cause>,
}

#[derive(Debug, StrumDisplay, PartialEq, Eq)]
pub enum Kind {
    Unhandled,
    WrongCredentials,
    Database,
    MissingItem,
}

#[derive(Debug)]
pub enum Cause {
    OAuthError {
        inner: OAuthError,
    },
    Unknown {
        inner: std::boxed::Box<dyn std::error::Error>,
    },
    SqlError {
        inner: rusqlite::Error,
    },
}

impl PartialEq for Cause {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Cause::OAuthError { inner }, Cause::OAuthError { inner: other }) => inner == other,
            (Cause::Unknown { inner }, Cause::Unknown { inner: other }) => {
                inner.to_string() == other.to_string()
            }
            _ => false,
        }
    }
}

impl From<OAuthError> for Cause {
    fn from(inner: OAuthError) -> Self {
        Cause::OAuthError { inner }
    }
}

impl From<rusqlite::Error> for Cause {
    fn from(inner: rusqlite::Error) -> Self {
        Cause::SqlError { inner }
    }
}

impl std::error::Error for PleasantError {}

impl std::fmt::Display for PleasantError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(
            f,
            "An error occurred while {}: {}. ",
            self.context, self.message
        )?;
        writeln!(f, "Error kind: {}", &self.kind);
        // if let Some(source) = &self.source {
        //     writeln!(f, "Cause: {}", self.source)
        // }?;
        if let Some(hint) = &self.hint {
            writeln!(f, "Hint: {}", hint)?;
        };
        Ok(())
    }
}

impl From<rusqlite::Error> for PleasantError {
    fn from(sql_error: rusqlite::Error) -> Self {
        PleasantError {
            kind: Kind::Database,
            message: "There was an error with the database connection".to_string(),
            context: "Performing an SQL operation".to_string(),
            hint: None,
            cause: Some(Cause::from(sql_error)),
        }
    }
}

pub trait ResultExt<T> {
    fn err_context(self, context: String) -> crate::types::PleasantResult<T>;
}

impl<T> ResultExt<T> for crate::rest::rest_client::RestResult<T, OAuthError> {
    fn err_context(self, context: String) -> PleasantResult<T> {
        match self {
            Ok(result) => Ok(result),
            Err(oauth_error) => match oauth_error {
                OAuthError::InvalidGrant => Err(PleasantError {
                    kind: Kind::WrongCredentials,
                    message: "Your supplied credentials where rejected by the server".to_string(),
                    context,
                    hint: None,
                    cause: Some(Cause::from(oauth_error)),
                }),
                _ => Err(PleasantError {
                    kind: Kind::Unhandled,
                    message: "An unhandled error occurred".to_string(),
                    context,
                    hint: None,
                    cause: Some(Cause::from(oauth_error)),
                }),
            },
        }
    }
}
