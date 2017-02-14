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
use input::*;

fn main() {

    // TODO: Pruefem, ob Datenbank vorhanden ist
    // Falls nein => Dateipfad erstellen, Datenbank anlegen und anschliessend fuellen
    // Falls ja => Datenbankverbindung herstellen, Anfrage bearbeiten

    println!("Welcome to Film-O-Mat");

    if !db_exists() {

        print!("Creating Database...");
        create_database();

        print!(" Done \nDownload movies...");
        let movies = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                 de/pub/misc/movies/database/ratings.list.gz"));
        print!(" Done. \nDownload genres...");
        let genres = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                  de/pub/misc/movies/database/genres.list.gz"));
        print!(" Done. \nStart parsing movies...");
        parse_rating(movies.unwrap());
        print!(" Done. \nStart parsing genres...");
        parse_genre(genres.unwrap());
        println!(" Done.");
    }

    println!("Database ready!");

    execute(get_search_params());

}
