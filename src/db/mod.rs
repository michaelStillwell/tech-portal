use crate::db::{event::InsertEvent, site::InsertSite, user::InsertUser};
use libsql::{Connection, Database};
use std::{ops::Deref, sync::Arc};
use tracing::info;

pub mod event;
pub mod session;
pub mod site;
pub mod user;

pub struct Db(Arc<Database>);

impl Db {
    pub fn conn(&self) -> Connection {
        // TODO: log better here so no losty
        self.connect().expect("could not connect to db")
    }
}

// TODO: still want this?
impl Deref for Db {
    type Target = Arc<Database>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub async fn init_db(database: Database) -> Db {
    info!("initializing the database");

    let conn = database.connect().expect("couldn't connect to database");
    init_schema(&conn).await;

    if cfg!(debug_assertions) {
        init_dummy_data(&conn).await;
    }

    Db(Arc::new(database))
}

async fn init_schema(conn: &Connection) {
    let _ = conn
        .execute(
            r#"CREATE TABLE IF NOT EXISTS users (
                user_id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                password_hash TEXT NOT NULL,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);"#,
            (),
        )
        .await;

    let _ = conn
        .execute(
            r#"CREATE TABLE IF NOT EXISTS sessions (
                session_id TEXT PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);"#,
            (),
        )
        .await;

    let _ = conn
        .execute(
            r#"CREATE TABLE IF NOT EXISTS sites (
                site_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);"#,
            (),
        )
        .await;

    let _ = conn
        .execute(
            r#"CREATE TABLE IF NOT EXISTS events (
                event_id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT UNIQUE NOT NULL,
                description TEXT NOT NULL,
                user_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                site_id INTEGER NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP);"#,
            (),
        )
        .await;
}

async fn init_dummy_data(conn: &Connection) {
    info!("seeding the db");
    let mut queries: Vec<String> = vec![];

    let dummy_users = vec![
        InsertUser::new("user@user1.com", "pass"),
        InsertUser::new("user@user2.com", "pass"),
        InsertUser::new("user@user3.com", "pass"),
    ];

    dummy_users.iter().for_each(|user| {
        queries.push(format!(
            "INSERT OR IGNORE INTO users (email, password_hash) VALUES ('{}', '{}');",
            user.email, user.password_hash,
        ))
    });

    let dummy_sites = vec![
        InsertSite::new("site1", 1),
        InsertSite::new("site2", 2),
        InsertSite::new("site3", 3),
    ];

    dummy_sites.iter().for_each(|site| {
        queries.push(format!(
            "INSERT OR IGNORE INTO sites (name, user_id) VALUES ('{}', {});",
            site.name, site.user_id
        ))
    });

    let dummy_events = vec![
        InsertEvent::new("First Event", "This is my first event!", 1, 1),
        InsertEvent::new("Second Event", "This is my second event!", 2, 2),
        InsertEvent::new("Thrid Event", "This is my third event!", 3, 3),
    ];

    dummy_events.iter().for_each(|event| {
        queries.push(format!(
            "INSERT OR IGNORE INTO events (name, description, user_id, site_id) 
                VALUES ('{}', '{}', {}, {});",
            event.name, event.description, event.user_id, event.site_id
        ))
    });

    let _ = conn.execute_batch(&queries.join("\n")).await;
}
