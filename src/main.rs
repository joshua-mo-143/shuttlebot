use octocrab::Octocrab;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

mod bot;
mod commands;
mod utils;

use bot::Bot;
use utils::get_secrets;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let (discord_token, github_token) = get_secrets(secret_store).unwrap();

    let octocrab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .expect("Failed to build Octocrab instance");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&discord_token, intents)
        .event_handler(Bot { pool, octocrab })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
