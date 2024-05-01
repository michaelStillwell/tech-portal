use askama::Template;
use axum::{routing::get, Router};

use crate::Routes;

pub async fn hello() -> &'static str {
    "Hello"
}

#[derive(Template)]
#[template(path = "components/test.html")]
pub struct TestComponent;

pub async fn test_action() -> TestComponent {
    TestComponent
}

pub fn routes() -> Routes {
    Router::new()
    .route("/hello", get(hello))
    .route("/test", get(test_action))
}
