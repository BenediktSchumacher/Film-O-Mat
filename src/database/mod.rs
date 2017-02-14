use rusqlite::Connection;
use std::env;
use std::fs::create_dir_all;
use ::input::*;
use std::collections::HashSet;
use output::SearchResult;

/// A struct to represent a movie to be attached to actors or directors
#[derive(Debug)]
pub struct Movie {
    pub title: String,
    pub year: String,
}

#[derive(Debug)]
struct Res {
    pub field: i64,
}

#[derive(Debug)]
struct GenreResult {
    pub field: String,
}

/// Checks if a database already exists
pub fn db_exists() -> bool {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");
    path_buf.push("database");
    path_buf.set_extension("db");
    path_buf.as_path().exists()
}

/// Returns a connection to existing database
fn get_connection() -> Connection {
    let tmp = env::home_dir();
    let mut path_buf = tmp.unwrap();
    path_buf.push("Film-O-Mat");
    path_buf.push("database");
    path_buf.set_extension("db");

    let conn = Connection::open(path_buf).unwrap();
    conn
}

/// Creates a database with movies and related information for movie suggestions
/// Movies: movie titles with associated release date and genre
/// Actors: names of all actors
/// Rankings: associates movie ID with IMDb ratings and number of ratings
/// Crew (actors and directors): associates actors/directors with their movies
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
                  title           TEXT NOT NULL,
                  year            TEXT NOT NULL,
                  rating          TEXT NOT NULL,
                  number          TEXT NOT NULL
                  )",
                 &[])
        .unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS genres (
                  movie_id        INTEGER NOT NULL,
                  genre           TEXT NOT NULL
            )",
                 &[])
        .unwrap();

    // conn.execute("CREATE TABLE IF NOT EXISTS actors (
    // id              INTEGER PRIMARY KEY,
    // name            TEXT NOT NULL
    // )",
    // &[])
    // .unwrap();
    //
    // conn.execute("CREATE TABLE IF NOT EXISTS directors (
    // id              INTEGER PRIMARY KEY,
    // name            TEXT NOT NULL
    // )",
    // &[])
    // .unwrap();
    //
    // conn.execute("CREATE TABLE IF NOT EXISTS crew_a (
    // actor_id        INTEGER,
    // movie_id        INTEGER,
    // FOREIGN KEY(movie_id) REFERENCES movies(id),
    // FOREIGN KEY(actor_id) REFERENCES actors(id)
    // )",
    // &[])
    // .unwrap();
    //
    // conn.execute("CREATE TABLE IF NOT EXISTS crew_d (
    // director_id        INTEGER,
    // movie_id        INTEGER,
    // FOREIGN KEY(movie_id) REFERENCES movies(id),
    // FOREIGN KEY(director_id) REFERENCES directors(id)
    // )",
    // &[])
    // .unwrap();
}

/// List of movies with their year of release
pub fn import_movie(title: &str, year: &str, rating: &str, number: &str) {
    let conn = get_connection();

    println!("{}", title.replace("'", "").as_str());
    println!("{:?}", year);
    println!("{:?}", rating);
    println!("{:?}", number);

    conn.execute(&format!("INSERT INTO movies (title, year, rating, number) VALUES ('{}', '{}', \
                           '{}', '{}')",
                          title.replace("'", "''").as_str(),
                          year,
                          rating,
                          number),
                 &[])
        .unwrap();
}

/// Genre information is added to the movie list
pub fn add_genres(title: &str, year: &str, genre: &str) {
    let conn = get_connection();

    let mut stm = conn.prepare(&format!("SELECT id FROM movies WHERE title = '{}' AND year = '{}'",
                          title.replace("'", "''").as_str(),
                          year))
        .unwrap();

    let res = stm.query_map(&[], |x| Res { field: x.get(0) })
        .unwrap();

    let mut existent = false;
    let mut movie_id = 0;

    for tmp in res {
        existent = true;
        movie_id = tmp.unwrap().field;
    }

    if existent {
        conn.execute(&format!("INSERT INTO genres (movie_id, genre) VALUES ('{}', '{}')",
                              movie_id,
                              genre),
                     &[])
            .unwrap();
    }
}

/// List of ratings with information of user ratings
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

pub fn execute(search_params: SearchParams) -> Vec<SearchResult> {
    // println!("{:?}", search_params);
    let conn = get_connection();
    let mut query = String::new();

    query.push_str("SELECT * FROM movies JOIN (SELECT movie_id FROM genres JOIN (SELECT movie_id \
                    as m_id, COUNT(movie_id) AS ctr FROM (SELECT * FROM genres WHERE ");

    let mut genres_string = String::new();
    let mut genres: HashSet<String> = HashSet::new();

    if !search_params.get_movies().is_empty() {
        let mut movie_query = String::new();
        movie_query.push_str("SELECT genre FROM movies JOIN genres ON id = movie_id WHERE");

        let mut tmp = String::new();

        for movie in search_params.get_movies() {
            if tmp.is_empty() {
                tmp.push_str(format!(" title = '{}'", movie.replace("'", "''")).as_str());
            } else {
                tmp.push_str(format!(" OR title = '{}'", movie.replace("'", "''")).as_str());
            }
        }

        movie_query.push_str(tmp.as_str());

        let mut stm = conn.prepare(movie_query.as_str()).unwrap();

        let res = stm.query_map(&[], |x| GenreResult { field: x.get(0) }).unwrap();

        for genre in res {
            genres.insert(genre.unwrap().field);
        }
    }

    for genre in search_params.get_genres() {
        genres.insert(genre);
    }

    for genre in &genres {
        if genres_string.is_empty() {
            genres_string.push_str(format!("genre = '{}'", genre).as_str());
        } else {
            genres_string.push_str(format!(" OR genre = '{}'", genre).as_str());
        }
    }

    query.push_str(genres_string.as_str());

    query.push_str(format!(") GROUP BY m_id) ON movie_id = m_id WHERE ctr >= {} GROUP BY \
                            movie_id) ON id = movie_id WHERE rating > {} ORDER BY rating DESC",
                           genres.len(),
                           search_params.get_rating())
        .as_str());

    let mut stm = conn.prepare(query.as_str()).unwrap();
    let res = stm.query_map(&[], |x| {
        SearchResult {
            title: x.get(1),
            score: x.get(3),
            number: x.get(4),
            year: x.get(2),
            genres: Vec::new(),
        }
    });

    let mut tmp = Vec::new();
    for movie in res.unwrap() {
        let mut mov = movie.unwrap().clone();

        let mut stm =
            conn.prepare(format!("SELECT genre FROM genres WHERE movie_id = (SELECT id FROM \
                                  movies WHERE title = '{}' AND year = '{}') GROUP BY genre \
                                  ORDER BY genre ASC",
                                 mov.title,
                                 mov.year)
                    .as_str())
                .unwrap();
        let genres = stm.query_map(&[], |x| GenreResult { field: x.get(0) }).unwrap();
        let mut genre = Vec::new();
        for g in genres {
            genre.push(g.unwrap().field);
        }
        mov.genres = genre;
        tmp.push(mov);
    }

    tmp

}
