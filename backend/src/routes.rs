use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use notifier::Notifier;
use serde::Deserialize;
use squiggle::{rest::types::Game, types::Team};
use store::Store;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{api_error::ApiError, api_response::ApiResponse};

#[derive(Clone)]
struct SharedState {
    store: Store,
    notifier: Notifier,
}

pub fn create_router(store: Store, notifier: Notifier) -> Router {
    let state = SharedState { store, notifier };

    Router::new()
        .route("/health", get(health))
        .route("/games", get(games))
        .route("/subscription", get(get_subscription))
        .route("/subscription", post(create_subscription))
        .route("/test_notification", post(test_notification))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive())
}

async fn health() -> &'static str {
    "healthy!"
}

async fn games(State(state): State<SharedState>) -> Result<ApiResponse<Vec<Game>>, ApiError> {
    let games = state.store.get_this_round_games().await?;
    let games: Vec<_> = games
        .into_iter()
        .map(squiggle::rest::types::Game::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(ApiError::GameConversion)?;

    Ok(ApiResponse::new(games, StatusCode::OK))
}

#[derive(Deserialize)]
struct Params {
    endpoint: String,
}

async fn get_subscription(
    State(state): State<SharedState>,
    Query(params): Query<Params>,
) -> Result<ApiResponse<Option<store::types::Subscription>>, ApiError> {
    let endpoint =
        urlencoding::decode(&params.endpoint).map_err(ApiError::SubscriptionUrlDecoding)?;
    let subscription = state.store.get_subscription_for_endpoint(&endpoint).await?;

    tracing::debug!("Trying to get subscription by endpoint {}", endpoint);

    let response = match subscription {
        None => ApiResponse::new(None, StatusCode::NOT_FOUND),
        Some(subscription) => ApiResponse::new(Some(subscription), StatusCode::OK),
    };

    Ok(response)
}

#[derive(Deserialize)]
struct Keys {
    pub p256dh: String,
    pub auth: String,
}

#[derive(Deserialize)]
struct WebPush {
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
) -> Result<ApiResponse<()>, ApiError> {
    state.store.add_subscription(subscription.into()).await?;

    Ok(ApiResponse::new((), StatusCode::CREATED))
}

async fn test_notification(
    State(state): State<SharedState>,
    Query(params): Query<Params>,
) -> Result<ApiResponse<()>, ApiError> {
    let endpoint =
        urlencoding::decode(&params.endpoint).map_err(ApiError::SubscriptionUrlDecoding)?;
    state
        .notifier
        .send_test_notification(&endpoint)
        .await
        .unwrap();

    tracing::debug!("Sending test notification for {}", endpoint);
    Ok(ApiResponse::new((), StatusCode::OK))
}
