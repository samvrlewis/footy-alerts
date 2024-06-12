use std::{error::Error, fmt::Debug, time::Duration};

use event_processor::processor::Processor;
use futures_util::{pin_mut, StreamExt};
use notifier::Notifier;
use sentry::Hub;
use squiggle::{event, rest};
use store::Store;
use tokio::{task::JoinHandle, time::sleep};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
struct EventError(Box<dyn Error + Sync + Send>);

async fn event_task(store: Store, notifier: Notifier) -> Result<(), EventError> {
    let rest_client = rest::Client::new("sam.vr.lewis@gmail.com - footyalerts")
        .map_err(|err| EventError(Box::new(err)))?;
    let mut event_client = event::client::Client::new("sam.vr.lewis@gmail.com - footyalerts")
        .map_err(|err| EventError(Box::new(err)))?;
    let event_processor = Processor::new(store, rest_client, notifier);
    let stream = event_client.stream();

    pin_mut!(stream);

    while let Some(Ok(event)) = stream.next().await {
        if let Err(err) = event_processor.process_event(event).await {
            tracing::error!(?err, "Error ingesting event");
            Hub::current().capture_error(&err);
        }
    }

    Ok(())
}
pub fn start_event_task(event_task_store: Store, event_task_notifier: Notifier) -> JoinHandle<()> {
    tokio::spawn(async move {
        loop {
            let res = event_task(event_task_store.clone(), event_task_notifier.clone()).await;
            tracing::warn!("Event loop finished with {:?}", res);

            if let Err(err) = res {
                Hub::current().capture_error(&err);
            }

            // naive backoff for now, so we don't hammer squiggle
            sleep(Duration::from_secs(30)).await;
        }
    })
}
