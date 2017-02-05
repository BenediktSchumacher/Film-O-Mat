mod output;

fn main() {
    //Only for testing
    let x = "Star Wars - The Last Generic";
    let xx = "Die Generic 3";
    let xxx = "Die Telefloppies";
    let mut act = Vec::new();
    act.push("Bruce Lee");
    act.push("Jason Statham");
    act.push("Cool Cat");
    let mut actTwo = Vec::new();
    actTwo.push("Irgendwer");
    actTwo.push("Geralt von Riva");
    let mut actThree = Vec::new();
    actThree.push("The Rock");
    actThree.push("james Bond");
    let yearOne = "1204";
    let yearTwo = "2013";
    let yearThree = "2016";
    let genOne = "Comedy";
    let genTwo = "Mystery";
    let genThree = "Horror";
    let y = output::buildRank(6);
    let z = output::buildRank (10);
    let zz = output::buildRank(2);
    let mut input = Vec::new();
    input.push("Comedy");
    input.push("Cool Cat");
    let mut inputTwo = Vec::new();
    inputTwo.push("**");
    inputTwo.push("Die Telefloppies");
    inputTwo.push("Darth Vader");

    output::outputResult(x, &y, &act, genOne, yearOne, &input);
    output::outputResult(xx, &z, &actTwo, genTwo, yearTwo, &input);
    output::outputResult(xxx, &zz, &actThree, genThree, yearThree, &inputTwo);
}
