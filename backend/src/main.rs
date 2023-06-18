use octocrab::Octocrab;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::path::PathBuf;
mod bot;
mod commands;
mod router;
mod utils;

use bot::init_discord_bot;
use router::init_router;
use utils::get_secrets;

pub struct Data {
    pool: PgPool,
    crab: Octocrab,
}

struct CustomService {
    pool: PgPool,
    bot: Bot,
    public: PathBuf,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

type Bot = poise::FrameworkBuilder<
    Data,
    Box<(dyn std::error::Error + std::marker::Send + Sync + 'static)>,
>;

#[shuttle_runtime::main]
async fn custom(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "public")] public: PathBuf,
) -> Result<CustomService, shuttle_runtime::Error> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Found an error while running migrations");

    // Get the discord token set in `Secrets.toml`
    let (discord_token, github_token) = get_secrets(secret_store).unwrap();

    let crab = Octocrab::builder()
        .personal_token(github_token)
        .build()
        .expect("Failed to build Octocrab instance");

    let bot = init_discord_bot(discord_token, pool.clone(), crab)
        .await
        .unwrap();

    Ok(CustomService { pool, bot, public })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = init_router(self.public, self.pool);

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select! {
            _ = self.bot.run() => {},
            _ = serve_router => {}
        };

        Ok(())
    }
}
