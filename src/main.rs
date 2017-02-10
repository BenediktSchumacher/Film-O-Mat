use std::io::prelude::*;
use::std::env;

use std::fs::File;

#[derive(Debug)]
struct Film {
    name: String,
    votes: String,
    rating: String,
    year: String,
}
impl Film {
    fn new() -> Self {
        Film {
            name: String::new(),
            votes: String::new(),
            rating: String::new(),
            year: String::new(),
        }
    }
}

fn main() {
    // hardcoded test file
    let mut f = File::open("test");
    let mut buffer = String::new();

    // TODO: Error handling
    // panics if errror occurs
    f.unwrap().read_to_string(&mut buffer);

    // skip useless sections at the beginning
    // let buffer_without_beginning = buffer.split("-----")

    // lines of the file in Vector
    let lines:Vec<&str> = buffer
        // teile in lines auf
        .split("\n")
        // skippe unnoetigen kram am anfang
        .skip(500)
        // skippe unnoetigen kram am ende
        .take_while(|line| !line
            .contains("------------------------------------------------------------------------------"))
        // into vector
        .collect();
    // Debug
    // println!("{:?}", lines);

    let mut all_votes:Vec<&str> = Vec::new();
    let mut all_films:Vec<Film> = Vec::new();

    // gehe alle lines des files durch
    for line in lines {
        let words_in_line:Vec<&str> = line.split_whitespace().collect();
        // first does not interust us
        let mut iter = words_in_line.iter().skip(1);
        // vote
        let vote = iter.next();
        // rating
        let rating = iter.next();
        // // not working because every film has different count of words
        // // name
        // let name = iter.take_while(|film_name| ;
        // // year
        // let year = iter.next();
        let mut film = Film::new();
        match vote {
            Some(vote) => film.votes.push_str(*vote),
            None => {},
        };
        match rating {
            Some(rating) => film.rating.push_str(*rating),
            None => {},
        };
        all_films.push(film);
    }
    println!("{:?}", all_films);
}
