use octocrab::Octocrab;
use poise::serenity_prelude as serenity;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
mod bot;
mod commands;
mod utils;

use bot::Bot;
use commands::DocsLinks;
use utils::get_secrets;

pub struct Data {
    pool: PgPool,
    crab: Octocrab,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Responds with "world!"
#[poise::command(slash_command)]
async fn hello(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("world!").await?;
    Ok(())
}

/// Link to Shuttle documentation
#[poise::command(slash_command)]
pub async fn docs(
    ctx: Context<'_>,
    #[description = "The docs you want to link to"] docs: DocsLinks,
) -> Result<(), Error> {
    ctx.say(docs.to_link()).await?;
    Ok(())
}

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
            commands: vec![hello(), docs()],
            ..Default::default()
        })
        .client_settings(|f| {
            f.intents(
                serenity::GatewayIntents::GUILDS
                    | serenity::GatewayIntents::GUILD_MESSAGES
                    | serenity::GatewayIntents::MESSAGE_CONTENT,
            )
            .event_handler(Bot { pool: pool2 })
        })
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
