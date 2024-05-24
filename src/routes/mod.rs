use rocket::{routes, Route};

mod login;
mod home;
mod site;
mod events;

pub fn routes() -> Vec<Route> {
    routes![
        login::login_page,
        login::login_page_with_session,
        login::actions::login_action,

        home::home_page,

        site::site_page,
        site::new_event_page,
        site::actions::add_event_action,
    ]
}
