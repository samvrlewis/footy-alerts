use async_stream::stream;
use futures::{Stream, StreamExt};
use reqwest::Method;
use reqwest_eventsource::{CannotCloneRequestError, EventSource, RequestBuilderExt};
use tracing::{debug, error, info, warn};

use super::types::Event;

pub struct Client {
    event_client: EventSource,
}

impl Client {
    pub fn new(user_agent: &str) -> Result<Self, CannotCloneRequestError> {
        let client = reqwest::Client::new()
            .request(Method::GET, "https://api.squiggle.com.au/sse/events")
            .header("user-agent", user_agent);

        let client = client.eventsource()?;

        Ok(Self {
            event_client: client,
        })
    }

    pub fn stream(&mut self) -> impl Stream<Item = Result<Event, reqwest_eventsource::Error>> + '_ {
        stream! {
            while let Some(event) = self.event_client.next().await {
                match event {
                    Ok(reqwest_eventsource::Event::Open) => println!("Connection Open!"),
                    Ok(reqwest_eventsource::Event::Message(message)) => {
                        if message.data == "\"Hello and welcome to the event channel for ALL events.\"" {
                            info!("Received welcome message");
                            continue
                        }

                        debug!(message.data, "Received message");

                        let event: Result<Event, _> = serde_json::from_str(&message.data);

                        match event {
                            Ok(event) => yield Ok(event),
                            Err(err) => {
                                warn!(
                                payload = ?message.data, error = ?err,
                                "Unable to deserialize event"
                                );
                            }
                        }
                    }
                    Err(err) => {
                        error!("Error: {}", err);
                        yield Err(err)
                    }
                }


            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     //#[tokio::test]
//     async fn it_works() {
//         let client = Client::new().expect("Client doesn't build");
//
//         while let Some(msg) = client.stream().next().await {
//             println!("Received {:?}", msg);
//         }
//     }
// }
