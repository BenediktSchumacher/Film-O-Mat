use rusqlite::Connection;
use std::env;

pub fn db_exists() -> bool {
    false
}

pub fn create_database() {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");
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
