use eventsource_client::SSE;
use futures::{stream::BoxStream, StreamExt};
use tracing::warn;

use super::types::Event;

pub struct Client {
    event_client: Box<dyn eventsource_client::Client>,
}

impl Client {
    pub fn new(user_agent: &str) -> Result<Self, eventsource_client::Error> {
        let client =
            eventsource_client::ClientBuilder::for_url("https://api.squiggle.com.au/sse/events")?
                .header("user-agent", user_agent)?
                .build();

        Ok(Self {
            event_client: Box::new(client),
        })
    }

    #[must_use]
    pub fn stream(&self) -> BoxStream<Result<Event, eventsource_client::Error>> {
        let stream = self.event_client.stream().filter_map(|event| async {
            let event = match event {
                Ok(event) => event,
                Err(err) => return Some(Err(err)),
            };

            if let SSE::Event(raw_event) = event {
                let event: Result<Event, _> = serde_json::from_str(&raw_event.data);

                match event {
                    Ok(event) => return Some(Ok(event)),
                    Err(err) => {
                        warn!(
                            payload = ?raw_event.data, error = ?err,
                            "Unable to deserialize event"
                        );
                    }
                }
            }

            None
        });
        Box::pin(stream)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[tokio::test]
    async fn it_works() {
        let client = Client::new().expect("Client doesn't build");

        while let Some(msg) = client.stream().next().await {
            println!("Received {:?}", msg);
        }
    }
}
