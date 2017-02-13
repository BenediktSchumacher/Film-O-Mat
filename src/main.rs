pub mod parser;
pub mod download;
pub mod database;
pub mod input;

extern crate curl;
extern crate flate2;
extern crate rusqlite;
extern crate regex;
extern crate clap;

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

}
