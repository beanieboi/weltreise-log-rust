use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Entry {
    pub id: i64,
    pub longitude: f64,
    pub latitude: f64,
    pub description: String,
    pub image_url: String,
    pub created_at: NaiveDateTime,
}

use super::schema::entries;

#[derive(Insertable)]
#[table_name = "entries"]
pub struct NewEntry {
    pub id: i64,
    pub longitude: f64,
    pub latitude: f64,
    pub description: String,
    pub image_url: String,
    pub created_at: NaiveDateTime,
}
