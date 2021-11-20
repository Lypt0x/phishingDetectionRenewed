use url::Url;
use twilight_model::application::callback::CallbackData;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::application::callback::InteractionResponse;
use crate::Safe;
use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use twilight_http::Client;
use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "allow", desc = "Allow a URL unless it is not affected by Google")]
pub struct AllowCommand {
    /// The URL to allow
    url: String,
}

impl AllowCommand {
    pub async fn allow(&self, command: ApplicationCommand, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<bool> {
        let link = Url::parse(&self.url);

        match link {
            Ok(url) => {
                let mut safe = safe.write().await;
                if safe.is_denied(url.as_str()).unwrap() {
                    safe.allow(url.as_str()).unwrap();
                    client.interaction_callback(command.id, &command.token,
                        &InteractionResponse::ChannelMessageWithSource(CallbackData {
                            allowed_mentions: None,
                            components: None,
                            content: Some("URL has been allowed".to_string()),
                            embeds: vec![],
                            flags: None,
                            tts: None
                        })).exec().await.expect("callback");
                } else {
                    client.interaction_callback(command.id, &command.token,
                        &InteractionResponse::ChannelMessageWithSource(CallbackData {
                            allowed_mentions: None,
                            components: None,
                            content: Some("URL is not denied".to_string()),
                            embeds: vec![],
                            flags: None,
                            tts: None
                        })).exec().await.expect("callback");
                }
            },
            Err(_) => {
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