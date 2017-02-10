mod output;

pub struct Ergebnis {
    name : String,
    rank : String,
    wertungen : String,
    regisseur : String,
    actors : Vec<String>,
    genre : String,
    year : String,
}

fn main() {
    //Only for testing
    let mut input = Vec::new();
    input.push(String::from("Cool Cat"));
    input.push(String::from("Meh"));
    input.push(String::from("10"));
    input.push(String::from("Bruce Lee"));
    let mut test = Vec::new();
    let mut act = Vec::new();
    act.push(String::from("Bruce Lee"));
    act.push(String::from("Jason Statham"));
    act.push(String::from("Cool Cat"));
    let test1 = Ergebnis{name: String::from("Star Wars"), rank: String::from("8"), wertungen: String::from("3450"), regisseur: String::from("Steven Spielberg"), actors: act, genre: String::from("Action"), year: String::from("2012")};
    test.push(test1);
    let mut actTwo = Vec::new();
    actTwo.push(String::from("Irgendwer"));
    actTwo.push(String::from("Geralt von Riva"));
    let test2 = Ergebnis{name: String::from("Hallo Welt"), rank: String::from("1"), wertungen: String::from("140"), regisseur: String::from("Mein Friseur"), actors: actTwo, genre: String::from("Stupid"), year: String::from("1980")};
    test.push(test2);
    let mut actThree = Vec::new();
    actThree.push(String::from("The Rock"));
    actThree.push(String::from("James Bond"));
    let test3 = Ergebnis{name: String::from("Der Film der Filme"), rank: String::from("10"), wertungen: String::from("154365"), regisseur: String::from("David Zucker"), actors: actThree, genre: String::from("Geilomeilo"), year: String::from("2016")};
    test.push(test3);

    output::outputResult(test, &input);
}
