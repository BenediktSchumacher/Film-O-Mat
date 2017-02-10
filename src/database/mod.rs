use rusqlite::Connection;
use std::env;
use std::fs::create_dir_all;

#[derive(Debug)]
// A struct to represent a movie to be attached to actors or directors
pub struct Movie {
    pub title: String,
    pub year: String,
}

// Checks if a database already exists
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

// Creates a database with movies and related information for movie suggestions
// Movies: movie titles with associated release date and genre
// Actors: names of all actors
// Rankings: associates movie ID with IMDb ratings and number of ratings
// Crew (actors and directors): associates actors/directors with their movies
pub fn create_database() {

    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");

    // creates path if not existing
    create_dir_all(&path_buf);

    path_buf.push("database");
    path_buf.set_extension("db");

    let conn = Connection::open(path_buf).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS movies (
                  id              INTEGER PRIMARY KEY,
                  title            TEXT NOT NULL,
                  year            TEXT NOT NULL,
                  genre           TEXT
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS actors (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS directors (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS rankings (
                  id              INTEGER PRIMARY KEY,
                  movie_id        INTEGER,
                  score           TEXT NOT NULL,
                  number          TEXT NOT NULL,
                  FOREIGN KEY(movie_id) REFERENCES movies(id)
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS crew_a (
                  actor_id        INTEGER,
                  movie_id        INTEGER,
                  FOREIGN KEY(movie_id) REFERENCES movies(id),
                  FOREIGN KEY(actor_id) REFERENCES actors(id)
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS crew_d (
                  director_id        INTEGER,
                  movie_id        INTEGER,
                  FOREIGN KEY(movie_id) REFERENCES movies(id),
                  FOREIGN KEY(director_id) REFERENCES directors(id)
                  )",
                 &[])
        .unwrap();
}

// List of movies with their year of release
pub fn import_movie(title: &str, year: &str) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO movies (title, year) VALUES ('{}', '{}')",
                          title,
                          year),
                 &[])
        .unwrap();
}

// Genre information is added to the movie list
pub fn add_genres(title: &str, year: &str, genre: &str) {
    let conn = get_connection();

    conn.execute(&format!("UPDATE movies SET genre = '{}' WHERE title = '{}' AND year = '{}'",
                          genre,
                          title,
                          year),
                 &[])
        .unwrap();
}

// List of ratings with information of user ratings
pub fn add_rating(title: &str, year: &str, rank: &str, votes: &str) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO rankings (movie_id, score, number) VALUES ((SELECT id \
                           FROM movies WHERE title = '{}' AND year = '{}'), '{}', '{}')",
                          title,
                          year,
                          rank,
                          votes),
                 &[])
        .unwrap();
}

// List of all actors, independent of their movies
// Connection between actors and their movies outsourced to separate list
pub fn add_actor(name: &str, movies: Vec<Movie>) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO actors (name) VALUES ('{}')", &name),
                 &[])
        .unwrap();

    for m in movies {
        conn.execute(&format!("INSERT INTO crew_a (actor_id, movie_id) VALUES ((SELECT id FROM \
                               actors WHERE name = '{}'), (SELECT id FROM movies WHERE title = \
                               '{}' AND year = '{}'))",
                              name,
                              m.title,
                              m.year),
                     &[])
            .unwrap();
    }
}

// List of all directors, independent of their movies
// Connection between directors and their movies outsourced to separate list
pub fn add_director(name: &str, movies: Vec<Movie>) {
    let conn = get_connection();

    conn.execute(&format!("INSERT INTO directors (name) VALUES ('{}')", &name),
                 &[])
        .unwrap();

    for m in movies {
        conn.execute(&format!("INSERT INTO crew_d (director_id, movie_id) VALUES ((SELECT id \
                               FROM directors WHERE name = '{}'), (SELECT id FROM movies WHERE \
                               title = '{}' AND year = '{}'))",
                              name,
                              m.title,
                              m.year),
                     &[])
            .unwrap();
    }
}
