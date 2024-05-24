use crate::{
    auth::AuthedUser,
    db::{
        event::{add_event, Event, InsertEvent},
        site::get_site,
    },
    htmx::HxHeader,
    AppState,
};
use askama::Template;
use rocket::{form::Form, http::Status, post, FromForm, Responder, State};

#[derive(Template)]
#[template(path = "components/event.html")]
pub struct EventComponent {
    pub event: Event,
}

#[derive(FromForm)]
pub struct AddEventForm {
    name: String,
    description: String,
}

#[derive(Responder)]
pub enum AddEventResponse {
    Success(Status, HxHeader),
    Failure(String),
}

#[post("/site/<site_id>/events", data = "<new_event>")]
pub async fn add_event_action(
    state: &State<AppState>,
    user: AuthedUser,
    site_id: i64,
    new_event: Form<AddEventForm>,
) -> AddEventResponse {
    let conn = state.db.conn();
    if let Some(site) = get_site(&conn, site_id).await {
        match add_event(
            &conn,
            InsertEvent::new(
                &new_event.name,
                &new_event.description,
                user.user_id,
                site.site_id,
            ),
        )
        .await
        {
            Ok(_) => AddEventResponse::Success(
                Status::Created,
                HxHeader::Redirect(format!("/site/{site_id}")),
            ),
            Err(_) => {
                AddEventResponse::Failure("An event already exists with that name".to_string())
            }
        }
    } else {
        AddEventResponse::Failure("Site does not exist".to_string())
    }
}
