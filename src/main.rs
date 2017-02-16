pub mod download;
pub mod database;
pub mod engine;

extern crate curl;
extern crate flate2;
extern crate rusqlite;
extern crate regex;
extern crate clap;
extern crate term_painter;

use download::*;
use download::decompressor::*;
use database::*;
use database::parser::*;
use engine::*;
use term_painter::ToStyle;
use term_painter::Color::*;

fn main() {

    println!("{}", BrightBlue.paint("Welcome to Film-O-Mat!"));

    if !db_exists() {

        print!("Initialize Database...");
        create_database();

        println!(" Done.");
        print!("Download movies...");
        let movies = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                 de/pub/misc/movies/database/ratings.list.gz"));
        println!(" Done.");
        print!("Download genres...");
        let genres = decompress(&download_archiv("ftp://ftp.fu-berlin.\
                                                  de/pub/misc/movies/database/genres.list.gz"));
        println!(" Done.");
        print!("Start parsing movies...");
        parse_rating(movies.unwrap());
        println!(" Done.");
        print!("Start parsing genres...");
        parse_genre(genres.unwrap());
        println!(" Done.");
    }

    println!("Database ready.");

    output_result(execute(get_search_params()));

}
