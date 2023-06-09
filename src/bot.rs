use octocrab::Octocrab;
use serenity::async_trait;
use serenity::model::channel::{GuildChannel, Message};
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use tracing::{error, info};

use crate::commands::link_docs;

pub struct Bot {
    pool: sqlx::PgPool,
    octocrab: Octocrab,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let cmd: Vec<&str> = msg.content.split_whitespace().collect::<Vec<&str>>();

        match cmd[0] {
            "/hello" => {
                if let Err(e) = msg.channel_id.say(ctx.http, "world!").await {
                    error!("Error sending message: {:?}", e);
                }
            }
            "/elevate" => {
                let user = msg.author.name;
                let messages = msg
                    .channel_id
                    .messages(&ctx.http, |message| message)
                    .await
                    .unwrap();

                let message_content = messages.iter().next().unwrap().content;

                let message = format!("This message was autogenerated by shuttlebot. The issue was originally created by {user} on discord. \
                        ---\
                     {message_content}");
                if let Err(e) = self
                    .octocrab
                    .issues("joshua-mo-143", "shuttlebot")
                    .create("[AUTOGENERATED]")
                    .body(message)
                    .send()
                    .await
                {
                    error!("Error creating Github issue: {:?}", e);
                }
            }

            "!docs" => link_docs(cmd[1], ctx, msg).await.unwrap(),
            _ => {}
        }
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        if let Err(e) = sqlx::query("INSERT INTO issues () VALUES ()")
            .execute(&self.pool)
            .await
        {
            error!("Error inserting issue to db: {:?}", e);
        }

        if let Err(e) = thread.say(ctx.http, "Thanks for reporting this issue! We'll be looking into this soon and you should receive a response shortly.").await {
            error!("Error sending message: {:?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
