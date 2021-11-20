use twilight_interactions::command::{CommandModel, CreateCommand};

#[derive(CommandModel, CreateCommand)]
#[command(name = "allow", desc = "Allow a URL unless it is not affected by Google")]
pub struct AllowCommand {
    /// The URL to allow
    url: String,
}