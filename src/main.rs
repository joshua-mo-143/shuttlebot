use octocrab::Octocrab;
use poise::serenity_prelude::GatewayIntents;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
mod bot;
mod commands;
mod utils;

use bot::Bot;
use commands::{docs, elevate, getchannel, hello};
use utils::get_secrets;

pub struct Data {
    pool: PgPool,
    crab: Octocrab,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let (discord_token, github_token) = get_secrets(secret_store).unwrap();

    let pool2 = pool.clone();

    let crab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .expect("Failed to build Octocrab instance");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![hello(), docs(), elevate(), getchannel()],
            ..Default::default()
        })
        .client_settings(|f| {
            f.intents(
                GatewayIntents::GUILDS
                    | GatewayIntents::GUILD_MESSAGES
                    | GatewayIntents::MESSAGE_CONTENT,
            )
            .event_handler(Bot { pool: pool2 })
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
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(framework.into())
}
