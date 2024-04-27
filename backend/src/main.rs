use std::error::Error;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use event_processor::processor::Processor;
use notifier::Notifier;
use squiggle::{
    event::types::{CompleteEvent, Event},
    rest::Client,
};
use store::Store;
use tower_http::trace::TraceLayer;

async fn event_task(store: Store) -> Result<(), Box<dyn Error + Send + Sync>> {
    let rest_client = Client::new("sam.vr.lewis@gmail.com - footyalerts")?;
    let notifier = Notifier;

    let event_processor = Processor::new(store, rest_client, notifier);

    let event = Event::Complete(CompleteEvent {
        game_id: 35822,
        complete: 99,
    });

    event_processor.process_event(event).await?;

    Ok(())
}

#[derive(Clone)]
struct SharedState {
    store: Store,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let store = Store::new("sqlite:store/alerts.sqlite").await?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let _handle = tokio::spawn(event_task(store.clone()));

    let state = SharedState { store };

    let app = Router::new()
        .route("/health", get(health))
        .route("/games", get(games))
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health() -> &'static str {
    "healthy!"
}

async fn games(State(state): State<SharedState>) -> impl IntoResponse {
    // todo: Figure out logging
    let games = state.store.get_this_round_games().await.unwrap();
    let games: Result<Vec<_>, _> = games
        .into_iter()
        .map(squiggle::rest::types::Game::try_from)
        .collect();

    (StatusCode::OK, Json(games.unwrap()))
}
