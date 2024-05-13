use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use rocket::{
    http::{Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{FromRequest, Outcome},
    response::Redirect,
    time::Duration,
    Request,
};
use std::{convert::Infallible, ops::Deref};
use tracing::{error, info};

const SESSION_NAME: &str = "session";

#[derive(Debug)]
pub struct SessionId(pub uuid::Uuid);
pub struct NoSession;

impl Deref for SessionId {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn get_session(jar: &CookieJar) -> Option<SessionId> {
    jar.get_private(SESSION_NAME)
        .and_then(|c| c.value().parse().ok())
        .map(|id| SessionId(id))
}

pub fn set_session(jar: &CookieJar, session: SessionId) {
    jar.add_private(
        Cookie::build((SESSION_NAME, session.0.to_string()))
            // TODO: place the domain?
            // .domain("www.rust-lang.org")
            .path("/")
            .secure(true)
            .http_only(true)
            .max_age(Duration::days(1))
            .build(),
    );
}

pub struct Password {
    pub hash: String,
}

pub fn create_hash(password: String) -> Password {
    let password_salt = SaltString::generate(OsRng);
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes(), &password_salt)
        .unwrap();

    Password {
        hash: password_hash.to_string(),
    }
}

pub fn validate_password(password: &str, prev: Password) -> bool {
    info!("hash: {}", prev.hash);
    match PasswordHash::new(&prev.hash) {
        Ok(parsed_hash) => Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok(),
        Err(err) => {
            error!("error parsing hash: {err}");
            false
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionId {
    type Error = Infallible;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        get_session(req.cookies()).or_forward(Status::Unauthorized)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for NoSession {
    type Error = Redirect;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match get_session(req.cookies()) {
            Some(_) => Outcome::Error((Status::Ok, Redirect::to("/404"))),
            None => Outcome::Success(NoSession),
        }
    }
}
