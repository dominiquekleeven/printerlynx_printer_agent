// Common agent adapter traits and structs. Used by impl blocks in the agent adapters.

use crate::common::app_error::AppError;

pub trait AgentAdapter {
    fn name(&self) -> String;
    fn setup(&self) -> Result<(), AppError>;
    fn teardown(&self) -> Result<(), AppError>;
    fn start(&self) -> Result<(), AppError>;
}
