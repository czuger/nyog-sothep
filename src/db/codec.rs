//! Reading stored codes and JSON back into domain values.

use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::domain::LocationId;
use crate::error::AppError;

/// Reads a location code.
pub fn location(column: &'static str, code: &str) -> Result<LocationId, AppError> {
    LocationId::from_code(code).ok_or_else(|| corrupt(column, code))
}

/// Reads an optional location code.
pub fn optional_location(
    column: &'static str,
    code: Option<&str>,
) -> Result<Option<LocationId>, AppError> {
    code.map(|code| location(column, code)).transpose()
}

/// Reads a JSON value.
pub fn from_json<T: DeserializeOwned>(column: &'static str, json: &str) -> Result<T, AppError> {
    serde_json::from_str(json).map_err(|_| corrupt(column, json))
}

/// Writes a JSON value.
pub fn to_json<T: Serialize>(column: &'static str, value: &T) -> Result<String, AppError> {
    serde_json::to_string(value).map_err(|error| corrupt(column, &error.to_string()))
}

/// The error for a value that cannot be read.
pub fn corrupt(column: &'static str, value: &str) -> AppError {
    AppError::CorruptData {
        column,
        value: value.to_string(),
    }
}
