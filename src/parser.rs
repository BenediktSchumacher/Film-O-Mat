use regex::Regex;
use database::*;

macro_rules! regex { ($re:expr) => { ::regex::Regex::new($re).unwrap() } }

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

#[derive(Debug)]
struct Genre {
    movie: String,
    genre: String,
}
impl Genre {
    fn new() -> Self {
        Genre {
            movie: String::new(),
            genre: String::new(),
        }
    }
}

pub fn parse_rating(string: String) {
    let mut all_films: Vec<Film> = Vec::new();

    let re = Regex::new(
        r"(\n [\s]+ [\d|\.]{10}[\s]+ (\d+)[\s]+ ([\d|\.]{3}) (.+) [(](\d{4})[)])"
    ).unwrap();

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

fn parse_genre(string: String) {

    // // hardcoded test file
    // let mut f = File::open("test2");
    // let mut buffer = String::new();
    // // TODO: Error handling
    // f.unwrap().read_to_string(&mut buffer);

    let mut all_genres:Vec<Genre> = Vec::new();

    let re = Regex::new(r"(\n(.+)[\s]+[(]\d{4}[)][\s]+(?:\(V\)|\(TV\))?[\s]*([^\{].+))").unwrap();
    //                     ^ title     ^ year          ^ (TV|V) filter            ^ Genre
    for cap in re.captures_iter(string.as_str()) {

        let mut genre = Genre::new();
        // Debug
        // println!("Name: {} Genre: {}", &cap[2], &cap[3] );
        genre.movie.push_str(&cap[2]);
        genre.genre.push_str(&cap[3]);
        all_genres.push(genre);
    }

    // all_genres
    // println!("{:?}", all_films);
}

pub fn parse_movies(string: String) {
    // let re = Regex::new(r"(\n\u{0022}?[#|\+|\-|\*|\/|\.|\,|\!|\:|\&|\%|\$|\§|\w|\d|\s]*\u{0022}? \([\d]{4}\))").unwrap();
    let re = Regex::new("(\\n\u{0022}?[#|\\+|\\-|\\*|\\041|\\057|\\054|\\056|\\072|\\073|\\044|\\045|\\046|\\w|\\d|\\s]*\u{0022}? \\([\\d]{4}\\))")
        .unwrap();
    println!("{:?}", re);
    for cap in re.captures_iter(string.as_str()) {
        let mut movie = &cap[0];
        movie = movie.trim();
        let tmp: Vec<&str> = movie.rsplitn(2, ' ').collect();
        let mut year = tmp[0];
        year = year.trim();
        import_movie(tmp[1], &year[1..5]);
    }
}
