use poise::serenity_prelude::{
    async_trait,
    model::{channel::GuildChannel, gateway::Ready},
    Context, EventHandler,
};
use tracing::{error, info};

pub struct Bot {
    pub pool: sqlx::PgPool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn thread_create(&self, ctx: Context, thread: GuildChannel) {
        if let Err(e) = sqlx::query("INSERT INTO issues () VALUES ()")
            .execute(&self.pool)
            .await
        {
            error!("Error inserting issue to db: {:?}", e);
        }

        if let Err(e) = thread.say(ctx.http, "Thanks for reporting this issue! We'll be looking into this soon and you should receive a response shortly.").await {
            error!("Error sending message: {:?}", e);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}
