use term_painter::ToStyle;
use term_painter::Attr::*;
use std::fmt;
use std::io;
use std::process;

#[derive(Clone)]
pub struct SearchResult {
    pub title: String,
    pub score: String,
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
        let score: f32 = self.score.parse::<f32>().unwrap();
        for i in 0..10 {
            if i < (score + 0.5) as i32 {
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
               "\n{}, ({} bei {} Bewertungen)",
               stars,
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

pub fn output_result(results: Vec<SearchResult>) {
    let further = results.clone();
    for res in results.into_iter().take(3) {
        println!("{}", res);
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
    println!("Sorry, no movie matches to your given Params!");
    process::exit(0);
}
