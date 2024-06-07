use std::{env, error::Error};

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use event_processor::processor::Processor;
use notifier::Notifier;
use serde::Deserialize;
use squiggle::{
    event::types::{Event, TimeStrEvent},
    rest::Client,
    types::{Team, TimeStr},
};
use store::Store;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

async fn event_task(store: Store, notifier: Notifier) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = Client::new("sam.vr.lewis@gmail.com - footyalerts")?;

    let event_processor = Processor::new(store, rest_client, notifier);

    let event = Event::TimeStr(TimeStrEvent {
        game_id: 35805,

        timestr: TimeStr::EndOfFirstQuarter,
    });

    event_processor.process_event(event).await?;

    Ok(())
}

#[derive(Clone)]
struct SharedState {
    store: Store,
    notifier: Notifier
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    if let Err(err) = dotenvy::dotenv() {
        tracing::info!(error = ?err, "Error loading dotenv" );
    }

    let store = Store::new("sqlite:store/alerts.sqlite").await?;
    let notifier = Notifier::new(
        store.clone(),
        &env::var("NOTIFICATION_PRIVATE_KEY").expect("Priv key not found"),
    )?;

    let event_task_store = store.clone();
    let event_task_notifier = notifier.clone();

    let _handle = tokio::spawn(async move {
        let res = event_task(event_task_store, event_task_notifier).await;

        tracing::debug!("Event loop finished with {:?}", res);
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

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
        .notifier.send_test_notification(&endpoint).await.unwrap();

    tracing::debug!("Sending test notification for {}", endpoint);
    (StatusCode::OK, Json(()))
}