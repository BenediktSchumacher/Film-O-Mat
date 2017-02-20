//! Gets the commandline input and packs it into an usable Stuct.
//! Data that can be included are genres, movies, rating.
use clap::{App, Arg};
use term_painter::ToStyle;
use term_painter::Attr::Bold;
use term_painter::Color::{BrightGreen, Yellow};
use std::{fmt, io, process};
use os_type;

/// Struct which contains all needed information to make film suggestions.
#[derive(Debug, Clone)]
pub struct SearchParams {
    genres: Vec<String>,
    movies: Vec<String>,
    rating: f64,
}

impl SearchParams {
    /// Initializes a SearchParams type with empty fields.
    fn init() -> Self {
        SearchParams {
            genres: Vec::new(),
            movies: Vec::new(),
            rating: 0.0,
        }
    }

    /// Returns a vector of Genres.
    pub fn get_genres(&self) -> Vec<String> {
        self.genres.clone()
    }

    // Returns a vector of Movies.
    pub fn get_movies(&self) -> Vec<String> {
        self.movies.clone()
    }

    /// Returns a rating.
    pub fn get_rating(&self) -> f64 {
        self.rating
    }
}

/// Struct which contains all information attached to a single movie.
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
        let os = os_type::current_platform();
        try!(write!(f,
                    "{}",
                    Bold.paint(format!("{} ({})\n", &self.title, &self.year))));
        let mut stars = String::new();
        let score: f64 = self.score;
        for i in 0..10 {
            if i < (score + 0.5) as i64 {
                match os {
                    os_type::OSType::Windows => stars.push_str("*"),
                    _ => stars.push_str("\u{2605}"),
                }
            } else {
                match os {
                    os_type::OSType::Windows => stars.push_str(" "),
                    _ => stars.push_str("\u{2606}"),
                }
            }
        }
        try!(write!(f, "{}", &self.genres[0]));
        for genre in self.genres.clone().into_iter().skip(1) {
            try!(write!(f, ", {}", genre));
        }
        write!(f,
               "\n{} ({} at {} ratings)",
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

/// Creates a struct SearchParams with the parameters given by commandline arguments.
pub fn get_search_params() -> SearchParams {
    // Status of program
    println!("Getting Commandline Input ...");

    // Search parameters
    let mut search_params = SearchParams::init();

    // Matches, we can later use those to handle our commandline input
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

/// Checks if a given Genre is a correct genre for IMDb database. The genre string is case sensitive
/// and doesn't allow typos or character replacement like "Film Noir" instead of "Film-Noir".
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

/// Checks if a given rating is a valid float number.
fn is_rating(value: String) -> Result<(), String> {
    match value.parse::<f64>() {
        Ok(_) => Ok(()),
        _ => Err(String::from("Given Rating is not a number")),
    }
}

/// Prints the movies that match the input parameters. The movies are printed in descending order
/// starting with the highest rating. If more than 3 movies match, the first 3 are displayed
/// immediately and all others can be accessed consecutively by pressing enter. The suggestion
/// process can be aborted by entering "q".
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
            let compare = match os_type::current_platform() {
                os_type::OSType::Windows => "q\r\n",
                _ => "q\n",
            };
            if buffer != compare {
                println!("{}", output);
            } else {
                process::exit(0);
            }
        }
    }
}

/// If no matching movie is found, a sad message is displayed and the program is ended.
pub fn cancel_request() {
    println!("Sorry, no movie matches to your given Params! \u{2639}");
    process::exit(0);
}
