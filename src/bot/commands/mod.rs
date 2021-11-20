use twilight_model::id::ApplicationId;
use twilight_http::Client;
use twilight_interactions::command::CreateCommand;

use anyhow::Result;

mod denied;
mod deny;
mod allow;

pub async fn init(client: &Client) -> Result<()> {
    
    client.set_application_id(ApplicationId::new(802929155049914468).expect("Invalid application id"));
    client.set_global_commands(&[
        denied::DeniedCommand::create_command().into(),
        deny::DenyCommand::create_command().into(),
        allow::AllowCommand::create_command().into(),
    ])?.exec().await?;

    Ok(())
}