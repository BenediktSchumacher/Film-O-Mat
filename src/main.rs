pub mod parser;
pub mod download;
pub mod database;

extern crate curl;
extern crate flate2;
extern crate rusqlite;
extern crate regex;

use download::*;
use download::decompressor::*;
use database::*;
use parser::*;

fn main() {

    // TODO: Pruefem, ob Datenbank vorhanden ist
    // Falls nein => Dateipfad erstellen, Datenbank anlegen und anschliessend fuellen
    // Falls ja => Datenbankverbindung herstellen, Anfrage bearbeiten
    if !db_exists() {
        create_database();

        println!("Download started");
        let movies = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                 de/pub/misc/movies/database/ratings.list.gz"));

        let genres = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                  de/pub/misc/movies/database/genres.list.gz"));
        println!("Download finished...");
        parse_rating(movies.unwrap());
        parse_genre(genres.unwrap());

    }

    let genres = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                  de/pub/misc/movies/database/genres.list.gz"));
    // println!("Download finished...");
    // parse_rating(movies.unwrap());
    parse_genre(genres.unwrap());

    // let movies = decompress(&download_archiv("ftp://ftp.fu-berlin.\
    // de/pub/misc/movies/database/movies.list.gz"));
    // println!("{:?}", movies.unwrap());
    //
    // parse_movies(movies.unwrap());

    // import_movie("8 myyttiä työstä", "2005");
    // add_genres("Batman", "2005", "Thriller");
    // add_rating("Batman", "2005", "8.324", "500");
    //
    // let movie_1 = Movie {
    // title: "Batman".to_string(),
    // year: "2005".into(),
    // };
    //
    // let name = "Heath Ledgers";
    //
    // add_actor(name, vec![movie_1]);
}
