#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = git_listener::config::Settings::load()?;

    git_listener::telemetry::init(&settings.logging)?;

    tracing::info!(
        "starting {} v{}",
        settings.app.name,
        settings.app.version
    );

    let db = git_listener::database::pool::create_pool(&settings.database).await?;

    if settings.database.run_migrations {
        let migrator = sqlx::migrate::Migrator::new(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations"),
        )
        .await?;
        migrator.run(&db).await?;
        tracing::info!("database migrations applied");
    }

    let state = git_listener::AppState::new(settings, db).await?;

    let webhook_handle = tokio::spawn(git_listener::webhooks::serve(
        state.clone(),
    ));

    let api_handle = if state.config.api.enabled {
        Some(tokio::spawn(git_listener::api::serve(state.clone())))
    } else {
        None
    };

    git_listener::discord::start(state).await?;

    webhook_handle.abort();
    if let Some(handle) = api_handle {
        handle.abort();
    }

    Ok(())
}
