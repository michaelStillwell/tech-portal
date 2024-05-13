use libsql::{de, Connection, Row};

#[derive(serde::Deserialize)]
pub struct Site {
    pub site_id: i64,
    pub name: String,
    pub created_at: String, // TODO: change to Date
}

impl From<&Row> for Site {
    fn from(row: &Row) -> Self {
       de::from_row(row).expect("could not unwrap row for site")
    }
}

pub struct InsertSite {
    pub name: String,
    pub user_id: i64,
}

impl InsertSite {
    pub fn new(name: &str, user_id: i64) -> Self {
        InsertSite {
            name: name.to_string(),
            user_id,
        }
    }
}

pub async fn get_sites(conn: &Connection, user_id: i64) -> Vec<Site>{
    let mut rows = conn
       .query(
            "SELECT site_id, name, created_at FROM sites WHERE user_id=?1;",
            [user_id.to_string()],
        )
       .await
       .unwrap();

    let mut sites = Vec::new();
    while let Some(row) = rows.next().await.unwrap() {
        let site = Site::from(&row);
        sites.push(site);
    }

    sites
}
