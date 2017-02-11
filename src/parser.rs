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

pub fn parse_rating(string: String) {
    let mut all_films: Vec<Film> = Vec::new();

    let re =
        Regex::new(r"(\n [\s]+ [\d|\.]{10}[\s]+ (\d+)[\s]+ ([\d|\.]{3}) (.+) ([(](\d{4})[)]))")
            .unwrap();
    for cap in re.captures_iter(string.as_str()) {
        let mut film = Film::new();
        // Debug
        // println!("Name:{} Votecount:{} Rating:{} Year:{}", &cap[4], &cap[2], &cap[3], &cap[5] );
        film.name.push_str(&cap[4]);
        film.votes.push_str(&cap[2]);
        film.rating.push_str(&cap[3]);
        film.year.push_str(&cap[5]);
        all_films.push(film);
    }

    println!("{:?}", all_films[5]);
}

pub fn parse_movies(string: String) {}
