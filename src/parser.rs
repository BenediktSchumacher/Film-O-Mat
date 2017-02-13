use regex::Regex;
use database::*;

macro_rules! regex { ($re:expr) => { ::regex::Regex::new($re).unwrap() } }

/// Represents a movie
// #[derive(Debug)]
// Brauchen wir das struct überhaupt?

// struct Film {
//     name: String,
//     votes: String,
//     rating: String,
//     year: String,
// }

// impl Film {
//     fn new() -> Self {
//         Film {
//             name: String::new(),
//             votes: String::new(),
//             rating: String::new(),
//             year: String::new(),
//         }
//     }
// }

// #[derive(Debug)]
// struct Genre {
//     movie: String,
//     genre: String,
// }
// impl Genre {
//     fn new() -> Self {
//         Genre {
//             movie: String::new(),
//             genre: String::new(),
//         }
//     }
// }

pub fn parse_rating(string: String) {
    // let re = Regex::new(r"(\n [\s]+ [\d|\.]{10}[\s]+ (\d+)[\s]+ ([\d|\.]{3}) (.+) [(](\d{4})[)])")
    let re = Regex::new("(\\n [\\s]+ [\\d|\\.]{10}[\\s]+ (\\d+)[\\s]+ \
                         ([\\d|\\.]{3})[\\s]{2}([^\u{0022}]{1}.+) \\(([\\d]{4})\\))")
        .unwrap(); //                    ^number     ^rating      ^title  ^year

    println!("{:?}", &re);

    for cap in re.captures_iter(string.as_str()) {
        let votes = &cap[2].parse::<i32>().unwrap();
        if *votes > 100000 {
            import_movie(&cap[4].trim(),
                         &cap[5].trim(),
                         &cap[3].trim(),
                         &cap[2].trim());
        }
    }
}

fn parse_genre(string: String) {
    // Series starting with "quotation marks" are filtered out
    let re = Regex::new(
        "(\\n([^\u{0022}].*)[\\s]+[\\(](\\d{4})[\\)][\\s]+(?:\\(V\\)|\\(TV\\))?[\\s]*([^\\{].+))"
    ).unwrap();
    //                ^title             ^year               ^filter for (V)/(TV)           ^genre 
    for cap in re.captures_iter(string.as_str()) {
        add_genres(&cap[2],
                   &cap[3],
                   &cap[4]);
    }
}

// TODO: Filter series with form "NAME (YEAR)   {EPISODE INFO}"
// pub fn parse_movies(string: String) {
// let re = Regex::new(r"(\n\u{0022}?[#|\+|\-|\*|\/|\.|\,|\!|\:|\&|\%|\$|\§|\w|\d|\s]*\u{0022}? \([\d]{4}\))").unwrap();
// let re = Regex::new("(\\n\u{0022}?[#|\\+|\\-|\\*|\\041|\\057|\\054|\\056|\\072|\\073|\\044|\\045|\\046|\\w|\\d|\\s]*\u{0022}? \\([\\d]{4}\\))")
// .unwrap();
// println!("{:?}", re);
// for cap in re.captures_iter(string.as_str()) {
// let mut movie = &cap[0];
// movie = movie.trim();
// let tmp: Vec<&str> = movie.rsplitn(2, ' ').collect();
// let mut year = tmp[0];
// year = year.trim();
// import_movie(tmp[1], &year[1..5]);
// }
// }

pub fn parse_actors() {
    // TODO: implement it!
    // Will it ever be implemented? :((
}

pub fn parse_directors() {
    // TODO: implement it!
}
