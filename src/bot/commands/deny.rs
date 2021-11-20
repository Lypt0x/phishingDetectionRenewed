use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "deny", desc = "Deny a URL")]
pub struct DenyCommand {
    /// The URL to deny
    url: String,
}