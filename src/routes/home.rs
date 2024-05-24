use crate::{
    auth::AuthedUser,
    db::site::{get_sites, Site},
    AppState,
};
use askama::Template;
use rocket::{get, response::Redirect, Responder, State};

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomePage {
    sites: Vec<Site>,
}

#[derive(Responder)]
pub enum HomeResponse {
    GotoSite(Redirect),
    Continue(HomePage),
}

#[get("/")]
pub async fn home_page(state: &State<AppState>, user: AuthedUser) -> HomeResponse {
    let conn = state.db.conn();
    let sites = get_sites(&conn, user.0.user_id).await;

    if sites.len() == 1 {
        HomeResponse::GotoSite(Redirect::to(format!("/site/{}", sites[0].site_id)))
    } else {
        HomeResponse::Continue(HomePage { sites })
    }
}
