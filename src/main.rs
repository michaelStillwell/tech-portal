use std::sync::Arc;

use axum::Router;
use dotenv;
use libsql::{Database, Row};
use shuttle_runtime::SecretStore;
use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod actions;
mod pages;

pub type Db = Arc<Database>;

#[derive(Clone)]
pub struct AppState {
    db: Db,
}

pub type Routes = Router<AppState>;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] _store: SecretStore,
    #[shuttle_turso::Turso(addr = "{secrets.TURSO_ADDR}", token = "{secrets.TURSO_TOKEN}")]
    db: Database,
) -> shuttle_axum::ShuttleAxum {
    let db = Arc::new(db);
    dotenv::dotenv().ok();

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();

    let state = AppState { db };
    let mut router = Router::new()
        .nest("/", pages::routes())
        .nest("/actions", actions::routes())
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        )
        .with_state(state);

    if cfg!(debug_assertions) {
        router = router.layer(tower_livereload::LiveReloadLayer::new());
    }

    Ok(router.into())
}

// NOTE: a first thought about general serialization
async fn _test(client: Database, action: fn(Row)) {
    let conn = client.connect().unwrap();
    let mut results = conn.query("SELECT * FROM items", ()).await.unwrap();

    while let Some(row) = results.next().await.unwrap() {
        action(row);
    }
}
