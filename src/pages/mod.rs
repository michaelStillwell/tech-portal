use rocket::{routes, Route};

mod home;
mod login;
mod events;

pub fn routes() -> Vec<Route> {
    routes![
        home::home_page,
        events::events_page,
        events::edit_event_page,
        login::login_page,
        login::login_page_with_session
    ]
}
