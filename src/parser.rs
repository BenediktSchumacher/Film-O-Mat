use regex::Regex;
use database::*;

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

pub fn parse_genre(string: String) {
    // Series starting with "quotation marks" are filtered out
    let re = Regex::new("(\\n([^\u{0022}].*)[\\s]+[\\(](\\d{4})[\\)][\\s]+(?:\
                         \\(V\\)|\\(TV\\))?[\\s]*([^\\{].+))")
        .unwrap();
    //                ^title             ^year               ^filter for (V)/(TV)           ^genre
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
