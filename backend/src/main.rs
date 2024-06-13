mod api;
mod notifier;
mod processor;
mod store;

use std::{env, error::Error};

use notifier::Notifier;
use sentry::ClientInitGuard;
use store::Store;
use tracing_subscriber::{fmt, layer::SubscriberExt};

use crate::api::{event_task::start_event_task, routes::create_router};

fn init_tracing() {
    tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            // subscriber configuration
            .with_max_level(tracing::Level::DEBUG)
            .finish()
            // add additional writers
            .with(sentry::integrations::tracing::layer()),
    )
    .expect("Unable to set global tracing subscriber");
}

fn init_sentry(sentry_url: &str) -> ClientInitGuard {
    sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: 1.0,
            ..Default::default()
        },
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = dotenvy::dotenv() {
        tracing::info!(error = ?err, "Error loading dotenv" );
    }

    init_tracing();

    // sentry needs to be initialized before we start tokio
    let sentry = env::var("SENTRY_DSN").ok().map(|url| init_sentry(&url));
    tracing::info!(tracing = sentry.is_some(), "Running with tracing");

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Tokio couldn't build")
        .block_on(async { async_main().await })
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let store = Store::new(&env::var("DATABASE_URL").expect("Database URL not found")).await?;
    let notifier = Notifier::new(
        store.clone(),
        &env::var("NOTIFICATION_PRIVATE_KEY").expect("Priv key not found"),
    )?;

    let event_task_store = store.clone();
    let event_task_notifier = notifier.clone();

    let _handle = start_event_task(event_task_store, event_task_notifier);

    let router = create_router(store, notifier);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::debug!(
        "listening on {}",
        listener.local_addr().expect("Couldn't get local addr")
    );
    axum::serve(listener, router).await?;

    Ok(())
}
