use linkify::LinkFinder;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::rest::Safe;
use twilight_model::gateway::payload::incoming::MessageCreate;

use anyhow::Result;

#[async_trait::async_trait]
pub trait MessagePrepare<'a> {

    async fn can_be_submitted(&self, safe: Arc<RwLock<Safe>>, message: Box<MessageCreate>) -> Result<bool> {

        let mut safe = safe.write().await;

        // check if message is from bot
        if message.author.bot {
            return Ok(true);
        }

        let finder = LinkFinder::new();
        let links: Vec<_> = finder.links(&message.content).collect();

        if links.len() >= 1 {
            for link in links {

                if let Ok(safe_state) = safe.is_safe(link.as_str()).await {
                    return Ok(safe_state.is_safe());
                }
            }
        }
        

        Ok(true)
    }
}