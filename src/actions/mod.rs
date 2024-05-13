use crate::actions::events::edit_event_action;
use crate::actions::login::login_action;
use askama::Template;
use rocket::routes;
use rocket::{get, Route};

mod login;
mod events;

pub fn routes() -> Vec<Route> {
    routes![login_action, edit_event_action, test_action]
}

#[derive(Template)]
#[template(path = "components/test.html")]
struct TestComponent;

#[get("/test")]
fn test_action() -> TestComponent {
    TestComponent
}
