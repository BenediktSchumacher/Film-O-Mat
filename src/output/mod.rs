// extern crate term_painter;

//use term_painter::ToStyle;
//use term_painter::Color::*;
//use term_painter::Attr::*;

fn main() {
    //Only for testing
    let x = "Star Wars - The Last Generic";
    let xx = "Die Generic 3";
    let mut act = Vec::new();
    act.push("Bruce Lee");
    act.push("Jason Statham");
    act.push("Cool Cat");
    let mut actTwo = Vec::new();
    actTwo.push("Irgendwer");
    actTwo.push("Geralt von Riva");
    let yearOne = 1204;
    let yearTwo = 2013;
    let genOne = "Comedy";
    let genTwo = "Mystery";
    let y = buildRank(6);
    let z = buildRank (10);

    outputResult(x, &y, &act, genOne, yearOne);
    outputResult(xx, &z, &actTwo, genTwo, yearTwo);
}

fn buildRank(x: u8) -> Vec<String> {
    /*
    let mut rank = String::from("(");
    for i in 0..10-x {
        &rank.push('-');
    }
    for i in 0..x {
        &rank.push('*');
    }

    rank.push(')');
    rank
    **/

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

fn outputResult(x: &str, y: &Vec<String>, act: &Vec<&str>, gen: &str, year: u32) {
    let lengthFilm = 25 + x.chars().count() + 12;
    let formatFilm = format!("{:-<1$}", "-", lengthFilm);
    print!("{}\n", formatFilm); //Header
    print!("= Film: {}         Rank: {}{}{}{} =\n",x, y[0], y[1], y[2], y[3]); //Movie and Rank, longest
    print!("= Main Actors:{}=\n", format!("{: <1$}", " ", lengthFilm - 15));
    for u in 0..act.len() {
        print!("=  - {}{}=\n", act[u], format!("{: <1$}", " ", lengthFilm - act[u].chars().count() - 6));
    }
    print!("= Genre: {}{} =\n", gen, format!("{: <1$}", " ", lengthFilm - 11 - gen.chars().count())); //Genre
    print!("= Year: {}{} =\n", year, format!("{: <1$}", " ", lengthFilm - 14)); //Year
    print!("{}\n", formatFilm); //Bottom
}
