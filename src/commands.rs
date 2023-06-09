use serenity::model::channel::{GuildChannel, Message};
use serenity::prelude::*;
use tracing::error;

pub async fn link_docs<'a>(flag: &'a str, ctx: Context, msg: Message) -> Result<(), anyhow::Error> {
    match DocsLinks::from_str(flag) {
        Ok(res) => {
            let res = res.to_link();

            if let Err(e) = msg.channel_id.say(ctx.http, flag).await {
                error!("Error sending message: {:?}", e);
            }
        }
        Err(e) => {
            msg.channel_id
                .say(&ctx.http, "That's not a valid flag!")
                .await;
        }
    }
    Ok(())
}

enum DocsLinks {
    Secrets,
    Databases,
    Persist,
    Assets,
    Custom,
}

impl DocsLinks {
    fn from_str<'a>(str: &'a str) -> Result<Self, String> {
        match str {
            "secrets" => Ok(DocsLinks::Secrets),
            "databases" => Ok(DocsLinks::Databases),
            "persist" => Ok(DocsLinks::Persist),
            "assets" => Ok(DocsLinks::Assets),
            "custom" => Ok(DocsLinks::Custom),
            _ => {
                return Err("Doc doesn't exist".to_string());
            }
        }
    }

    fn to_link(&self) -> String {
        match self {
            DocsLinks::Secrets => String::from("https://docs.shuttle.rs/resources/shuttle-secrets"),
            DocsLinks::Databases => {
                String::from("https://docs.shuttle.rs/resources/shuttle-shared-db")
            }
            DocsLinks::Persist => String::from("https://docs.shuttle.rs/resources/shuttle-persist"),
            DocsLinks::Assets => {
                String::from("https://docs.shuttle.rs/resources/shuttle-static-folder")
            }
            DocsLinks::Custom => String::from("https://docs.shuttle.rs/tutorials/custom-service"),
        }
    }
}
