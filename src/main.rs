//! The Film-O-Mat -- The only tRUSTworthy movie suggestion engine
//!
//! You want to invite your friends for a film night, you are already procRUSTinating your
//! programming exercises or you just want to have a relaxed night with popcorn and cRUSTy pizza,
//! but all search engines are just fRUSTrating and always suggest the wrong movies?
//!
//! Then the Film-O-Mat should be your weapon of choice! It is just RUSTastic, completely save and
//! absolutely tRUSTworthy.
//!
//! Just type in movies you like, your favourite genres or rely on the IMDb community's ratings.
pub mod download;
pub mod database;
pub mod engine;

extern crate curl;
extern crate flate2;
extern crate rusqlite;
extern crate regex;
extern crate clap;
extern crate term_painter;
extern crate os_type;

use download::*;
use download::decompressor::*;
use database::*;
use database::parser::*;
use engine::{get_search_params, output_result};
use term_painter::ToStyle;
use term_painter::Color::BrightBlue;

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
