use url::Url;
use twilight_model::application::interaction::ApplicationCommand;
use crate::Safe;
use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::bot::utils::interaction::InteractionReply;

#[derive(CommandModel, CreateCommand)]
#[command(name = "allow", desc = "Allow a URL unless it is not affected by Google")]
pub struct AllowCommand {
    /// The URL to allow
    url: String,
}

impl AllowCommand {
    pub async fn allow(&self, command: ApplicationCommand, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<()> {
        let link = Url::parse(&self.url);

        match link {
            Ok(url) => {
                let mut safe = safe.write().await;
                if safe.is_denied(url.as_str()).unwrap() {
                    safe.allow(url.as_str()).unwrap();
                    client.reply_content(&command, "URL has been allowed").await.expect("reply");
                } else {
                    client.reply_content(&command, "URL is already allowed").await.expect("reply");
                }
            },
            Err(_) => {
                client.reply_content(&command, "Invalid URL. Example: `https://example.com`").await.expect("reply");
            }
        }
        Ok(())
    }
}