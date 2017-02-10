extern crate term_painter;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;
use ::Ergebnis;

fn buildRank(x: &str) -> Vec<String> {
    let ziffer = match x {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "10" => 10,
        &_ => 10,
    };
    let mut rank = Vec::new();
    rank.push(String::from("("));
    rank.push(String::new());
    for i in 0..10-ziffer {
        rank[1].push('-');
    }
    rank.push(String::new());
    for i in 0..ziffer {
        rank[2].push('*');
    }
    rank.push(String::from(")"));
    rank
}

//x: &str, y: &Vec<String>, act: &Vec<&str>, gen: &str, year: &str, input: &Vec<&str>

/*
pub fn outputResult(out: Vec<result>) {
    for i in 0..out.len();
    let mut colors = Vec::new();
    for i in 0..3 + y.len() + act.len() {
        colors.push(White);
    }
    let count = createHits(x, y, act, gen, year, input, &mut colors);
    let hits = match count {
        0|1|2|3|4|5|6|7|8|9 => 15,
        100 => 17,
        _ => 16,
    };
    let lengthFilm = 25 + x.chars().count() + 12;
    let formatFilm = format!("{:-<1$}", "-", lengthFilm);
    print!("{}\n", formatFilm); //Header
    print!("= {} {}%{}=\n", Blue.paint("Hit Rate:"), Blue.paint(count), format!("{: <1$}", " ", lengthFilm - hits)); 
    print!("= Film: {}         Rank: {}{}{}{} =\n", colors[0].paint(x), y[0], y[1], colors[3].paint(&y[2]), y[3]); //Movie and Rank, longest
    print!("Regisseur: {}{}\n", colors[4].paint())
    print!("= Main Actors:{}=\n", format!("{: <1$}", " ", lengthFilm - 15));
    for u in 0..act.len() {
        print!("=  - {}{}=\n", colors[5+u].paint(&act[u]), format!("{: <1$}", " ", lengthFilm - act[u].chars().count() - 6));
    }
    print!("= Genre: {}{} =\n", colors[5+act.len()].paint(gen), format!("{: <1$}", " ", lengthFilm - 11 - gen.chars().count())); //Genre
    print!("= Year: {}{} =\n", colors[6+act.len()].paint(year), format!("{: <1$}", " ", lengthFilm - 14)); //Year
    print!("{}\n", formatFilm); //Bottom
}

*/

pub fn outputResult(out: Vec<Ergebnis>, input: &Vec<String>) {
    print!("You were searching for the following:\n");
    for y in 0..input.len(){
        print!("- {}\n", input[y]);
    }
    print!("\nThese are the {}\n", Red.paint("results:"));
    for i in 0..out.len() {
        let mut colors = Vec::new();
        for u in 0..6+out[i].actors.len() {
            colors.push(White);
        }
        let count = colorize(&out[i], &input, &mut colors);
        let hits = match count {
            0|1|2|3|4|5|6|7|8|9 => 15,
            100 => 17,
            _ => 16,
        };
        let ranking = buildRank(out[i].rank.as_str());
        let lengthFilm = 25 + out[i].name.chars().count() + 12;
        let formatFilm = format!("{:-<1$}", "-", lengthFilm);
        print!("{}\n", formatFilm); //Header
        print!("= {} {}%{}=\n", Blue.paint("Hit Rate:"), Blue.paint(count), format!("{: <1$}", " ", lengthFilm - hits)); 
        print!("= Film: {}         Rank: {}{}{}{} =\n", colors[0].paint(&out[i].name), ranking[0], ranking[1], colors[1].paint(&ranking[2]), ranking[3]); //Movie and Rank, longest
        print!("= Amount Evaluations: {}{} =\n", colors[2].paint((&out[i].wertungen).to_string()), format!("{: <1$}", " ", lengthFilm - 24 - out[i].wertungen.len()));
        print!("= Regisseur: {}{}=\n", colors[3].paint(&out[i].regisseur), format!("{: <1$}", " ", lengthFilm - 14 - out[i].regisseur.len()));
        print!("= Main Actors:{}=\n", format!("{: <1$}", " ", lengthFilm - 15));
        for u in 0..out[i].actors.len() {
            print!("=  - {}{}=\n", colors[4+u].paint(&out[i].actors[u]), format!("{: <1$}", " ", lengthFilm - out[i].actors[u].chars().count() - 6));
        }
        print!("= Genre: {}{} =\n", colors[colors.len()-2].paint(&out[i].genre), format!("{: <1$}", " ", lengthFilm - 11 - out[i].genre.chars().count())); //Genre
        print!("= Year: {}{} =\n", colors[colors.len()-1].paint(&out[i].year), format!("{: <1$}", " ", lengthFilm - 14)); //Year
        print!("{}\n", formatFilm); //Bottom
    }
}

fn colorize<'a>(out: &'a Ergebnis, input: &Vec<String>, colors: &mut Vec<self::term_painter::Color>) -> usize {
    let mut data = Vec::new();
    let mut count = 0;
    data.push(&out.name);
    data.push(&out.rank);
    data.push(&out.wertungen);
    data.push(&out.regisseur);
    for i in 0..out.actors.len() {
        data.push(&out.actors[i]);
    }
    data.push(&out.genre);
    data.push(&out.year);
    colors[1] = Yellow;

    for u in 0..input.len() {
        for v in 0..data.len() {
            if(data[v].eq(&input[u])) {
                colors[v] = Green;
                count = count + 1;
                break;
            }
        }
    }

    (count * 100)/input.len()
}