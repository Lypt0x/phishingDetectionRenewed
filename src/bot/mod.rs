mod message_parser;
mod message_prepare;
mod message_cluster;

use crate::bot::message_parser::MessageParser;
use linkify::LinkFinder;
use twilight_model::guild::Permissions;
use twilight_command_parser::Command;
use twilight_command_parser::CommandParserConfig;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::rest::Safe;
use crate::bot::message_prepare::MessagePrepare;
use crate::bot::message_cluster::MessageCluster;
use twilight_gateway::Event;
use futures::StreamExt;
use anyhow::Result;

use twilight_cache_inmemory::permission::InMemoryCachePermissions;

pub async fn start_message_cluster<'a>(parser_config: CommandParserConfig<'static>, safe: Safe, token: String) -> Result<()> {
    let message_cluster = Arc::new(MessageCluster::new(parser_config, token).await?);
    message_cluster.start_cluster()?;
    
    let mut events = message_cluster.events.write().await;
    let safe = Arc::new(RwLock::new(safe));

    while let Some((_shard_id, event)) = events.next().await {
        let cache = &message_cluster.cache;
        cache.update(&event);
        

        let message_cluster = Arc::clone(&message_cluster);
        let safe = Arc::clone(&safe);

        if let Event::MessageCreate(msg) = event {
            tokio::spawn(async move {

                let parser = message_cluster.config.clone();
                if !message_cluster.parse(parser, message_cluster.cache().permissions(),
                            &message_cluster.client, &msg, Arc::clone(&safe)).await.expect("forward state") {
                    return;
                }

                if !message_cluster.can_be_submitted(safe, Box::clone(&msg)).await.expect("can't be submitted") {
                    let client = &message_cluster.client;

                    // Small workaround due to the fact that the message sometimes doesn't get deleted
                    while client.delete_message(msg.channel_id, msg.id).exec().await.is_err() {
                        println!("Failed to delete message - retry in 500ms");
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }
                }
            });
        }
    }

    Ok(())
}

impl<'a> MessagePrepare<'a> for MessageCluster<'a> {}
impl<'a> MessageParser for MessageCluster<'a> {}