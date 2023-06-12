use crate::{
    commands::{docs, elevate, set_locked},
    Bot, Data,
};
use anyhow::Error;
use octocrab::Octocrab;
use poise::serenity_prelude::{Context, GatewayIntents};
use poise::Event;
use sqlx::PgPool;
use std::collections::HashSet;

use tracing::{error, info};

const _BOT_USER_ID: &str = "1116377484296978452";

type EventError = Box<dyn std::error::Error + Send + Sync>;

async fn handle_event(ctx: &Context, event: &Event<'_>, data: &Data) -> Result<(), EventError> {
    match event {
        Event::ThreadCreate { thread, .. } => {
            let thread_id: String = thread.id.to_string();

            if let Err(e) =
                sqlx::query("INSERT INTO issues (DiscordThreadId, SevCat) VALUES ($1, $2)")
                    .bind(thread_id)
                    .bind(5)
                    .execute(&data.pool)
                    .await
            {
                error!("Error inserting issue to db while creating new helpthread record: {e:?}");
            }
        }
        Event::Message { new_message, .. } => {
            let channel_id = new_message.channel_id.to_string();

            let mut messages = new_message
                .channel_id
                .messages(ctx.http.clone(), |message| message)
                .await
                .unwrap();
            messages.reverse();

            let mut messages_filtered = messages.clone();

            let thread_owner = messages.first().unwrap().author.clone();

            let mut hsstr = messages
                .clone()
                .iter()
                .map(|x| *x.author.id.as_u64())
                .collect::<HashSet<u64>>();

            hsstr.remove(thread_owner.id.as_u64());

            let msg_iter = messages.clone().into_iter();

            for res in msg_iter {
                let message_owner = res.author.id.to_string();

                if message_owner != thread_owner.id.to_string() {
                    messages_filtered.retain(|m| m.author.id.to_string() == message_owner);

                    if messages_filtered.len() < 2 && hsstr.len() < 2 {
                        if let Err(e) = sqlx::query(
                            "UPDATE issues SET
                    FirstResponseUser = $1, 
                    FirstResponseTimedate = CURRENT_TIMESTAMP 
                    WHERE DiscordThreadId = $2",
                        )
                        .bind(&message_owner)
                        .bind(channel_id)
                        .execute(&data.pool)
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
        _ => {}
    }
    Ok(())
}

pub async fn init_discord_bot(
    discord_token: String,
    pool: PgPool,
    crab: Octocrab,
) -> Result<Bot, Error> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![docs(), elevate(), set_locked()],
            event_handler: |ctx, event, _, data| Box::pin(handle_event(ctx, event, data)),
            ..Default::default()
        })
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
        });

    Ok(framework)
}
