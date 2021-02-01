use crate::error::PleasantError;

pub type PleasantResult<T, E = PleasantError> = std::result::Result<T, E>;
