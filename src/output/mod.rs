extern crate term_painter;

use self::term_painter::ToStyle;
use self::term_painter::Color::*;
use self::term_painter::Attr::*;

pub fn buildRank(x: u8) -> Vec<String> {
    let mut rank = Vec::new();
    rank.push(String::from("("));
    rank.push(String::new());
    for i in 0..10-x {
        rank[1].push('-');
    }
    rank.push(String::new());
    for i in 0..x {
        rank[2].push('*');
    }
    rank.push(String::from(")"));
    rank
}

pub fn outputResult(x: &str, y: &Vec<String>, act: &Vec<&str>, gen: &str, year: &str, input: &Vec<&str>) {
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
    print!("= Film: {}         Rank: {}{}{}{} =\n",colors[0].paint(x), y[0], y[1], colors[3].paint(&y[2]), y[3]); //Movie and Rank, longest
    print!("= Main Actors:{}=\n", format!("{: <1$}", " ", lengthFilm - 15));
    for u in 0..act.len() {
        print!("=  - {}{}=\n", colors[5+u].paint(&act[u]), format!("{: <1$}", " ", lengthFilm - act[u].chars().count() - 6));
    }
    print!("= Genre: {}{} =\n", colors[5+act.len()].paint(gen), format!("{: <1$}", " ", lengthFilm - 11 - gen.chars().count())); //Genre
    print!("= Year: {}{} =\n", colors[6+act.len()].paint(year), format!("{: <1$}", " ", lengthFilm - 14)); //Year
    print!("{}\n", formatFilm); //Bottom
}

fn createHits(x: &str, y: &Vec<String>, act: &Vec<&str>, gen: &str, year: &str, input: &Vec<&str>, 
colors: &mut Vec<self::term_painter::Color>) -> usize {
    let mut data = Vec::new();
    let mut count = 0;
    data.push(x);
    for i in 0..4 {
        data.push(&y[i]);
    }
    for i in 0..act.len() {
        data.push(act[i]);
    }
    data.push(gen);
    data.push(year);
    colors[3] = Yellow;

    for u in 0..input.len() {
        for v in 0..data.len() {
            if(data[v].eq(input[u])) {
                colors[v] = Green;
                count = count + 1;
                break;
            }
        }
    }

    (count * 100)/input.len()
}
