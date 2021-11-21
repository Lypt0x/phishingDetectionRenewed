pub mod interaction;

use twilight_model::application::callback::CallbackData;
use twilight_model::application::callback::InteractionResponse;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_http::Client;
use interaction::InteractionReply;

use anyhow::Result;

#[async_trait::async_trait]
impl InteractionReply for Client {
    async fn reply_content(&self, command: &ApplicationCommand, content: &str) -> Result<()> {

        self.interaction_callback(command.id, &command.token,
            &InteractionResponse::ChannelMessageWithSource(CallbackData {
                allowed_mentions: None,
                components: None,
                content: Some(content.into()),
                embeds: vec![],
                flags: None,
                tts: None
            })).exec().await.expect("callback");

        Ok(())
    }
}