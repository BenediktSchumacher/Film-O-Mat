// clap used as argument handler
extern crate clap;

use clap::{App, Arg};

// This Struct is later given to our main program,
// it contains all the needed information to make suggestions of films
#[derive(Debug, Clone)]
struct SearchParams {
    genre: Vec<String>,
    movies: Vec<String>,
    actors: Vec<String>,
}

impl SearchParams {
    // initialize a SearchParams type with empty fields
    fn init() -> Self {
        SearchParams {
            genre: Vec::new(),
            movies: Vec::new(),
            actors: Vec::new(),
        }
    }
}

// Creates struct SearchParams in which all given parameters are included.
fn get_search_params() -> SearchParams {
    // later given to main program, contains all arguments given in
    // command line
    let mut search_params = SearchParams::init();

    // our matches, we can later use those to handle our commandline input
    let matches = App::new("FOM")
        .arg(Arg::with_name("genre")
            .short("g")
            .takes_value(true))
        .arg(Arg::with_name("actor")
            .short("a")
            .takes_value(true))
        .arg(Arg::with_name("movie")
            .short("m")
            .takes_value(true)
            .multiple(true))
        .get_matches();

    // If a genre was given, save it in our SearchParams Type
    if matches.is_present("genre") {
        let genre = match matches.value_of("genre") {
            Some(genre) => genre,
            None => panic!(),
        };
        search_params.genre.push(genre.into());
        // Debug
        // println!("Genre: {}", genre);
    }

    // If an actor was given, save it in our SearchParams Type
    if matches.is_present("actor") {
        let actor = match matches.value_of("actor") {
            Some(actor) => actor,
            None => panic!(),
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
            None => panic!(),
        };
        search_params.movies = movies;
        // Debug
        // println!("Movies: {:?}", search_params.genre);
    }
    // Debug
    // println!("{:?}", search_params);

    // return struct with all needed search_params
    search_params
}

fn main() {
    // Test
    // possible test_input could be
    // cargo run -- -a Name -m Lieblingsfilm1 Lieblingsfilm2 -g horror

    // let args = get_search_params();
    // println!("{:?}", args);
}
