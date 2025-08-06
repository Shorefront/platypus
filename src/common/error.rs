//! Error Module
//!
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct PlatypusError {
    pub message: String,
}

impl std::fmt::Display for PlatypusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Platypus Error: {}", self.message)
    }
}

impl From<surrealdb::Error> for PlatypusError {
    fn from(value: surrealdb::Error) -> Self {
        PlatypusError {
            message: value.to_string(),
        }
    }
}

impl From<&str> for PlatypusError {
    fn from(value: &str) -> Self {
        PlatypusError {
            message: value.to_owned(),
        }
    }
}

#[derive(Default, Deserialize, Serialize)]
pub struct TMFError {
    code: String,
    reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_error: Option<String>,
}

impl From<PlatypusError> for TMFError {
    fn from(value: PlatypusError) -> Self {
        TMFError {
            code: "PLAT001".into(),
            reason: value.message.clone(),
            message: Some(format!("PLAT001: {}", value.message.clone())),
            status: None,
            reference_error: None,
        }
    }
}

impl From<serde_json::Error> for PlatypusError {
    fn from(value: serde_json::Error) -> Self {
        PlatypusError {
            message: value.to_string(),
        }
    }
}
