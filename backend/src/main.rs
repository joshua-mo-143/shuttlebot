use octocrab::Octocrab;
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::path::PathBuf;

mod bot;
mod commands;
mod github;
mod oauth;
mod router;
mod utils;

use bot::init_discord_bot;
use github::init_github_app;
use router::init_router;
use utils::get_secrets;

pub struct Data {
    pool: PgPool,
    crab: Octocrab,
    staff_role_id: String,
    server_id: String,
}

struct CustomService {
    pool: PgPool,
    bot: Bot,
    public: PathBuf,
    oauth_id: String,
    oauth_secret: String,
    persist: PersistInstance,
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
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> Result<CustomService, shuttle_runtime::Error> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Found an error while running migrations");

    // Get the discord token set in `Secrets.toml`
    let secrets = get_secrets(secret_store).unwrap();

    let crab = init_github_app(secrets.github_app_id, secrets.github_app_pem_key)
        .await
        .unwrap();

    let bot = init_discord_bot(
        &secrets.discord_token,
        pool.clone(),
        crab,
        secrets.discord_server_staff_role_id,
        secrets.discord_server_id,
    )
    .await
    .unwrap();

    Ok(CustomService {
        pool,
        bot,
        public,
        oauth_id: secrets.oauth_id,
        oauth_secret: secrets.oauth_secret,
        persist,
    })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = init_router(
            self.public,
            self.pool,
            self.oauth_id,
            self.oauth_secret,
            self.persist,
        );

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select! {
            _ = self.bot.run() => {},
            _ = serve_router => {}
        };

        Ok(())
    }
}
