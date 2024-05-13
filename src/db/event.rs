use libsql::{de, Connection, Row};

#[derive(serde::Deserialize)]
pub struct Event {
    pub event_id: i64,
    pub name: String,
    pub description: String,
    pub user_id: i64,
    pub site_id: i64,
}

impl Default for Event {
    fn default() -> Self {
        Event {
            event_id: -1,
            name: String::new(),
            description: String::new(),
            user_id: -1,
            site_id: -1,
        }
    }
}

impl From<&Row> for Event {
    fn from(row: &Row) -> Self {
        de::from_row(row).expect("could not unwrap row for event")
    }
}

pub struct InsertEvent {
    pub name: String,
    pub description: String,
    pub user_id: i64,
    pub site_id: i64,
}

impl InsertEvent {
    pub fn new(name: &str, description: &str, user_id: i64, site_id: i64) -> Self {
        InsertEvent {
            name: name.to_string(),
            description: description.to_string(),
            user_id,
            site_id,
        }
    }
}

pub async fn get_event(conn: &Connection, event_id: i64) -> Option<Event> {
    let mut rows = conn
        .query(
            "SELECT event_id, name, description, user_id, site_id FROM events WHERE event_id=?1;",
            [event_id.to_string()],
        )
        .await
        .unwrap();

    if let Some(row) = rows.next().await.unwrap() {
        Some(Event::from(&row))
    } else {
        None
    }
}

pub async fn get_events_by_user(conn: &Connection, user_id: i64) -> Vec<Event> {
    let mut rows = conn
        .query(
            "SELECT event_id, name, description, user_id, site_id FROM events WHERE user_id=?1;",
            [user_id.to_string()],
        )
        .await
        .unwrap();

    let mut events = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        let event = Event::from(&row);
        events.push(event);
    }

    events
}
