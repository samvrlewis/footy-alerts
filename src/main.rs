use std::error::Error;

use event_processor::processor::Processor;
use notifier::Notifier;
use squiggle::{
    event::types::{CompleteEvent, Event},
    rest::Client,
};
use store::Store;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let store = Store::new("sqlite:store/alerts.sqlite").await?;
    let rest_client = Client::new("sam.vr.lewis@gmail.com - footyalerts")?;
    let notifier = Notifier;

    let event_processor = Processor::new(store, rest_client, notifier);

    let event = Event::Complete(CompleteEvent {
        game_id: 35740,
        complete: 99,
    });

    event_processor.process_event(event).await?;

    Ok(())
}
