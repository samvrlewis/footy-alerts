use std::{fmt, fmt::Formatter};

use futures::{StreamExt, TryFutureExt};
use squiggle::{
    rest::types::Game,
    types::{Team, TimeStr},
};
use store::Store;
use web_push::{
    ContentEncoding, IsahcWebPushClient, PartialVapidSignatureBuilder, SubscriptionInfo,
    SubscriptionKeys, VapidSignatureBuilder, WebPushClient, WebPushError, WebPushMessageBuilder,
    URL_SAFE_NO_PAD,
};
use store::types::Subscription;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Web push: {0}")]
    WebPush(#[from] WebPushError),
    #[error("Store: {0}")]
    Store(#[from] store::Error),
}

#[derive(Debug, thiserror::Error)]
enum PushError {
    #[error("Push failed for endpoint: {1} with error {0}")]
    Expired(WebPushError, String),
    #[error("Transient error {0}")]
    Other(WebPushError),
}

#[derive(Debug, thiserror::Error)]
pub enum InitError {
    #[error("sig builder: {0}")]
    SigBuilder(WebPushError),
    #[error("client: {0}")]
    Client(WebPushError),
}

#[derive(Clone)]
pub struct Notifier {
    store: Store,
    sig_builder: PartialVapidSignatureBuilder,
    client: IsahcWebPushClient,
}

#[derive(Debug)]
pub enum Quarter {
    First,
    Second,
    Third,
}

impl fmt::Display for Quarter {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Quarter::First => write!(f, "Q1"),
            Quarter::Second => write!(f, "Q2"),
            Quarter::Third => write!(f, "Q3"),
        }
    }
}

#[derive(Debug)]
pub enum Notification {
    EndOfQuarter {
        quarter: Quarter,
        home_team: Team,
        away_team: Team,
        home_score: u16,
        away_score: u16,
    },
    EndOfGame {
        home_team: Team,
        away_team: Team,
        home_score: u16,
        away_score: u16,
    },
    CloseGame {
        home_team: Team,
        away_team: Team,
        home_score: u16,
        away_score: u16,
        time_str: TimeStr,
    },
}

impl Notification {
    fn to_notification_text(&self) -> String {
        match self {
            Notification::EndOfQuarter {
                quarter,
                home_team,
                away_team,
                home_score,
                away_score,
            } => {
                format!("End of {quarter}: {home_team} {home_score} - {away_team} {away_score}")
            }
            Notification::EndOfGame {
                home_team,
                away_team,
                home_score,
                away_score,
            } => {
                format!("End of game: {home_team} {home_score} - {away_team} {away_score}")
            }
            Notification::CloseGame {
                home_team,
                away_team,
                home_score,
                away_score,
                time_str,
            } => {
                format!(
                    "Close game ({time_str}): {home_team} {home_score} - {away_team} {away_score}"
                )
            }
        }
    }
}

impl From<&Notification> for store::types::Notification {
    fn from(value: &Notification) -> Self {
        match value {
            Notification::EndOfQuarter { quarter, .. } => match quarter {
                Quarter::First => store::types::Notification::EndOfFirstQuarter,
                Quarter::Second => store::types::Notification::EndOfSecondQuarter,
                Quarter::Third => store::types::Notification::EndOfThirdQuarter,
            },
            Notification::EndOfGame { .. } => store::types::Notification::EndOfGame,
            Notification::CloseGame { .. } => store::types::Notification::CloseGame,
        }
    }
}

impl Notifier {
    pub fn new(store: Store, private_key: &str) -> Result<Self, InitError> {
        let sig_builder = VapidSignatureBuilder::from_base64_no_sub(private_key, URL_SAFE_NO_PAD)
            .map_err(InitError::SigBuilder)?;
        let client = IsahcWebPushClient::new().map_err(InitError::Client)?;
        Ok(Self {
            store,
            sig_builder,
            client,
        })
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn notify(&self, game: Game, notification: Notification) -> Result<(), Error> {
        let db_notification = store::types::Notification::from(&notification);
        let users_to_notify = self
            .store
            .get_subscriptions_for_notification(game.home_team, game.away_team, db_notification)
            .await?;

        let notification = notification.to_notification_text();

        let futures = users_to_notify
            .into_iter()
            .map(|user| async {

                self.send_user_notification(&notification, user).await
            })
            .collect::<Vec<_>>();

        let stream = futures::stream::iter(futures).buffer_unordered(10);
        let results = stream.collect::<Vec<Result<(), PushError>>>().await;

        for res in results {
            let Err(err) = res else { continue };

            match err {
                PushError::Expired(err, endpoint) => {
                    tracing::info!(error=?err, endpoint, "Error indicating endpoint expired");
                    if let Err(err) = self.store.delete_subscription(&endpoint).await {
                        tracing::error!(?err, endpoint, "Couldn't delete expired subscription");
                    }
                }
                PushError::Other(err) => {
                    tracing::warn!(error=?err, "Transient web push error");
                }
            }
        }

        Ok(())
    }

    #[tracing::instrument(skip(self), err)]

    async fn send_user_notification(&self, notification: &str, user: Subscription) -> Result<(), PushError> {
        let endpoint = user.endpoint.clone();
        let subscription = SubscriptionInfo {
            endpoint: user.endpoint,
            keys: SubscriptionKeys {
                p256dh: user.p256dh,
                auth: user.auth,
            },
        };

        let signature = self
            .sig_builder
            .clone()
            .add_sub_info(&subscription)
            .build()
            .unwrap();

        //Now add payload and encrypt.
        let mut builder = WebPushMessageBuilder::new(&subscription);
        let content = notification.as_bytes();
        builder.set_payload(ContentEncoding::Aes128Gcm, content);
        builder.set_vapid_signature(signature);

        self.client
            .send(builder.build().unwrap())
            .map_err(|err| match err {
                WebPushError::EndpointNotValid | WebPushError::EndpointNotFound => {
                    PushError::Expired(err, endpoint)
                }
                err => PushError::Other(err),
            }).await
    }

    #[tracing::instrument(skip(self), err)]
    pub async fn send_test_notification(&self, endpoint: &str) -> Result<(), Error> {
        let maybe_subscription = self.store.get_subscription_for_endpoint(endpoint).await?;
        let Some(subscription) = maybe_subscription else {
            tracing::info!(endpoint, "Couldn't find subscription");
            return Ok(());
        };

        let utc_now = chrono::Utc::now();
        let aest_now = utc_now.with_timezone(&chrono_tz::Australia::Melbourne).format("%Y-%m-%d %H:%M:%S %Z");
        let notification = format!("Test notification from FootyAlerts ({aest_now})");

        self.send_user_notification(&notification, subscription).await.unwrap();

        Ok(())
    }
}
