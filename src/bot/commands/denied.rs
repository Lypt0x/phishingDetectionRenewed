use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "denied", desc = "Check if a URL has been denied")]
pub struct DeniedCommand {
    /// The URL to check
    url: String,
}