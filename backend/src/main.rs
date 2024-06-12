mod api_error;
mod api_response;
mod events;
mod routes;

use std::{env, error::Error};

use notifier::Notifier;
use sentry::ClientInitGuard;
use store::Store;

use crate::{events::start_event_task, routes::create_router};

fn init_tracing() {
    if env::var("LOG_FORMAT").is_ok_and(|format| format == "json") {
        tracing_subscriber::fmt()
            .json()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    } else {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }
}

fn init_sentry(sentry_url: &str) -> ClientInitGuard {
    sentry::init((
        sentry_url,
        sentry::ClientOptions {
            release: sentry::release_name!(),
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
        .unwrap()
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

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await?;

    Ok(())
}
