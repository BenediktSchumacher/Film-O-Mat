fn main() {
    let x = "Star Wars - The Last Generic";
    let xx = "Die Generic 3";
    let yearOne = 1204;
    let yearTwo = 2013;
    let genOne = "Comedy";
    let genTwo = "Mystery";
    let y = buildRank(6);
    let z = buildRank (9);

    outputResult(x, y, genOne, yearOne);
    outputResult(xx, z, genTwo, yearTwo);
}

fn buildRank(x: u8) -> String {
    let mut rank = String::from("(");
    for i in 0..10-x {
        &rank.push('-');
    }
    for i in 0..x {
        &rank.push('*');
    }

    rank.push(')');
    rank
}

fn outputResult(x: &str, y: String, gen: &str, year: u32) {
    let lengthFilm = 25 + x.chars().count() + y.chars().count(); 
    let formatFilm = format!("{:-<1$}", "-", lengthFilm);
    let formatGen = format!("{: <1$}", " ", lengthFilm - 11 - gen.chars().count());
    let formatYear = format!("{: <1$}", " ", lengthFilm - 14);
    print!("{}\n", formatFilm);
    print!("= Film: {}         Rank: {} =\n",x, y ); //24
    print!("= Genre: {}{} =\n", gen, formatGen); //11
    print!("= Year: {}{} =\n", year, formatYear); //14
    print!("{}\n", formatFilm);
}
