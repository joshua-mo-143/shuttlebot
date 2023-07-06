use octocrab::Octocrab;
use shuttle_persist::PersistInstance;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use std::path::PathBuf;
use tokio::time::{sleep, Duration};
use poise::serenity_prelude::http::client::Http;
mod bot;
mod commands;
mod database;
mod github;
mod oauth;
mod persist;
mod router;
mod utils;

use bot::{init_discord_bot, monitor_service};
use database::DBQueries;
use github::Github;
use persist::Persist;
use router::init_router;
use utils::get_secrets;

pub struct Data {
    db: DBQueries,
    crab: Octocrab,
    staff_role_id: String,
    server_id: String,
}

struct CustomService {
    http: Http,
    db: DBQueries,
    bot: Bot,
    public: PathBuf,
    oauth_id: String,
    oauth_secret: String,
    persist: PersistInstance,
    crab: Octocrab,
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
    #[shuttle_shared_db::Postgres] db: PgPool,
    #[shuttle_static_folder::StaticFolder(folder = "public")] public: PathBuf,
    #[shuttle_persist::Persist] persist: PersistInstance,
) -> Result<CustomService, shuttle_runtime::Error> {
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Found an error while running migrations");

    let db = DBQueries { db };

    // Get the discord token set in `Secrets.toml`
    let secrets = get_secrets(secret_store).unwrap();

    // set up octocrab instance
    // if the PEM key and app ID exist, initiate as app - otherwise, initiate using personal key
    let crab = if secrets.github_app_id != *"None" && secrets.github_app_pem_key != *"None" {
        Github::init_as_app(secrets.github_app_id, secrets.github_app_pem_key)
            .await
            .unwrap()
    } else {
        Github::init_as_personal(secrets.github_personal_token).unwrap()
    };

    let bot = init_discord_bot(
        &secrets.discord_token,
        db.clone(),
        crab.clone(),
        secrets.discord_server_staff_role_id,
        secrets.discord_server_id,
    )
    .await
    .unwrap();

    let http = Http::new(&secrets.discord_token.to_owned());

    Ok(CustomService {
        http,
        db,
        bot,
        public,
        oauth_id: secrets.oauth_id,
        oauth_secret: secrets.oauth_secret,
        persist,
        crab,
    })
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = init_router(
            self.public,
            self.db,
            self.oauth_id,
            self.oauth_secret,
            self.persist.clone(),
            self.crab,
        );

        let serve_router = axum::Server::bind(&addr).serve(router.into_make_service());

        tokio::select! {
            _ = self.bot.run() => {},
            _ = serve_router => {},
            _ = remove_expired_sessions(self.persist) => {},
            _ = monitor_service(self.http) => {}
        };

        Ok(())
    }
}

#[allow(unreachable_code)]
pub async fn remove_expired_sessions(persist: PersistInstance) -> Result<(), anyhow::Error> {
    loop {
        Persist::filter_records(persist.clone())
            .expect("Error occurred while filtering out expired sessions");

        sleep(Duration::from_secs(300)).await;
    }

    Ok(())
}