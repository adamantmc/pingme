use std::ops::{Add, Sub};
use std::{thread, time};

use chrono::{Local, NaiveDateTime, TimeDelta, TimeZone, Utc};
use clap::Parser;
use diesel::connection::SimpleConnection;
use diesel::SqliteConnection;
use diesel_migrations::{embed_migrations, MigrationHarness};
use diesel_migrations::EmbeddedMigrations;
use prettytable::{row, Table};

use crate::db::utils::get_db_path;
use crate::models::Message;

mod cli;
mod db;
mod models;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!(".\\src\\db\\migrations");

fn str_to_timedelta(s: &str) -> Result<TimeDelta, &str> {
    if s.len() < 2 {
        return Err("Timedelta argument must be at least 2 characters long");
    }

    let granularity = s.chars().last().unwrap();
    let value: &str = &s[0..s.len() - 1];
    let value_num = value.to_string().parse::<i64>();

    if value_num.is_err() {
        return Err("Failed to parse number from input string");
    }

    if granularity == 'd' {
        return Ok(TimeDelta::days(value_num.unwrap()));
    } else if granularity == 'h' {
        return Ok(TimeDelta::hours(value_num.unwrap()));
    } else if granularity == 'm' {
        return Ok(TimeDelta::minutes(value_num.unwrap()));
    } else if granularity == 's' {
        return Ok(TimeDelta::seconds(value_num.unwrap()));
    }

    return Err("Unknown time granularity");
}


fn handle_add(conn: &mut SqliteConnection, text: String, notify_after: Option<String>) -> Result<(), String> {
    let mut msg = Message::new(text, None);

    if notify_after.is_some() {
        let val = notify_after.unwrap();

        let td = str_to_timedelta(val.as_str());

        if td.is_err() {
            return Err(format!("Failed to parse 'notify_after' value {}", val));
        }

        msg.notify_at = msg.added_at.add(td.unwrap()).into();
    }

    return db::insert(conn, msg);
}


fn handle_delete(conn: &mut SqliteConnection, id: i32) -> Result<usize, String> {
    db::delete(conn, id)
}


fn handle_list(
    conn: &mut SqliteConnection, last: Option<String>, limit: Option<i64>, offset: Option<i64>,
) -> Result<Vec<Message>, String> {
    let mut from: Option<NaiveDateTime> = None;

    if last.is_some() {
        let val = last.unwrap();
        let td = str_to_timedelta(val.as_str());

        if td.is_err() {
            return Err(format!("Failed to parse 'last' value {}", val));
        }
        from = Option::from(Utc::now().naive_utc().sub(td.unwrap()));
    }

    let res = db::list(conn, from, None, None, limit, offset);

    return res;
}


fn format_messages(messages: Vec<Message>) -> Table {
    let mut table = Table::new();

    table.add_row(row!["ID", "Text", "Added At", "Notify At"]);

    for msg in messages.iter() {
        table.add_row(row![
                msg.id.unwrap(),
                msg.text,
                Local.from_utc_datetime(&msg.added_at).to_string(),
                if msg.notify_at.is_some() {
                    Local.from_utc_datetime(&msg.notify_at.unwrap()).to_string()
                }
                else { "".into() }
            ]
        );
    }

    return table;
}


fn run_daemon(conn: &mut SqliteConnection) {
    use notify_rust::Notification;
    let sleep_duration = 5; // seconds

    loop {
        let notify_at_from = Option::from(Utc::now().naive_utc());
        let notify_at_to = Option::from(
            Utc::now().naive_utc().add(TimeDelta::seconds(sleep_duration))
        );

        let res = db::list(
            conn, None, notify_at_from, notify_at_to, None, None
        ).unwrap();

        if res.len() == 0 {
            continue
        }

        for msg in res.iter() {
            let r = Notification::new()
                .appname("PingMe")
                .summary("PingMe - Reminder")
                .body(msg.text.as_str()).show();

            if r.is_err() {
                eprintln!("Failed to send notification");
                break;
            } else {
                println!("Sent notification for message {}", msg.id.unwrap());
            }
        }

        thread::sleep(time::Duration::from_secs(sleep_duration as u64));
    }
}


fn main() {
    let conn_res = db::connect(get_db_path().to_str().unwrap());
    if conn_res.is_err() {
        panic!("{}", conn_res.err().unwrap());
    }
    let mut conn = conn_res.unwrap();
    conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")
        .unwrap_or_else(|_| panic!("Failed to set connection params"));

    let args = cli::CLIParser::parse();

    match args.command {
        cli::Commands::Add(args) => {
            conn.run_pending_migrations(MIGRATIONS).expect("Failed to run DB migrations");

            let res = handle_add(&mut conn, args.text, args.notify_after);

            if res.is_err() {
                eprintln!("Error: {}", res.err().unwrap());
                return;
            }
        }

        cli::Commands::Delete(args) => {
            let res = handle_delete(&mut conn, args.id);

            if res.is_err() {
                eprintln!("Error: {}", res.err().unwrap());
            }
            else if res.unwrap() == 0 {
                eprintln!("No matching record found for id={}", args.id);
            }
        }

        cli::Commands::List(args) => {
            conn.run_pending_migrations(MIGRATIONS).expect("Failed to run DB migrations");

            let res = handle_list(
                &mut conn, args.last, args.limit, args.offset,
            );

            if res.is_err() {
                eprintln!("Error: {}", res.err().unwrap());
                return;
            }

            let table = format_messages(res.unwrap());

            table.printstd();
        }

        cli::Commands::Daemon => run_daemon(&mut conn),
    }
}
