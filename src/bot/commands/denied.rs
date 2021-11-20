use crate::rest::SafeData;
use crate::Safe;
use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "denied", desc = "Check if a URL has been denied")]
pub struct DeniedCommand {
    /// The URL to check
    url: String,
}

impl DeniedCommand {
    pub async fn is_denied(&self, safe: Arc<RwLock<Safe>>) -> Result<Option<SafeData>> {
        let mut safe = safe.write().await;
        return Ok(Some(safe.is_safe(&self.url).await?));
    }
}