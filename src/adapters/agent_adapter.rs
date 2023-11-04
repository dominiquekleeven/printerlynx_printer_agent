// Common agent adapter traits and structs. Used by impl blocks in the agent adapters.

use crate::common::app_error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait AgentAdapter {
    async fn name(&self) -> String;
    async fn setup(&self) -> Result<(), AppError>;
    async fn teardown(&self) -> Result<(), AppError>;
}
