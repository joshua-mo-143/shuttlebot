use crate::{
    commands::{docs, elevate, set_locked},
    BotInit, Data,
};
use anyhow::Error;
use octocrab::Octocrab;
use poise::serenity_prelude::{
    async_trait,
    model::{channel::GuildChannel, gateway::Ready},
    Context, EventHandler, GatewayIntents, Message,
};
use sqlx::PgPool;
use tracing::{error, info};

const BOT_USER_ID: &str = "1116377484296978452";

pub struct Bot {
    pub pool: PgPool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let channel_id = msg.channel_id.to_string();

        let mut messages = msg
            .channel_id
            .messages(ctx.http, |message| message)
            .await
            .unwrap();
        messages.reverse();

        let mut messages_filtered = messages.clone();

        let thread_owner = messages.first().unwrap().author.name.clone();

        while let Some(res) = messages.clone().iter().next() {
            let message_owner = res.author.id.to_string();

            if message_owner != thread_owner && message_owner != *BOT_USER_ID {
                messages_filtered.retain(|m| m.author.id.to_string() == message_owner);

                if messages_filtered.len() < 2 {
                    if let Err(e) = sqlx::query(
                        "UPDATE issues SET
                    FirstResponseUser = $1, 
                    FirstResponseTimedate = CURRENT_TIMESTAMP 
                    WHERE DiscordThreadId = $2",
                    )
                    .bind(&message_owner)
                    .bind(channel_id)
                    .execute(&self.pool)
                    .await
                    {
                        error!(
                            "Error when updating record to show who responded first: {:?}",
                            e
                        );
                    }

                    info!("Created new initial message for {message_owner}");
                }
                break;
            }
        }
    }

    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        let thread_id: String = thread.id.to_string();

        if let Err(e) = sqlx::query("INSERT INTO issues (DiscordThreadId, SevCat) VALUES ($1, $2)")
            .bind(thread_id)
            .bind(5)
            .execute(&self.pool)
            .await
        {
            error!("Error inserting issue to db while creating new helpthread record: {e:?}");
        }

        if let Err(e) = thread.say(ctx.http, "Thanks for reporting this issue! We'll be looking into this soon and you should receive a response shortly.").await {
            error!("Error sending message: {:?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

pub async fn init_discord_bot(
    discord_token: String,
    pool: PgPool,
    crab: Octocrab,
) -> Result<BotInit, Error> {
    let pool2 = pool.clone();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![docs(), elevate(), set_locked()],
            ..Default::default()
        })
        .client_settings(|f| f.event_handler(Bot { pool: pool2 }))
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT,
        )
        .token(discord_token)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { pool, crab })
            })
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework)
}
