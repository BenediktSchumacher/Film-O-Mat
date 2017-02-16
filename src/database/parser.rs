//! Parser to evaluate long strings of movies and genres. 
use regex::Regex;
use database::{import_movie, add_genres};

/// Parses a ratings string with movies and ratings. Adds the results to the movie database. 
pub fn parse_rating(string: String) {
    let re = Regex::new("(\\n [\\s]+ [\\d|\\.]{10}[\\s]+ (\\d+)[\\s]+ \
                         ([\\d|\\.]{3})[\\s]{2}([^\u{0022}]{1}.+) \\(([\\d]{4})\\))")
        .unwrap();

    for cap in re.captures_iter(string.as_str()).skip(260) {
        let votes = &cap[2].parse::<i32>().unwrap();
        if *votes > 50000 {
            import_movie(&cap[4].trim(),
                         &cap[5].trim(),
                         &cap[3].trim(),
                         &cap[2].trim());
        }
    }
}

/// Parses a genre string with movies and their corresponding ratings. Adds the results to the
/// genre database.
pub fn parse_genre(string: String) {
    // Series starting with "quotation marks" are filtered out
    let re = Regex::new("(\\n([^\u{0022}].*)[\\s]+[\\(](\\d{4})[\\)][\\s]+(?:\
                         \\(V\\)|\\(TV\\)|\\(VG\\))?[\\s]*([^\\{].+))")
        .unwrap();
    for cap in re.captures_iter(string.as_str()) {
        add_genres(&cap[2], &cap[3], &cap[4]);
    }
}
