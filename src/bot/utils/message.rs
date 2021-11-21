use twilight_model::id::ChannelId;
use anyhow::Result;

#[async_trait::async_trait]
pub trait MessageReply {
    async fn reply_message(&self, channel_id: ChannelId, content: &str) -> Result<()>;
}