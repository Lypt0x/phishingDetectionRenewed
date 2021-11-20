use twilight_model::application::callback::InteractionResponse;
use twilight_model::application::callback::CallbackData;
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
    
    client.set_application_id(ApplicationId::new(802929155049914468).expect("Invalid application id"));
    client.set_global_commands(&[
        denied::DeniedCommand::create_command().into(),
        deny::DenyCommand::create_command().into(),
        allow::AllowCommand::create_command().into(),
    ])?.exec().await?;

    Ok(())
}

// format UTC+1
fn format_date(time: i64) -> String {
    let date = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(time / 1000 + 3600, 0), chrono::Utc);
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub async fn handle_command(command: ApplicationCommand, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<()> {
 
    let interaction_name = &command.data.name;
    match interaction_name.as_str() {
        "denied" => {

            let denied_command: DeniedCommand = DeniedCommand::from_interaction(command.data).expect("parse command");
            if let Some(data) = denied_command.is_denied(safe).await? {
                if data.safe {
                    client.interaction_callback(command.id, &command.token,
                        &InteractionResponse::ChannelMessageWithSource(CallbackData {
                            allowed_mentions: None,
                            components: None,
                            content: Some("URL is not denied".to_string()),
                            embeds: vec![],
                            flags: None,
                            tts: None
                    })).exec().await.expect("callback");
                } else {
                    client.interaction_callback(command.id, &command.token,
                        &InteractionResponse::ChannelMessageWithSource(CallbackData {
                            allowed_mentions: None,
                            components: None,
                            content: Some(format!("URL is denied at {}", format_date(data.time))),
                            embeds: vec![],
                            flags: None,
                            tts: None
                    })).exec().await.expect("callback");
                }
            }

            Ok(())
        },
        "deny" => {
            Ok(())
        },
        "allow" => {
            Ok(())
        },
        _ => Ok(()),
    }
}