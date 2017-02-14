use regex::Regex;
use database::*;

/// Parser for a string of many movies and their ratings.
/// For movie identity the year parsed. To filter movies with too less ratings,
/// the number of ratings is parsed, too.
/// Series are marked with quotation marks. Series are not encountered. All 
/// entries with title "[Series Title]" are filtered out.
pub fn parse_rating(string: String) {
    // (old regex, can be deleted later)let re = Regex::new(r"(\n [\s]+ [\d|\.]{10}[\s]+ 
    // (\d+)[\s]+ ([\d|\.]{3}) (.+) [(](\d{4})[)])")
    let re = Regex::new("(\\n [\\s]+ [\\d|\\.]{10}[\\s]+ (\\d+)[\\s]+ \
                         ([\\d|\\.]{3})[\\s]{2}([^\u{0022}]{1}.+) \\(([\\d]{4})\\))")
        .unwrap();  //                                    ^number of ratings          
                    //     ^rating                            ^title   ^year 
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

/// Parser for a string of movies and their genres.
/// Again for movie identity the year is parsed, too. Series with title
/// "[Series Title]" are filtered out. 
pub fn parse_genre(string: String) {
    // Series starting with "quotation marks" are filtered out
    let re = Regex::new("(\\n([^\u{0022}].*)[\\s]+[\\(](\\d{4})[\\)][\\s]+(?:\
                         \\(V\\)|\\(TV\\))?[\\s]*([^\\{].+))")
        .unwrap();  //                   ^title         ^year 
                    //   ^filter for (V)/(TV)           ^genre         ^genre
    for cap in re.captures_iter(string.as_str()) {
        add_genres(&cap[2], &cap[3], &cap[4]);
    }
}

pub fn parse_actors() {
    // TODO: implement it!
    // Will it ever be implemented? :((
}

pub fn parse_directors() {
    // TODO: implement it!
}
