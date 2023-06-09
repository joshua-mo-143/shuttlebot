use crate::{Context, Error};
use poise::serenity_prelude::model::channel::GuildChannel;
use shuttle_secrets::SecretStore;
use tracing::error;

pub struct Secrets {
    pub discord_token: String,
    pub github_app_pem_key: String,
    pub github_personal_token: String,
    pub github_app_id: String,
    pub discord_server_staff_role_id: String,
    pub discord_server_id: String,
    pub oauth_id: String,
    pub oauth_secret: String,
}

pub fn get_secrets(secrets: SecretStore) -> Result<Secrets, anyhow::Error> {
    let discord_token = get_secret("DISCORD_TOKEN", secrets.clone());
    let github_app_pem_key = get_secret("GITHUB_APP_PRIVATE_KEY", secrets.clone());
    let github_personal_token = get_secret("GITHUB_PERSONAL_TOKEN", secrets.clone());
    let github_app_id = get_secret("GITHUB_APP_ID", secrets.clone());
    let discord_server_staff_role_id = get_secret("DISCORD_SERVER_STAFF_ROLE_ID", secrets.clone());
    let discord_server_id = get_secret("DISCORD_SERVER_ID", secrets.clone());
    let oauth_id = get_secret("GITHUB_OAUTH_ID", secrets.clone());
    let oauth_secret = get_secret("GITHUB_OAUTH_SECRET", secrets);

    Ok(Secrets {
        discord_token,
        github_app_pem_key,
        github_personal_token,
        github_app_id,
        discord_server_staff_role_id,
        discord_server_id,
        oauth_id,
        oauth_secret,
    })
}

fn get_secret(token: &str, secrets: SecretStore) -> String {
    secrets.get(token).unwrap_or_else(|| "None".to_string())
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

    pub fn url_from_poise_ctx(ctx: Context<'_>) -> String {
        format!(
            "https://discord.com/channels/{}/{}",
            ctx.guild_id().unwrap(),
            ctx.channel_id()
        )
    }
}
