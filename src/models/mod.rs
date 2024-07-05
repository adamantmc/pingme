use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::db::schema::messages)]
pub struct Message {
    pub id: Option<i32>,
    pub text: String,
    pub added_at: NaiveDateTime,
    pub notify_at: Option<NaiveDateTime>
}


impl Message {
    pub fn new(text: String, notify_at: Option<NaiveDateTime>) -> Self {
        Message { id: None, text, added_at: Utc::now().naive_utc(), notify_at }
    }

    pub fn print(&self) {
        println!(
            "[{added_at}] {text} {notify_at}",
            added_at=self.added_at.to_string(),
            text=self.text,
            notify_at= if self.notify_at.is_some() { self.notify_at.unwrap().to_string() } else { "".to_string() }
        );
    }
}
