use crate::rest::SafeData;
use linkify::Link;
use twilight_command_parser::Parser;
use crate::Safe;
use crate::bot::*;
use twilight_model::channel::Message;
use twilight_http::Client;
use anyhow::Result;

/* LEGACY COMMANDS */

#[async_trait::async_trait]
pub trait MessageParser {
    async fn parse(&self, parser: Parser<'_>, permission: InMemoryCachePermissions<'_>, client: &Client,
                    message: &Message, safe: Arc<RwLock<Safe>>) -> Result<bool> {

        if message.author.bot {
            return Ok(false);
        }

        let user_permissions = permission.in_channel(message.author.id, message.channel_id).expect("in channel");
        if user_permissions.contains(Permissions::MANAGE_MESSAGES) {
            let finder = LinkFinder::new();

            return match parser.parse(message.content.as_str()) {
                Some(Command {
                    name: "deny", mut arguments, ..
                }) => {
                    if arguments.clone().count() == 0 { return Ok(true); }
                    let link = finder.links(arguments.next().expect("next argument")).nth(0);

                    return self.deny(link, &message, &client, safe).await
                },

                Some(Command {
                    name: "allow", mut arguments, ..
                }) => {
                    if arguments.clone().count() == 0 { return Ok(true); }
                    let link = finder.links(arguments.next().expect("next argument")).nth(0);

                    return self.allow(link, &message, &client, safe).await
                },

                Some(Command {
                    name: "is_denied", mut arguments, ..
                }) => {
                    if arguments.clone().count() == 0 { return Ok(true); }
                    let link = finder.links(arguments.next().expect("next argument")).nth(0);

                    if let Some(data) = self.is_denied(link, safe).await.expect("data") {
                        if data.safe {
                            client.create_message(message.channel_id)
                            .content("Url is not denied").expect("message")
                            .exec()
                            .await.expect("message");
                        } else {
                            client.create_message(message.channel_id)
                            .content(&format!("Url is denied at {}", self.format_date(data.time))).expect("message")
                            .exec()
                            .await.expect("message");
                        }
                 
                    }

                    return Ok(false)
                },

                Some(_) => Ok(true),
                None => Ok(true)
            }
        }
        

        Ok(true)
    }

    // format UTC+1
    fn format_date(&self, time: i64) -> String {
        let date = chrono::DateTime::<chrono::Utc>::from_utc(
            chrono::NaiveDateTime::from_timestamp(time / 1000 + 3600, 0), chrono::Utc);
        date.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    async fn is_denied(&self, link: Option<Link<'_>>, safe: Arc<RwLock<Safe>>) -> Result<Option<SafeData>> {
        if let Some(link) = link {
            let link = link.as_str();
            let mut safe = safe.write().await;
            return Ok(Some(safe.is_safe(link).await?));
        }

        Ok(None)
    }

    async fn allow(&self, link: Option<Link<'_>>, message: &Message, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<bool> {
        match link {
            Some(url) => {
                let mut safe = safe.write().await;
                if safe.is_denied(url.as_str()).unwrap() {
                    safe.allow(url.as_str()).unwrap();
                    client.create_message(message.channel_id)
                        .content("Url has been allowed").expect("message")
                        .exec()
                        .await.expect("message");
                } else {
                    client.create_message(message.channel_id)
                        .content("Url is not denied").expect("message")
                        .exec()
                        .await.expect("message");
                }
            },
            None => {
                client.create_message(message.channel_id)
                    .content("Invalid URL").expect("message")
                    .exec()
                    .await.expect("message");
            }
        }
        Ok(false)
    }

    async fn deny(&self, link: Option<Link<'_>>, message: &Message, client: &Client, safe: Arc<RwLock<Safe>>) -> Result<bool> {
        match link {
            Some(url) => {
                let mut safe = safe.write().await;
                if safe.is_denied(url.as_str()).unwrap() {
                    client.create_message(message.channel_id)
                        .content("Url is already denied").expect("message")
                        .exec()
                        .await.expect("message");
                } else {
                    safe.deny(url.as_str()).unwrap();
                    client.create_message(message.channel_id)
                        .content("Url has been denied").expect("message")
                        .exec()
                        .await.expect("message");
                }
            },
            None => {
                client.create_message(message.channel_id)
                    .content("Invalid URL").expect("message")
                    .exec()
                    .await.expect("message");
            }
        }
        Ok(false)
    }

}