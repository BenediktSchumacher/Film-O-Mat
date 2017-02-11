extern crate regex;

use std::io::prelude::*;
use ::std::env;
use regex::Regex;
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

// Now with Regex
fn main() {
    // hardcoded test file
    let mut f = File::open("test");
    let mut buffer = String::new();
    // TODO: Error handling
    f.unwrap().read_to_string(&mut buffer);

    let mut all_films:Vec<Film> = Vec::new();

    let re = Regex::new(r"(\n [\s]+ [\d|\.]{10}[\s]+ (\d+)[\s]+ ([\d|\.]{3}) (.+) ([(](\d{4})[)]))").unwrap();
    for cap in re.captures_iter(buffer.as_str()) {
    let mut film = Film::new();
        // Debug
        // println!("Name:{} Votecount:{} Rating:{} Year:{}", &cap[4], &cap[2], &cap[3], &cap[5] );
        film.name.push_str(&cap[4]);
        film.votes.push_str(&cap[2]);
        film.rating.push_str(&cap[3]);
        film.year.push_str(&cap[5]);
        all_films.push(film);
    }

    println!("{:?}", all_films);
}

// Ugly code, do not worry my child, deletion already sheduled
// fn old() {
//     // hardcoded test file
//     let mut f = File::open("test");
//     let mut buffer = String::new();

//     // TODO: Error handling
//     f.unwrap().read_to_string(&mut buffer);

//     // lines of the file in Vector
//     let lines:Vec<&str> = buffer
//         // teile in lines auf
//         .split("\n")
//         // skippe unnoetigen kram am anfang
//         .skip(500)
//         // skippe unnoetigen kram am ende
//         .take_while(|line| !line
//             .contains("------------------------------------------------------------------------------"))
//         // into vector
//         .collect();
//     // Debug
//     // println!("{:?}", lines);

//     let mut all_votes:Vec<&str> = Vec::new();
//     let mut all_films:Vec<Film> = Vec::new();

//     // gehe alle lines des files durch
//     for line in lines {
//         let mut film = Film::new();
//         let words_in_line:Vec<&str> = line.split_whitespace().collect();
//         // first does not interust us
//         let mut iter = words_in_line.iter().skip(1);

//         // vote
//         let vote = iter.next();
//         match vote {
//             Some(vote) => film.votes.push_str(*vote),
//             None => {},
//         };

//         // rating
//         let rating = iter.next();
//         match rating {
//             Some(rating) => film.rating.push_str(*rating),
//             None => {},
//         };

//         // name
//         let re = Regex::new(r"[(](\d{4})[)]").unwrap();
//         let name:Vec<String> = iter
//             .clone()
//             .take_while(|year| !re.is_match(year))
//             .map(|word| (*word).trim())
//             .map(|word| (*word).to_string())
//             .collect();
//         film.name = name;

//         // year
//         let year = iter
//             .skip_while(|year| !re.is_match(year))
//             .next();
//         match year {
//             Some(year) => film.year.push_str(*year),
//             None => {},
//         };
//         all_films.push(film);
//     }
//     println!("{:?}", all_films);
// }
