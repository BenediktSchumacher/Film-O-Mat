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
    conn.execute("CREATE TABLE IF NOT EXISTS movies (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  year            TEXT NOT NULL,
                  genre           TEXT NOT NULL,
                  director        TEXT NOT NULL
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS actors (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS rankings (
                  id              INTEGER PRIMARY KEY,
                  movie_id        INTEGER,
                  score           TEXT NOT NULL,
                  number          INTEGER NOT NULL,
                  FOREIGN KEY(movie_id) REFERENCES movies(id)
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS crew (
                  actor_id        INTEGER,
                  movie_id        INTEGER,
                  FOREIGN KEY(movie_id) REFERENCES movies(id),
                  FOREIGN KEY(actor_id) REFERENCES actors(id)
                  )",
                 &[])
        .unwrap();
}
