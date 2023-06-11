use crate::{Context, Error};
use anyhow::anyhow;
use poise::serenity_prelude::model::channel::GuildChannel;
use shuttle_secrets::SecretStore;
use tracing::error;

pub fn get_secrets(secrets: SecretStore) -> Result<(String, String), anyhow::Error> {
    let discord_token = if let Some(discord_token) = secrets.get("DISCORD_TOKEN") {
        discord_token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found"));
    };

    let github_token = if let Some(github_token) = secrets.get("GITHUB_TOKEN") {
        github_token
    } else {
        return Err(anyhow!("'GITHUB_TOKEN' was not found"));
    };

    Ok((discord_token, github_token))
}

pub struct Thread {}

impl Thread {
    pub async fn get(ctx: Context<'_>) -> GuildChannel {
        // unwrap should be fine here as channel/guild ID should always be present in the happy path
        ctx.serenity_context()
            .http
            .get_channel(ctx.channel_id().into())
            .await
            .unwrap()
            .guild()
            .unwrap()
    }

    pub async fn set_locked_status(ctx: Context<'_>, status: bool) -> Result<(), Error> {
        if let Err(e) = Thread::get(ctx)
            .await
            .edit_thread(ctx.serenity_context().http.clone(), |f| f.locked(status))
            .await
        {
            error!("Couldn't lock thread: {:?}", e);
        }

        Ok(())
    }
}
