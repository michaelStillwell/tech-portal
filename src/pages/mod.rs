use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::State,
    routing::{get, Router},
    Json,
};
use tracing::info;

use crate::{AppState, Routes};

#[derive(Template)]
#[template(path = "pages/hello.html")]
pub struct HomeTemplate;

pub async fn home(State(state): State<AppState>) -> HomeTemplate {
    info!("hitting home");
    let _conn = state.db.connect().unwrap();
    HomeTemplate
}

pub fn routes() -> Routes {
    Router::new()
        .route("/", get(home))
}
