use crate::{
    auth::AuthedUser,
    db::{
        event::{get_events_by_site, Event},
        site::{get_site, Site},
    },
    AppState,
};
use askama::Template;
use rocket::{get, response::Redirect, Responder, State};

pub mod actions;

#[derive(Template)]
#[template(path = "pages/site.html")]
pub struct SitePage {
    pub site: Site,
    pub events: Vec<Event>,
}

#[derive(Responder)]
pub enum SiteResponse {
    Success(SitePage),
    Failure(Redirect),
}

#[get("/site/<site_id>")]
pub async fn site_page(state: &State<AppState>, user: AuthedUser, site_id: i64) -> SiteResponse {
    let conn = state.db.conn();

    match get_site(&conn, site_id).await {
        Some(site) => {
            let events = get_events_by_site(&conn, user.user_id).await;

            SiteResponse::Success(SitePage { site, events })
        }
        // TODO: this could probably end up in an infinite loop
        None => SiteResponse::Failure(Redirect::to("/")),
    }
}

#[derive(Template)]
#[template(path = "pages/new_event.html")]
pub struct NewEventPage {
    pub site: Site,
}

#[get("/site/<site_id>/events/new")]
pub async fn new_event_page(
    state: &State<AppState>,
    _user: AuthedUser,
    site_id: i64,
) -> NewEventPage {
    let conn = state.db.conn();
    let site = get_site(&conn, site_id).await.unwrap();

    NewEventPage { site }
}
