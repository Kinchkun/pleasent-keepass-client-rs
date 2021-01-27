use crate::error::PleasantError;

pub type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;

pub type PleasantResult<T> = std::result::Result<T, PleasantError>;
