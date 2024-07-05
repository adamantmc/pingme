pub mod schema;
pub mod utils;

use std::fs::create_dir_all;
use std::path::PathBuf;
use chrono::NaiveDateTime;
use diesel::sqlite::SqliteConnection;
use diesel::connection::Connection;
use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
use diesel::RunQueryDsl;
use crate::db;
use crate::db::schema::messages::{added_at, notify_at};
use crate::models::Message;


pub fn connect(db_path: &str) -> Result<SqliteConnection, String> {
    let res = create_dir_all(PathBuf::from(db_path).parent().unwrap());

    if res.is_err() {
        return Err(format!("Failed to create dirs for {}", db_path));
    }

    let conn = SqliteConnection::establish(db_path);

    if conn.is_err() {
        return Err(format!("Failed to establish connection to SQLite DB at {}", db_path))
    }
    return Ok(conn.unwrap());
}


pub fn insert(conn: &mut SqliteConnection, message: Message) -> Result<(), String>{
    let res = diesel::insert_into(db::schema::messages::table)
        .values(&message)
        .execute(conn);

    if res.is_err() {
        return Err(format!("Failed to write to DB - {}", res.err().unwrap().to_string()));
    }

    return Ok(());
}


pub fn list(
    conn: &mut SqliteConnection,
    added_at_from: Option<NaiveDateTime>,
    notify_at_from: Option<NaiveDateTime>,
    notify_at_to: Option<NaiveDateTime>,
    limit: Option<i64>, offset: Option<i64>
) -> Result<Vec<Message>, String> {
    use db::schema::messages;

    let mut q  = messages::table.into_boxed().order_by(added_at.desc());

    if added_at_from.is_some() {
        q = q.filter(added_at.ge(added_at_from.unwrap()))
    }

    if notify_at_from.is_some() {
        q = q.filter(notify_at.ge(notify_at_from.unwrap()))
    }

    if notify_at_to.is_some() {
        q = q.filter(notify_at.le(notify_at_to.unwrap()))
    }

    if offset.is_some() {
        q = q.offset(offset.unwrap());
    }

    if limit.is_some() {
        q = q.limit(limit.unwrap());
    }

    let res = q.select(Message::as_select()).load(conn);

    if res.is_err() {
        return Err(format!("Failed to list messages - {}", res.err().unwrap().to_string()));
    }

    return Ok(res.unwrap());
}


pub fn delete(conn: &mut SqliteConnection, row_id: i32) -> Result<usize, String> {
    use db::schema::messages::dsl::*;

    let res = diesel::delete(messages.filter(id.eq(&row_id))).execute(conn);

    if res.is_err() {
        return Err(
            format!(
                "Failed to delete row with id {row_id} - {err}",
                row_id=row_id, err=res.err().unwrap().to_string()
            )
        );
    }

    return Ok(res.unwrap());
}
