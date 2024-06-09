use std::{env, error::Error, time::Duration};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use event_processor::processor::Processor;
use futures_util::{pin_mut, StreamExt};
use notifier::Notifier;
use serde::Deserialize;
use squiggle::{event, rest, types::Team};
use store::Store;
use tokio::time::sleep;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[derive(Clone)]
struct SharedState {
    store: Store,
    notifier: Notifier,
}

async fn event_task(store: Store, notifier: Notifier) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = rest::Client::new("sam.vr.lewis@gmail.com - footyalerts")?;
    let mut event_client = event::client::Client::new("sam.vr.lewis@gmail.com - footyalerts")?;
    let event_processor = Processor::new(store, rest_client, notifier);
    let stream = event_client.stream();

    pin_mut!(stream);

    while let Some(Ok(event)) = stream.next().await {
        if let Err(err) = event_processor.process_event(event).await {
            tracing::error!(?err, "Error ingesting event");
        }
    }

    Ok(())
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Err(err) = dotenvy::dotenv() {
        tracing::info!(error = ?err, "Error loading dotenv" );
    }

    init_tracing();

    let store = Store::new(&env::var("DATABASE_URL").expect("Database URL not found")).await?;
    let notifier = Notifier::new(
        store.clone(),
        &env::var("NOTIFICATION_PRIVATE_KEY").expect("Priv key not found"),
    )?;

    let event_task_store = store.clone();
    let event_task_notifier = notifier.clone();

    let _handle = tokio::spawn(async move {
        loop {
            let res = event_task(event_task_store.clone(), event_task_notifier.clone()).await;
            tracing::warn!("Event loop finished with {:?}", res);

            // naive backoff for now, so we don't hammer squiggle
            sleep(Duration::from_secs(30)).await;
        }
    });

    let state = SharedState { store, notifier };

    let app = Router::new()
        .route("/health", get(health))
        .route("/games", get(games))
        .route("/subscription", get(get_subscription))
        .route("/subscription", post(create_subscription))
        .route("/test_notification", post(test_notification))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "healthy!"
}

async fn games(State(state): State<SharedState>) -> impl IntoResponse {
    // todo: Figure out errors
    let games = state.store.get_this_round_games().await.unwrap();
    let games: Result<Vec<_>, _> = games
        .into_iter()
        .map(squiggle::rest::types::Game::try_from)
        .collect();

    (StatusCode::OK, Json(games.unwrap()))
}

#[derive(Deserialize)]
struct Params {
    endpoint: String,
}

async fn get_subscription(
    State(state): State<SharedState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let endpoint = urlencoding::decode(&params.endpoint).unwrap();
    let subscription = state
        .store
        .get_subscription_for_endpoint(&endpoint)
        .await
        .unwrap();

    tracing::debug!("Trying to get subscription by endpoint {}", endpoint);

    match subscription {
        None => (StatusCode::NOT_FOUND, Json(None)),
        Some(subscription) => (StatusCode::OK, Json(Some(subscription))),
    }
}

#[derive(Deserialize)]
pub struct Keys {
    pub p256dh: String,
    pub auth: String,
}

#[derive(Deserialize)]
pub struct WebPush {
    pub endpoint: String,
    pub keys: Keys,
}

#[derive(Deserialize)]
struct Subscription {
    pub team: Option<Team>,
    pub close_games: bool,
    pub final_scores: bool,
    pub quarter_scores: bool,
    pub web_push: WebPush,
}

impl From<Subscription> for store::types::Subscription {
    fn from(value: Subscription) -> Self {
        Self {
            team: value.team,
            close_games: value.close_games,
            final_scores: value.final_scores,
            quarter_scores: value.quarter_scores,
            endpoint: value.web_push.endpoint,
            p256dh: value.web_push.keys.p256dh,
            auth: value.web_push.keys.auth,
        }
    }
}

async fn create_subscription(
    State(state): State<SharedState>,
    Json(subscription): Json<Subscription>,
) -> impl IntoResponse {
    state
        .store
        .add_subscription(subscription.into())
        .await
        .unwrap();

    (StatusCode::CREATED, Json(()))
}

async fn test_notification(
    State(state): State<SharedState>,
    Query(params): Query<Params>,
) -> impl IntoResponse {
    let endpoint = urlencoding::decode(&params.endpoint).unwrap();
    state
        .notifier
        .send_test_notification(&endpoint)
        .await
        .unwrap();

    tracing::debug!("Sending test notification for {}", endpoint);
    (StatusCode::OK, Json(()))
}
