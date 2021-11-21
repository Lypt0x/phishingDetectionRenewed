pub mod interaction;
pub mod message;

use twilight_model::application::callback::CallbackData;
use twilight_model::application::callback::InteractionResponse;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::id::ChannelId;

use twilight_http::Client;

use interaction::InteractionReply;
use message::MessageReply;

use anyhow::Result;

#[async_trait::async_trait]
impl InteractionReply for Client {
    async fn reply_interaction(&self, command: &ApplicationCommand, content: &str) -> Result<()> {

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

#[async_trait::async_trait]
impl MessageReply for Client {
    async fn reply_message(&self, channel_id: ChannelId, content: &str) -> Result<()> {

        self.create_message(channel_id)
            .content(content).expect("message").exec()
            .await.expect("message");

        Ok(())
    }
}