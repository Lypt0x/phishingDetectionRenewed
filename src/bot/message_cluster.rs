use tokio::task::JoinHandle;
use tokio::sync::RwLock;
use twilight_gateway::cluster::Events;
use std::sync::Arc;
use twilight_gateway::{cluster::{Cluster, ShardScheme}};
use twilight_http::Client;
use twilight_model::{gateway::Intents};
use twilight_command_parser::{CommandParserConfig, Parser};
use twilight_cache_inmemory::{InMemoryCache, ResourceType};

use anyhow::Result;

pub struct MessageCluster<'a> {
    pub token: String,
    pub cluster: Arc<Cluster>,
    pub client: Arc<Client>,
    pub events: Arc<RwLock<Events>>,
    pub config: Parser<'a>,
    pub cache: InMemoryCache,
}

#[allow(dead_code)]
impl<'a> MessageCluster<'a> {
    pub async fn new(parser_config: CommandParserConfig<'a>, token: String) -> Result<MessageCluster<'a>> {
        
        let (cluster, events) = Cluster::builder(token.to_owned(),
            Intents::GUILD_MESSAGES |
            Intents::GUILDS |
            Intents::GUILD_MEMBERS)
            .shard_scheme(ShardScheme::Auto)
            .build().await?;

        Ok(MessageCluster {
            client: Arc::new(Client::new(token.clone())),
            cluster: Arc::new(cluster),
            events: Arc::new(RwLock::new(events)),
            config: Parser::new(parser_config),
            cache: InMemoryCache::builder()
                .resource_types(ResourceType::CHANNEL | ResourceType::MESSAGE | ResourceType::ROLE | ResourceType::GUILD)
                .build(),
            token,
        })
    }

    pub fn start_cluster(&self) -> Result<JoinHandle<()>> {
        let cluster = Arc::clone(&self.cluster);
        Ok(tokio::spawn(async move {
            cluster.up().await
        }))
    }

    pub fn stop_cluster(&self) -> Result<()> {
        Ok(self.cluster.down())
    }

    pub fn events(&self) -> Arc<RwLock<Events>> {
        Arc::clone(&self.events)
    }

    pub fn cache(&self) -> &InMemoryCache {
        &self.cache
    }

}