use askama::Template;
use rocket::{http::Status, post, State};

use crate::{
    db::event::{_get_event, Event},
    AppState,
};

#[derive(Template)]
#[template(path = "components/edit_event.html")]
pub struct EditEventComponent {
    event: Event,
}

#[post("/events/<event_id>")]
pub async fn _edit_event_action(
    state: &State<AppState>,
    event_id: i64,
) -> (Status, EditEventComponent) {
    let conn = state.db.conn();
    let event = _get_event(&conn, event_id).await;

    if let Some(event) = event {
        (Status::Ok, EditEventComponent { event })
    } else {
        (
            Status::NotFound,
            EditEventComponent {
                event: Event::default(),
            },
        )
    }
}
