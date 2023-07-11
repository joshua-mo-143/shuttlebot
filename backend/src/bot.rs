use crate::{
    commands::{docs, elevate, refresh, resolve, set_locked, set_severity},
    Bot, DBQueries, DiscordBotData,
};
use anyhow::Error;
use octocrab::Octocrab;
use poise::serenity_prelude::{Context, GatewayIntents};
use poise::Event;
use std::collections::HashSet;
use tracing::{info};
use sqlx::types::Json;

type EventError = Box<dyn std::error::Error + Send + Sync>;

async fn handle_event(ctx: &Context, event: &Event<'_>, data: &DiscordBotData) -> Result<(), EventError> {
    match event {
        Event::ThreadCreate { thread, .. } => {
            let thread_url = {
                format!(
                    "https://discord.com/channels/{}/{}",
                    thread.guild_id, thread.id
                )
            };

            let categories = thread.applied_tags.iter().map(|x| x.0.to_string()).collect::<Vec<String>>();

            if let Err(e) = data
                .db
                .clone()
                .discord_create_issue_record(thread_url, thread.id.to_string(), Json(categories.into()))
                .await
            {
                return Err(format!("Error when creating a new issue record: {e}").into());
            }
        }
        Event::Message { new_message, .. } => {
            let thread_url = {
                format!(
                    "https://discord.com/channels/{}/{}",
                    new_message.guild_id.unwrap(),
                    new_message.id
                )
            };

            let mut messages = new_message
                .channel_id
                .messages(ctx.http.clone(), |message| message)
                .await
                .unwrap();
            messages.reverse();

            if messages.len() == 1 {
                let initial_message = messages.first().unwrap();
                let (author, contents) = (
                    initial_message.author.name.to_owned(),
                    initial_message.content.to_owned(),
                );

                if let Err(e) = data
                    .db
                    .clone()
                    .discord_update_initial_message(author, contents, thread_url)
                    .await
                {
                    return Err(format!("Error when updating initial thread message: {e}").into());
                }

                return Ok(());
            }

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
                let message_owner = res.author.name.to_string();

                if message_owner != thread_owner.name {
                    messages_filtered.retain(|m| m.author.id.to_string() == message_owner);

                    if messages_filtered.len() < 2 && hsstr.len() < 2 {
                        if let Err(e) = data
                            .db
                            .clone()
                            .discord_get_first_response(&message_owner, thread_url)
                            .await
                        {
                            return Err(
                                format!("Error when updating initial responder: {e}").into()
                            );
                        }

                        info!("Created new initial message for {message_owner}");
                    }
                    break;
                }
            }
        }
        Event::ReactionAdd {add_reaction, ..} => {
            let message = add_reaction.message(&ctx.http).await.unwrap();
            if message.author.name == *"shuttlebot"
            {
            let channel_id = add_reaction.channel_id.to_string();
            let upvotes = message.reactions[0].count.to_string().parse::<i32>().unwrap() - 1;     
            let downvotes = message.reactions[1].count.to_string().parse::<i32>().unwrap() - 1;     

                data.db.clone().discord_get_feedback(
                    channel_id,
                    upvotes,
                    downvotes
                ).await.unwrap();

                }
    }
        Event::ReactionRemove {removed_reaction, ..} => {
            let message = removed_reaction.message(&ctx.http).await.unwrap();
            if message.author.name == "shuttlebot" {
                   
            let channel_id = removed_reaction.channel_id.to_string();
            let upvotes = message.reactions[0].count.to_string().parse::<i32>().unwrap() - 1;     
            let downvotes = message.reactions[1].count.to_string().parse::<i32>().unwrap() - 1;     

                data.db.clone().discord_get_feedback(
                    channel_id,
                    upvotes,
                    downvotes
                ).await.unwrap();

                }
    }
        
        _ => {println!("{:?}", event);}
    }
    Ok(())
}

pub async fn init_discord_bot(
    discord_token: &str,
    db: DBQueries,
    crab: Octocrab,
    staff_role_id: String,
    server_id: String,
) -> Result<Bot, Error> {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                docs(),
                elevate(),
                set_locked(),
                resolve(),
                set_severity(),
                refresh(),
            ],
            event_handler: |ctx, event, _, data| Box::pin(handle_event(ctx, event, data)),
            ..Default::default()
        })
        .intents(
            GatewayIntents::GUILDS
                | GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT
                | GatewayIntents::GUILD_MESSAGE_REACTIONS
                | GatewayIntents::GUILD_MEMBERS
        )
        .token(discord_token)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(DiscordBotData {
                    db,
                    crab,
                    staff_role_id,
                    server_id,
                })
            })
        });

    Ok(framework)
}