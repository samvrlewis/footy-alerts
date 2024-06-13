use std::time::Duration;

use axum::{
    extract::{Query, Request, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use serde::{Deserialize, Serialize};
use squiggle::{rest::types::Game, types::Team};
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::MakeRequestUuid,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

use crate::{
    api::{error::ApiError, response::ApiResponse},
    notifier::Notifier,
    store::Store,
};

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
        .layer(
            tower::ServiceBuilder::new()
                .set_x_request_id(MakeRequestUuid)
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .propagate_x_request_id()
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .layer(NewSentryLayer::<Request>::new_from_top())
                .layer(SentryHttpLayer::with_transaction())
                .layer(CorsLayer::permissive().allow_origin([
                    "https://footyalerts.fyi".parse().unwrap(),
                    "http://localhost:5173".parse().unwrap(),
                    "http://127.0.0.1:5173".parse().unwrap(),
                ]))
                .layer(CompressionLayer::new()),
        )
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

#[derive(Serialize)]
struct SubscriptionOptions {
    team: Option<Team>,
    close_games: bool,
    final_scores: bool,
    quarter_scores: bool,
}

impl From<crate::store::types::Subscription> for SubscriptionOptions {
    fn from(value: crate::store::types::Subscription) -> Self {
        Self {
            team: value.team,
            close_games: value.close_games,
            final_scores: value.final_scores,
            quarter_scores: value.quarter_scores,
        }
    }
}

async fn get_subscription(
    State(state): State<SharedState>,
    Query(params): Query<Params>,
) -> Result<ApiResponse<Option<SubscriptionOptions>>, ApiError> {
    let endpoint =
        urlencoding::decode(&params.endpoint).map_err(ApiError::SubscriptionUrlDecoding)?;
    let subscription = state.store.get_subscription_for_endpoint(&endpoint).await?;

    tracing::debug!("Trying to get subscription by endpoint {}", endpoint);

    let response = match subscription {
        None => ApiResponse::new(None, StatusCode::NOT_FOUND),
        Some(subscription) => ApiResponse::new(Some(subscription.into()), StatusCode::OK),
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

impl From<Subscription> for crate::store::types::Subscription {
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
    state.notifier.send_test_notification(&endpoint).await?;

    tracing::debug!("Sending test notification for {}", endpoint);
    Ok(ApiResponse::new((), StatusCode::OK))
}
