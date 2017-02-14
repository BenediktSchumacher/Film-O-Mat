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
extern crate term_painter;

use download::*;
use download::decompressor::*;
use database::*;
use parser::*;
use input::*;
use output::*;


fn main() {

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
