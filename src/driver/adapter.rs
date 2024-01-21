// Common agent adapter traits and structs. Used by impl blocks in the agent driver.

use crate::common::app_error::AppError;
use async_trait::async_trait;

#[async_trait]
pub trait Adapter: Sync + Send {
    async fn is_running(&self) -> bool;
    async fn configure(&mut self, printer: String) -> Result<(), AppError>;
    async fn start(&mut self) -> Result<(), AppError>;
    async fn stop(&mut self) -> Result<(), AppError>;
    async fn send_command(&mut self, command: &str) -> Result<(), AppError>;
    async fn start_print(&mut self, commands: Vec<String>) -> Result<(), AppError>;
    async fn stop_print(&mut self) -> Result<(), AppError>;
    async fn pause_print(&mut self) -> Result<(), AppError>;
    async fn resume_print(&mut self) -> Result<(), AppError>;
}
