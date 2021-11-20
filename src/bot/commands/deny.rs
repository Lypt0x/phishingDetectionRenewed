use twilight_model::application::callback::CallbackData;
use twilight_model::application::callback::InteractionResponse;
use crate::Safe;
use tokio::sync::RwLock;
use anyhow::Result;
use std::sync::Arc;
use twilight_http::Client;
use twilight_model::application::interaction::ApplicationCommand;
use linkify::Link;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "deny", desc = "Deny a URL")]
pub struct DenyCommand {
    /// The URL to deny
    url: String,
}

impl DenyCommand {
    pub async fn deny(&self, link: Option<Link<'_>>, command: ApplicationCommand, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<bool> {
        match link {
            Some(url) => {
                let mut safe = safe.write().await;
                if safe.is_denied(url.as_str()).unwrap() {
                    client.interaction_callback(command.id, &command.token,
                        &InteractionResponse::ChannelMessageWithSource(CallbackData {
                            allowed_mentions: None,
                            components: None,
                            content: Some("URL is already denied".to_string()),
                            embeds: vec![],
                            flags: None,
                            tts: None
                        })).exec().await.expect("callback");
                } else {
                    safe.deny(url.as_str()).unwrap();
                    client.interaction_callback(command.id, &command.token,
                        &InteractionResponse::ChannelMessageWithSource(CallbackData {
                            allowed_mentions: None,
                            components: None,
                            content: Some("URL has been denied".to_string()),
                            embeds: vec![],
                            flags: None,
                            tts: None
                        })).exec().await.expect("callback");
                }
            },
            None => {
                client.interaction_callback(command.id, &command.token,
                    &InteractionResponse::ChannelMessageWithSource(CallbackData {
                        allowed_mentions: None,
                        components: None,
                        content: Some("Invalid URL. Example: `https://example.com`".to_string()),
                        embeds: vec![],
                        flags: None,
                        tts: None
                    })).exec().await.expect("callback");
            }
        }
        Ok(false)
    }
}