use crate::bot::commands::allow::AllowCommand;
use crate::bot::commands::deny::DenyCommand;
use crate::bot::commands::denied::DeniedCommand;
use crate::Safe;
use anyhow::Result;
use tokio::sync::RwLock;
use std::sync::Arc;
use twilight_model::application::interaction::ApplicationCommand;
use twilight_model::id::ApplicationId;
use twilight_http::Client;
use twilight_interactions::command::CommandModel;
use twilight_interactions::command::CreateCommand;

mod denied;
mod deny;
mod allow;

pub async fn init(client: &Client) -> Result<()> {
    client.set_application_id(ApplicationId::new(std::env::var("APP")?.parse()?).expect("Invalid application id"));
    client.set_global_commands(&[
        denied::DeniedCommand::create_command().into(),
        deny::DenyCommand::create_command().into(),
        allow::AllowCommand::create_command().into(),
    ])?.exec().await?;

    Ok(())
}

pub async fn handle_command(command: ApplicationCommand, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<()> {
 
    let interaction_name = &command.data.name;
    match interaction_name.as_str() {

        "denied" => {
            let denied_command: DeniedCommand = DeniedCommand::from_interaction(command.clone().data)?;
            Ok(denied_command.is_denied_reply(client, &command, safe).await?)
        },

        "deny" => {
            let deny_command: DenyCommand = DenyCommand::from_interaction(command.clone().data)?;
            Ok(deny_command.deny(command, &client, safe).await?)
        },

        "allow" => {
            let allow_command: AllowCommand = AllowCommand::from_interaction(command.clone().data)?;
            Ok(allow_command.allow(command, &client, safe).await?)
        },

        _ => Ok(()),
    }
}