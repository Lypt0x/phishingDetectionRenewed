use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;
use crate::rest::SafeData;
use crate::Safe;
use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::bot::utils::interaction::InteractionReply;

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

    pub async fn is_denied_reply(&self, client: &Client, command: &ApplicationCommand, safe: Arc<RwLock<Safe>>) -> Result<()> {
        if let Some(safe_data) = self.is_denied(safe).await? {
            if safe_data.safe {
                client.reply_interaction(&command, "URL is not denied").await.expect("reply");
            } else {
                client.reply_interaction(&command, &format!("URL is denied at {}", format_date(safe_data.time))).await.expect("reply");
            }
        }

        Ok(())
    }

}

// format UTC+1
fn format_date(time: i64) -> String {
    let date = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(time / 1000 + 3600, 0), chrono::Utc);
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}