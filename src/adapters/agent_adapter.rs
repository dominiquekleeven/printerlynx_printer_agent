// Common agent adapter traits and structs. Used by impl blocks in the agent adapters.

use crate::common::app_error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait AgentAdapter: {
    fn name(&self) -> String;
    async fn start(&self) -> Result<(), AppError>;
    async fn teardown(&self) -> Result<(), AppError>;
}
