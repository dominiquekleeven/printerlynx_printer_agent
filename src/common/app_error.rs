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
    InternalServer { message: String },
}
