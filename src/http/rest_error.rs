use serde::export::Formatter;

#[derive(Debug)]
pub enum OAuthError {
    InvalidGrant,
    UnsupportedGrantType,
    UnsupportedTokenType {
        token_type: String,
    },
    ProtocolError {
        message: String,
    },
    InternalServerError,
    NetworkError {
        message: String,
        cause: std::boxed::Box<dyn std::error::Error>,
    },
}

impl std::error::Error for OAuthError {}

impl std::fmt::Display for OAuthError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OAuthError::InvalidGrant => write!(f, "The server rejected the provided credentials."),
            OAuthError::UnsupportedGrantType => {
                write!(f, "The server did not support the provided grant type")
            }
            OAuthError::UnsupportedTokenType { token_type } => write!(
                f,
                "The server returned a token of type '{}' which is not supported",
                token_type
            ),
            OAuthError::ProtocolError { message } => write!(
                f,
                "The server got an unexpected response. Message: {}",
                message
            ),
            OAuthError::InternalServerError => write!(f, "The server has an internal error."),
            OAuthError::NetworkError {  message, cause } => write!(f,"An network error occurred while communicating with the server. Message: {}. \t cause: {}", message, cause)
        }
    }
}

impl PartialEq for OAuthError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (OAuthError::InvalidGrant, OAuthError::InvalidGrant)
            | (OAuthError::InternalServerError, OAuthError::InternalServerError)
            | (OAuthError::UnsupportedGrantType, OAuthError::UnsupportedGrantType) => true,
            (
                OAuthError::UnsupportedTokenType { token_type },
                OAuthError::UnsupportedTokenType {
                    token_type: other_token_type,
                },
            ) => token_type == other_token_type,
            (
                OAuthError::ProtocolError { message },
                OAuthError::ProtocolError {
                    message: other_message,
                },
            ) => message == other_message,
            (
                OAuthError::NetworkError { message, cause },
                OAuthError::NetworkError {
                    message: other_message,
                    cause: other_cause,
                },
            ) => message == other_message && cause.to_string() == other_cause.to_string(),
            _ => false,
        }
    }
}

impl From<reqwest::Error> for OAuthError {
    fn from(error: reqwest::Error) -> Self {
        OAuthError::NetworkError {
            message: "An unhandled network error occurred".to_string(),
            cause: Box::new((error)),
        }
    }
}
