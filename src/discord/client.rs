use poise::serenity_prelude as serenity;
use std::sync::Arc;

use crate::AppState;

pub struct Data {
    pub state: Arc<AppState>,
}

pub type Error = anyhow::Error;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn run(state: AppState) -> anyhow::Result<()> {
    let token = state.config.discord.token.clone();
    let intents = serenity::GatewayIntents::from_bits_truncate(state.config.discord.intents);

    let commands = vec![
        crate::discord::commands::setup::setup(),
        crate::discord::commands::repository::repository(),
        crate::discord::commands::repository::add(),
        crate::discord::commands::repository::remove(),
        crate::discord::commands::repository::list(),
        crate::discord::commands::repository::info(),
        crate::discord::commands::provider::provider(),
        crate::discord::commands::provider::configure(),
        crate::discord::commands::provider::info(),
        crate::discord::commands::channel::channel(),
        crate::discord::commands::events::events(),
        crate::discord::commands::events::enable(),
        crate::discord::commands::events::disable(),
        crate::discord::commands::events::list(),
        crate::discord::commands::filters::filters(),
        crate::discord::commands::filters::add(),
        crate::discord::commands::filters::remove(),
        crate::discord::commands::filters::list(),
        crate::discord::commands::test::test(),
        crate::discord::commands::ping::ping(),
        crate::discord::commands::status::status(),
        crate::discord::commands::stats::stats(),
        crate::discord::commands::templates::templates(),
        crate::discord::commands::templates::set(),
        crate::discord::commands::templates::reset(),
        crate::discord::commands::templates::show(),
        crate::discord::commands::preview::preview(),
        crate::discord::commands::help::help(),
        crate::discord::commands::permissions::permissions(),
        crate::discord::commands::reload::reload(),
        crate::discord::commands::about::about(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, _data| {
                Box::pin(async move {
                    handle_event(ctx, event).await;
                    Ok(())
                })
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    state: Arc::new(state),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?;

    client.start().await?;

    Ok(())
}

async fn handle_event(_ctx: &serenity::Context, event: &poise::serenity_prelude::FullEvent) {
    match event {
        poise::serenity_prelude::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("logged in as {}", data_about_bot.user.name);
        }
        poise::serenity_prelude::FullEvent::GuildCreate { guild, is_new } => {
            if is_new.unwrap_or(false) {
                tracing::info!("joined guild: {} ({})", guild.name, guild.id);
            }
        }
        _ => {}
    }
}
