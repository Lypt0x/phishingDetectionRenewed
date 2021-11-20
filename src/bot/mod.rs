mod handling;
mod parser;
mod preparer;
mod cluster;
mod commands;

use crate::bot::parser::MessageParser;
use crate::bot::handling::EventHandler;
use twilight_model::guild::Permissions;
use twilight_command_parser::Command;
use twilight_command_parser::CommandParserConfig;
use tokio::sync::RwLock;
use std::sync::Arc;
use crate::rest::Safe;
use crate::bot::preparer::MessagePrepare;
use crate::bot::cluster::MessageCluster;
use anyhow::Result;

use twilight_cache_inmemory::permission::InMemoryCachePermissions;

pub async fn start_message_cluster(parser_config: CommandParserConfig<'static>, safe: Safe, token: String) -> Result<()> {
    let message_cluster = Arc::new(MessageCluster::new(parser_config, token).await?);
    message_cluster.start_cluster()?;

    commands::init(&message_cluster.client).await?;
    message_cluster.handle_event(Arc::clone(&message_cluster), Arc::new(RwLock::new(safe))).await?;

    Ok(())
}

impl<'a> MessagePrepare<'a> for MessageCluster<'a> {}
impl<'a> MessageParser for MessageCluster<'a> {}
impl<'a> EventHandler for MessageCluster<'a> {}