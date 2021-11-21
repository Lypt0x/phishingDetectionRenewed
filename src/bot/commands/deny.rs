use url::Url;
use crate::Safe;
use tokio::sync::RwLock;
use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_interactions::command::{CommandModel, CreateCommand};

use crate::bot::utils::interaction::InteractionReply;

#[derive(CommandModel, CreateCommand)]
#[command(name = "deny", desc = "Deny a URL")]
pub struct DenyCommand {
    /// The URL to deny
    url: String,
}

impl DenyCommand {
    pub async fn deny(&self, command: ApplicationCommand, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<()> {
        let link = Url::parse(&self.url);

        match link {
            Ok(url) => {
                let mut safe = safe.write().await;
                if safe.is_denied(url.as_str()).unwrap() {
                    client.reply_interaction(&command, "URL is already denied").await.expect("reply");
                } else {
                    safe.deny(url.as_str()).unwrap();
                    client.reply_interaction(&command, "URL has been denied").await.expect("reply");
                }
            },
            Err(_) => {
                client.reply_interaction(&command, "Invalid URL. Example: `https://example.com`").await.expect("reply");
            }
        }
        Ok(())
    }
}