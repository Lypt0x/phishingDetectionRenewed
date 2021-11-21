mod rest;
mod bot;

use twilight_command_parser::CommandParserConfig;
use crate::rest::Safe;
use anyhow::Result;

/*
 * This is the main function of the bot.
 * It is called when the bot is started.
 * 
 * In order to start the bot you need to set the following environment variables:
 *  export DISCORD_TOKEN=your_token_here
 *  export APP=your_app_id_here
 */

#[tokio::main]
async fn main() -> Result<()> {
    let safe = Safe::new("db").await?;
    println!("Starting");

    let mut command_parser = CommandParserConfig::new();
    command_parser.add_prefix("+");
    command_parser.add_command("deny", false);
    command_parser.add_command("allow", false);
    command_parser.add_command("denied", false);

    bot::start_message_cluster(command_parser, safe, std::env::var("DISCORD_TOKEN")?).await?;

    Ok(())
}
