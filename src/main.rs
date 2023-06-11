use octocrab::Octocrab;
use shuttle_poise::ShuttlePoise;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
mod bot;
mod commands;
mod utils;

use bot::init_discord_bot;
use utils::get_secrets;

pub struct Data {
    pool: PgPool,
    crab: Octocrab,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

type BotInit = std::sync::Arc<
    poise::Framework<Data, Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>>,
>;

#[shuttle_runtime::main]
async fn poise(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttlePoise<Data, Error> {
    // Get the discord token set in `Secrets.toml`
    let (discord_token, github_token) = get_secrets(secret_store).unwrap();

    let crab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .expect("Failed to build Octocrab instance");

    let framework = init_discord_bot(discord_token, pool, crab).await.unwrap();

    Ok(framework.into())
}
