use clap::{App, Arg};
use std::process;

// This Struct is later given to our main program,
// it contains all the needed information to make suggestions of films
#[derive(Debug, Clone)]
pub struct SearchParams {
    genres: Vec<String>,
    movies: Vec<String>,
    actors: Vec<String>,
    rating: Vec<f32>,
}

impl SearchParams {
    // initialize a SearchParams type with empty fields
    fn init() -> Self {
        SearchParams {
            genres: Vec::new(),
            movies: Vec::new(),
            actors: Vec::new(),
            rating: Vec::new(),
        }
    }
}

// Creates struct SearchParams in which all given parameters are included.
pub fn get_search_params() -> SearchParams {
    // status of program
    println!("Getting Commandline Input ...");

    // later given to main program, contains all arguments given in
    // command line
    let mut search_params = SearchParams::init();

    // our matches, we can later use those to handle our commandline input
    let matches = App::new("Film-O-Mat")
        .arg(Arg::with_name("genre")
            .short("g")
            .help("Takes one or more genres")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("actor")
            .short("a")
            .help("Takes an actor")
            .takes_value(true))
        .arg(Arg::with_name("movie")
            .short("m")
            .help("Takes one or more movies. e.g. \u{0022}Schindler's List\u{0022}")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("rating")
            .short("r")
            .help("Takes a rating")
            .takes_value(true))
        .get_matches();

    // If a genre was given, save it in our SearchParams Type
    if matches.is_present("genre") {
        let genres: Vec<String> = match matches.values_of("genre") {
            Some(vals) => {
                vals.map(|x| x.to_string())
                    .collect()
            }
            None => {
                println!("Invalid Genre");
                process::exit(0);
            }
        };
        search_params.genres = genres;
        // Debug
        // println!("Genre: {}", genre);
    }

    // If an actor was given, save it in our SearchParams Type
    if matches.is_present("actor") {
        let actor = match matches.value_of("actor") {
            Some(actor) => actor,
            None => {
                println!("Invalid Actor");
                process::exit(0);
            }
        };
        search_params.actors.push(actor.into());
        // Debug
        // println!("Actor: {}", actor);
    }

    // If a genre was given, save it in our SearchParams Type
    if matches.is_present("movie") {
        let movies: Vec<String> = match matches.values_of("movie") {
            Some(vals) => {
                vals.map(|x| x.to_string())
                    .collect()
            }
            None => {
                println!("Invalid Movie");
                process::exit(0);
            }
        };
        search_params.movies = movies;
        // Debug
        // println!("Movies: {:?}", search_params.genre);
    }

    if matches.is_present("rating") {
        let val = match matches.value_of("rating") {
            Some(vals) => vals,
            None => {
                println!("Invalid Rating");
                process::exit(0);
            }
        };
        search_params.rating.push(
            match val.parse() {
                Ok(val) => val,
                Err(err) => {
                    println!("{:?}", err);
                    process::exit(0);
                }
            }
        );
        // Debug
        // println!("Rating: {:?}", search_params.rating);
    }

    // Debug
    // println!("{:?}", search_params);
    search_params
}
