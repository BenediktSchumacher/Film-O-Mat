//! Gets the commandline input and packs it into an usable Stuct.
//! Data that can be included: genres, movies, rating.
use clap::{App, Arg};
use term_painter::ToStyle;
use term_painter::Attr::*;
use term_painter::Color::*;
use std::{fmt, io, process};

/// Struct contains all the needed information to make suggestions of films
#[derive(Debug, Clone)]
pub struct SearchParams {
    genres: Vec<String>,
    movies: Vec<String>,
    rating: f64,
}

impl SearchParams {
    /// initialize a SearchParams type with empty fields
    fn init() -> Self {
        SearchParams {
            genres: Vec::new(),
            movies: Vec::new(),
            rating: 0.0,
        }
    }

    pub fn get_genres(&self) -> Vec<String> {
        self.genres.clone()
    }

    pub fn get_movies(&self) -> Vec<String> {
        self.movies.clone()
    }

    pub fn get_rating(&self) -> f64 {
        self.rating
    }
}

#[derive(Clone)]
pub struct SearchResult {
    pub title: String,
    pub score: f64,
    pub number: String,
    pub genres: Vec<String>,
    pub year: String,
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f,
                    "{}",
                    Bold.paint(format!("{} ({})\n", &self.title, &self.year))));
        let mut stars = String::new();
        let score: f64 = self.score;
        for i in 0..10 {
            if i < (score + 0.5) as i64 {
                stars.push_str("\u{2605}");
            } else {
                stars.push_str("\u{2606}");
            }
        }
        try!(write!(f, "{}", &self.genres[0]));
        for genre in self.genres.clone().into_iter().skip(1) {
            try!(write!(f, ", {}", genre));
        }
        write!(f,
               "\n{}, ({} at {} ratings)",
               Yellow.paint(stars),
               &self.score,
               &self.number)
    }
}

impl fmt::Debug for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{:?}\n{:?}\n{:?}\n{:?}\n{:?}\n******",
               self.title,
               self.year,
               self.genres,
               self.score,
               self.number)
    }
}

/// Creates struct SearchParams with all given parameters
pub fn get_search_params() -> SearchParams {
    // status of program
    println!("Getting Commandline Input ...");

    // search parameters
    let mut search_params = SearchParams::init();

    // matches, we can later use those to handle our commandline input
    let matches = App::new("Film-O-Mat")
        .arg(Arg::with_name("genre")
            .short("g")
            .help("Takes one or more genres")
            .takes_value(true)
            .multiple(true)
            .validator(feasable_genre))
        .arg(Arg::with_name("movie")
            .short("m")
            .help("Takes one or more movies. e.g. \u{0022}Schindler's List\u{0022}")
            .takes_value(true)
            .multiple(true))
        .arg(Arg::with_name("rating")
            .short("r")
            .help("Takes a rating")
            .takes_value(true)
            .validator(is_rating))
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
    }

    // If a rating was given, save it in our SearchParams
    if matches.is_present("rating") {
        let val = match matches.value_of("rating") {
            Some(vals) => vals,
            None => {
                println!("Invalid Rating");
                process::exit(0);
            }
        };
        search_params.rating = match val.parse() {
            Ok(val) => val,
            Err(err) => {
                println!("{:?}", err);
                process::exit(0);
            }
        };
    }
    search_params
}

fn feasable_genre(value: String) -> Result<(), String> {
    match value.as_str() {
        "Action" => Ok(()),
        "Adult" => Ok(()),
        "Adventure" => Ok(()),
        "Animation" => Ok(()),
        "Biography" => Ok(()),
        "Comedy" => Ok(()),
        "Crime" => Ok(()),
        "Documentary" => Ok(()),
        "Drama" => Ok(()),
        "Family" => Ok(()),
        "Fantasy" => Ok(()),
        "Film-Noir" => Ok(()),
        "History" => Ok(()),
        "Horror" => Ok(()),
        "Music" => Ok(()),
        "Musical" => Ok(()),
        "Mystery" => Ok(()),
        "Romance" => Ok(()),
        "Sci-Fi" => Ok(()),
        "Short" => Ok(()),
        "Sport" => Ok(()),
        "Thriller" => Ok(()),
        "War" => Ok(()),
        "Western" => Ok(()),
        _ => Err(String::from("Given Genre is not valid")),
    }
}

fn is_rating(value: String) -> Result<(), String> {
    match value.parse::<f32>() {
        Ok(_) => Ok(()),
        _ => Err(String::from("Given Rating is not a number")),
    }
}

pub fn output_result(results: Vec<SearchResult>) {
    if results.is_empty() {
        cancel_request();
    }

    println!("{} match(es) found:", &results.len());

    let further = results.clone();
    for res in results.into_iter().take(3) {
        println!("\n{}", res);
    }

    if further.len() > 3 {
        println!("For further suggestions just {}. To quit, {}.",
            BrightGreen.paint("press ENTER"),
            BrightGreen.paint("type q and press ENTER"),
        );
    }

    for output in further.into_iter().skip(3) {
        let mut buffer = String::new();
        if io::stdin().read_line(&mut buffer).is_ok() {
            if buffer != "q\n" {
                println!("{}", output);
            } else {
                process::exit(0);
            }
        }
    }
}

pub fn cancel_request() {
    println!("Sorry, no movie matches to your given Params! \u{2639}");
    process::exit(0);
}
