use rusqlite::Connection;
use std::env;
use std::fs::create_dir_all;

pub fn db_exists() -> bool {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");
    path_buf.push("database");
    path_buf.set_extension("db");
    path_buf.as_path().exists()
}

pub fn create_database() {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");

    // creates path if not existing
    create_dir_all(&path_buf);

    path_buf.push("database");
    path_buf.set_extension("db");

    let conn = Connection::open(path_buf).unwrap();

    // Example of SQL-Statement
    conn.execute("CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  time_created    TEXT NOT NULL,
                  data            BLOB
                  )",
                 &[])
        .unwrap();
}
