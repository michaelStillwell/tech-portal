use crate::{
    auth::SessionId,
    db::{
        session::Session,
        site::{get_sites, Site},
        user::get_user_with_session,
    },
    AppState,
};
use askama::Template;
use rocket::{get, State};

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePage {
    sites: Vec<Site>,
}

#[get("/")]
pub async fn home_page(state: &State<AppState>, session: SessionId) -> HomePage {
    let conn = state.db.conn();
    let user = get_user_with_session(&conn, Session::with_id(&session)).await;
    let sites = if let Ok(user) = user {
        get_sites(&conn, user.user_id).await
    } else {
        Vec::new()
    };

    HomePage { sites }
}
