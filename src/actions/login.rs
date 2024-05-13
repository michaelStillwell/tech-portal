use crate::{
    auth::{set_session, SessionId},
    db::{
        session::{create_session, Session},
        user::get_user_with_login,
    },
    AppState,
};
use askama::Template;
use rocket::{form::Form, post, FromForm, Request, Response, State};

pub enum LoginActionResponder {
    Success(Session, Option<String>),
    Error(ErrorText),
}

impl<'r> rocket::response::Responder<'r, 'static> for LoginActionResponder {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        println!("req {}", req.uri());
        match self {
            LoginActionResponder::Success(session, redirect_url) => {
                set_session(req.cookies(), SessionId(session.session_id));
                Response::build()
                    .raw_header(
                        "HX-Redirect",
                        if let Some(redirect_url) = redirect_url {
                            if redirect_url.starts_with('/') {
                                redirect_url
                            } else {
                                format!("/{redirect_url}")
                            }
                        } else {
                            "/".to_string()
                        },
                    )
                    .ok()
            }
            LoginActionResponder::Error(text) => text.respond_to(req),
        }
    }
}

#[derive(Template)]
#[template(source = "<p class=\"error\">{{ text }}</p>", ext = "html")]
pub struct ErrorText {
    text: String,
}

#[derive(FromForm)]
pub struct LoginActionForm {
    username: String,
    password: String,
}

#[post("/login?<redirect_url>", data = "<form>")]
pub async fn login_action(
    redirect_url: Option<String>,
    form: Form<LoginActionForm>,
    state: &State<AppState>,
) -> LoginActionResponder {
    let conn = state.db.conn();
    if let Ok(user) = get_user_with_login(&conn, &form.username, &form.password).await {
        let session = create_session(&conn, &user).await;
        LoginActionResponder::Success(session, redirect_url)
    } else {
        LoginActionResponder::Error(ErrorText {
            text: "Invalid login".to_string(),
        })
    }
}
