use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub message: String,
}

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AppError {
    #[error("{message:}")]
    AdapterError { message: String },
    #[error("{message:}")]
    DeviceError { message: String },
    #[error("{message:}")]
    RegistrationError { message: String },
    #[error("{message:}")]
    InternalError { message: String },
}
