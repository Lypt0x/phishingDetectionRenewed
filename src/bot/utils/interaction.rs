use twilight_model::application::interaction::ApplicationCommand;

use anyhow::Result;

#[async_trait::async_trait]
pub trait InteractionReply {
    async fn reply_interaction(&self, command: &ApplicationCommand, content: &str) -> Result<()>;
}