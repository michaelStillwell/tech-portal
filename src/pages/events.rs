use crate::{
    auth::SessionId,
    db::{
        event::{get_event, get_events_by_user, Event},
        session::Session,
        user::get_user_with_session,
    },
    AppState,
};
use askama::Template;
use rocket::{get, State};

#[derive(Template)]
#[template(path = "pages/events.html")]
pub struct EventsPage {
    events: Vec<Event>,
}

#[get("/events")]
pub async fn events_page(state: &State<AppState>, session_id: SessionId) -> EventsPage {
    let conn = state.db.conn();
    let user = get_user_with_session(&conn, Session::with_id(&session_id))
        .await
        .expect("user not found");
    let events = get_events_by_user(&conn, user.user_id).await;
    EventsPage { events }
}

#[derive(Template)]
#[template(path = "pages/edit_event.html")]
pub struct EditEventPage {
    event: Event,
}

#[get("/event/<event_id>")]
pub async fn edit_event_page(
    state: &State<AppState>,
    session_id: SessionId,
    event_id: i64,
) -> EditEventPage {
    let conn = state.db.conn();
    let user = get_user_with_session(&conn, Session::with_id(&session_id))
        .await
        .expect("user not found");
    let event = get_event(&conn, event_id).await.unwrap();
    EditEventPage { event }
}
