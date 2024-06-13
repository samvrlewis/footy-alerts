use footy_alerts::{
    notifier::Notifier,
    processor::Processor,
    store::{types::Subscription, Store},
};
use httptest::{matchers::*, responders::*, Expectation, Server};
use sqlx::SqlitePool;
use squiggle::{
    event::types::{CompleteEvent, Event, TimeStrEvent},
    rest::Client,
    types::{Team, TimeStr},
};

const TEST_PRIVATE_KEY: &str = "EHXHoeBHyP8hqP5pHfvTRSNXUATFGQUlwBgejQT80qM";
const TEST_AUTH: &str = "ENDd0ot5n0ftnJlA658u9Q";
const TEST_P256DH: &str =
    "BJO9qnHW0_lKWB351O6Y-3M0hjoOUI0tBcBy7q9WqL1WWLdGSkBDZhXSs5saIGJ73MUlmzn4Et_Tn0FuU225wK4";

static SERVER_POOL: httptest::ServerPool = httptest::ServerPool::new(5);

fn create_processor(pool: SqlitePool, mock_squiggle_url: String) -> Processor {
    let store = Store::new_from_pool(pool);
    let client = Client::new("test-user-agent")
        .expect("Client creation")
        .with_base_url(mock_squiggle_url);

    let notifier = Notifier::new(store.clone(), TEST_PRIVATE_KEY).expect("Notifier creation");
    Processor::new(store, client, notifier)
}

fn expect_squiggle_response(mock_server: &Server, query: &str, response: &str) {
    let val: serde_json::Value = serde_json::from_str(response).expect("Unable to parse JSON");
    mock_server.expect(
        Expectation::matching(all_of![
            request::method_path("GET", "/mock_squiggle/"),
            request::query(query.to_string())
        ])
        .respond_with(json_encoded(val)),
    );
}

fn expect_notification(mock_server: &Server, path: &str) {
    mock_server.expect(
        Expectation::matching(request::method_path("POST", path.to_string()))
            .respond_with(status_code(200)),
    );
}

struct TestSubscriptionBuilder {
    team: Option<Team>,
    close_games: bool,
    final_scores: bool,
    quarter_scores: bool,
    endpoint: String,
    p256dh: Option<String>,
    auth: Option<String>,
}

impl TestSubscriptionBuilder {
    #[must_use]
    fn new(endpoint: String) -> Self {
        Self {
            team: None,
            close_games: false,
            final_scores: false,
            quarter_scores: false,
            endpoint,
            p256dh: None,
            auth: None,
        }
    }
    #[must_use]
    fn team(mut self, team: Team) -> Self {
        self.team = Some(team);
        self
    }
    #[must_use]
    fn close_games(mut self) -> Self {
        self.close_games = true;
        self
    }
    #[must_use]
    fn final_scores(mut self) -> Self {
        self.final_scores = true;
        self
    }
    #[must_use]
    fn quarter_scores(mut self) -> Self {
        self.quarter_scores = true;
        self
    }
    #[must_use]
    fn build(self) -> Subscription {
        Subscription {
            team: self.team,
            close_games: self.close_games,
            final_scores: self.final_scores,
            quarter_scores: self.quarter_scores,
            endpoint: self.endpoint,
            p256dh: self.p256dh.unwrap_or_else(|| TEST_P256DH.to_string()),
            auth: self.auth.unwrap_or_else(|| TEST_AUTH.to_string()),
        }
    }
}

#[sqlx::test]
async fn it_sends_notification_on_game_end(pool: SqlitePool) -> sqlx::Result<()> {
    let mock_server = SERVER_POOL.get_server();
    let processor = create_processor(pool.clone(), mock_server.url_str("/mock_squiggle/"));

    let store = Store::new_from_pool(pool);

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_1/"))
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    expect_squiggle_response(
        &mock_server,
        "q=games;game=35740",
        include_str!("example_game.json"),
    );

    expect_squiggle_response(
        &mock_server,
        "q=games;year=2024;round=5",
        include_str!("example_round.json"),
    );

    expect_notification(&mock_server, "/mock_notification_1/");

    processor
        .process_event(Event::TimeStr(TimeStrEvent {
            game_id: 35740,
            timestr: TimeStr::EndOfGame,
        }))
        .await
        .expect("Couldn't process");

    Ok(())
}

