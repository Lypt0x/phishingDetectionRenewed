use twilight_model::application::interaction::ApplicationCommand;
use twilight_http::Client;
use twilight_model::application::interaction::Interaction;
use crate::Safe;
use tokio::sync::RwLock;
use std::sync::Arc;
use anyhow::Result;
use futures::StreamExt;
use crate::bot::message_cluster::MessageCluster;
use crate::bot::message_parser::MessageParser;
use crate::bot::message_prepare::MessagePrepare;
use twilight_gateway::Event;

use crate::bot::commands::handle_command;

#[async_trait::async_trait]
pub trait EventHandler {

    async fn handle_event<'a>(&self, message_cluster: Arc<MessageCluster<'static>>, safe: Arc<RwLock<Safe>>) -> Result<()> {
        let mut events = message_cluster.events.write().await;

        while let Some((_shard_id, event)) = events.next().await {
            let cache = &message_cluster.cache;
            cache.update(&event);
            
            let message_cluster = Arc::clone(&message_cluster);
            let safe = Arc::clone(&safe);
    
            match event {
                Event::MessageCreate(msg) => {
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
                },
                Event::InteractionCreate(interaction) => {
                    tokio::spawn(async move {
                        let inter: Interaction = interaction.0;
                        match inter {
                            Interaction::ApplicationCommand(command) => {
                                let client: &Client = &message_cluster.client;
                                let command: ApplicationCommand = *command;
    
                                handle_command(command, client, Arc::clone(&safe)).await.expect("handle command");
                            },
                            _ => ()
                        }
                    });
                }
                _ => {}
            }
    
        }

        Ok(())
    }
}