use rusqlite::Connection;
use std::env;
use std::fs::create_dir_all;

#[derive(Debug)]
pub struct Movie {
    pub title: String,
    pub year: String,
}

pub fn db_exists() -> bool {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");
    path_buf.push("database");
    path_buf.set_extension("db");
    path_buf.as_path().exists()
}

fn get_connection() -> Connection {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");
    path_buf.push("database");
    path_buf.set_extension("db");

    let conn = Connection::open(path_buf).unwrap();
    conn
}

pub fn create_database() {

    // TODO: Verzeichnissbaum erstellen
    let conn = get_connection();

    conn.execute("CREATE TABLE IF NOT EXISTS movies (
                  id              INTEGER PRIMARY KEY,
                  title            TEXT NOT NULL,
                  year            TEXT NOT NULL,
                  genre           TEXT,
                  director        TEXT
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

pub fn import_movie(title: &str, year: &str) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO movies (title, year) VALUES ('{}', '{}')",
                          title,
                          year),
                 &[])
        .unwrap();
}

pub fn add_genres(title: &str, year: &str, genre: &str) {
    let conn = get_connection();

    conn.execute(&format!("UPDATE movies SET genre = '{}' WHERE title = '{}' AND year = '{}'",
                          genre,
                          title,
                          year),
                 &[])
        .unwrap();
}

pub fn add_rating(title: &str, year: &str, rank: &str, votes: u64) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO rankings (movie_id, score, number) VALUES ((SELECT id \
                           FROM movies WHERE title = '{}' AND year = '{}'), '{}', {})",
                          title,
                          year,
                          rank,
                          votes),
                 &[])
        .unwrap();
}

pub fn add_actor(name: &str, movies: Vec<Movie>) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO actors (name) VALUES ('{}')", &name),
                 &[])
        .unwrap();

    for m in movies {
        conn.execute(&format!("INSERT INTO crew (actor_id, movie_id) VALUES ((SELECT id FROM \
                               actors WHERE name = '{}'), (SELECT id FROM movies WHERE title = \
                               '{}' AND year = '{}'))",
                              name,
                              m.title,
                              m.year),
                     &[])
            .unwrap();
    }
}
