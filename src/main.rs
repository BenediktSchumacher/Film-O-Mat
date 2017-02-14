pub mod output;
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
use output::*;


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

    output_result(execute(get_search_params()));

}

fn tmp() {
    // Only for testing
    // let mut input = Vec::new();
    // input.push(String::from("Cool Cat"));
    // input.push(String::from("Meh"));
    // input.push(String::from("10"));
    // input.push(String::from("Bruce Lee"));
    // let mut test = Vec::new();
    // let mut act = Vec::new();
    // act.push(String::from("Bruce Lee"));
    // act.push(String::from("Jason Statham"));
    // act.push(String::from("Cool Cat"));
    // let test1 = Ergebnis {
    // name: String::from("Star Wars"),
    // rank: String::from("8"),
    // wertungen: String::from("3450"),
    // regisseur: String::from("Steven Spielberg"),
    // actors: act,
    // genre: String::from("Action"),
    // year: String::from("2012"),
    // };
    // test.push(test1);
    // let mut actTwo = Vec::new();
    // actTwo.push(String::from("Irgendwer"));
    // actTwo.push(String::from("Geralt von Riva"));
    // let test2 = Ergebnis {
    // name: String::from("Hallo Welt"),
    // rank: String::from("1"),
    // wertungen: String::from("140"),
    // regisseur: String::from("Mein Friseur"),
    // actors: actTwo,
    // genre: String::from("Stupid"),
    // year: String::from("1980"),
    // };
    // test.push(test2);
    // let mut actThree = Vec::new();
    // actThree.push(String::from("The Rock"));
    // actThree.push(String::from("James Bond"));
    // let test3 = Ergebnis {
    // name: String::from("Der Film der Filme"),
    // rank: String::from("10"),
    // wertungen: String::from("154365"),
    // regisseur: String::from("David Zucker"),
    // actors: actThree,
    // genre: String::from("Geilomeilo"),
    // year: String::from("2016"),
    // };
    // test.push(test3);
    //
    // output::outputResult(test, &input);
    //
}