#[sqlx::test]
async fn it_sends_multiple_notification_on_game_end(pool: SqlitePool) -> sqlx::Result<()> {
    let mock_server = SERVER_POOL.get_server();
    let processor = create_processor(pool.clone(), mock_server.url_str("/mock_squiggle/"));

    let store = Store::new_from_pool(pool);

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_1/"))
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_2/"))
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    expect_squiggle_response(
        &mock_server,
        "q=games;game=35740",
        include_str!("example_game.json"),
    );

    expect_squiggle_response(
        &mock_server,
        "q=games;year=2024;round=5",
        include_str!("example_round.json"),
    );

    expect_notification(&mock_server, "/mock_notification_1/");
    expect_notification(&mock_server, "/mock_notification_2/");

    processor
        .process_event(Event::TimeStr(TimeStrEvent {
            game_id: 35740,
            timestr: TimeStr::EndOfGame,
        }))
        .await
        .expect("Couldn't process");

    Ok(())
}

#[sqlx::test]
async fn it_filters_notifications_by_team(pool: SqlitePool) -> sqlx::Result<()> {
    let mock_server = SERVER_POOL.get_server();
    let processor = create_processor(pool.clone(), mock_server.url_str("/mock_squiggle/"));

    let store = Store::new_from_pool(pool);

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_1/"))
        .team(Team::Geelong)
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_2/"))
        .team(Team::StKilda)
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    expect_squiggle_response(
        &mock_server,
        "q=games;game=35740",
        include_str!("example_game.json"),
    );

    expect_squiggle_response(
        &mock_server,
        "q=games;year=2024;round=5",
        include_str!("example_round.json"),
    );

    expect_notification(&mock_server, "/mock_notification_2/");

    processor
        .process_event(Event::TimeStr(TimeStrEvent {
            game_id: 35740,
            timestr: TimeStr::EndOfGame,
        }))
        .await
        .expect("Couldn't process");

    Ok(())
}

#[sqlx::test]
async fn it_filters_notifications_by_quarter_full_selection(pool: SqlitePool) -> sqlx::Result<()> {
    let mock_server = SERVER_POOL.get_server();
    let processor = create_processor(pool.clone(), mock_server.url_str("/mock_squiggle/"));

    let store = Store::new_from_pool(pool);

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_1/"))
        .quarter_scores()
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_2/"))
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    expect_squiggle_response(
        &mock_server,
        "q=games;game=35740",
        include_str!("example_game.json"),
    );

    expect_squiggle_response(
        &mock_server,
        "q=games;year=2024;round=5",
        include_str!("example_round.json"),
    );

    expect_notification(&mock_server, "/mock_notification_1/");

    processor
        .process_event(Event::TimeStr(TimeStrEvent {
            game_id: 35740,
            timestr: TimeStr::EndOfThirdQuarter,
        }))
        .await
        .expect("Couldn't process");

    Ok(())
}

#[sqlx::test]
async fn it_filters_notifications_by_close_selection(pool: SqlitePool) -> sqlx::Result<()> {
    let mock_server = SERVER_POOL.get_server();
    let processor = create_processor(pool.clone(), mock_server.url_str("/mock_squiggle/"));

    let store = Store::new_from_pool(pool);

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_1/"))
        .quarter_scores()
        .final_scores()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    let subscription = TestSubscriptionBuilder::new(mock_server.url_str("/mock_notification_2/"))
        .final_scores()
        .close_games()
        .build();

    store
        .add_subscription(subscription)
        .await
        .expect("Couldn't add subscription");

    expect_squiggle_response(
        &mock_server,
        "q=games;game=35740",
        include_str!("example_game.json"),
    );

    expect_squiggle_response(
        &mock_server,
        "q=games;year=2024;round=5",
        include_str!("example_round.json"),
    );

    expect_notification(&mock_server, "/mock_notification_2/");

    processor
        .process_event(Event::Complete(CompleteEvent {
            game_id: 35740,
            complete: 99,
        }))
        .await
        .expect("Couldn't process");

    Ok(())
}
