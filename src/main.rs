use clap::{arg, Command};

fn main() {
    let matches = Command::new("prog")
        .arg(arg!(--two <VALUE>).required(true).requires("three"))
        .arg(arg!(--three <VALUE>))
        .arg(arg!(--four <VALUE>).exclusive(true))
        .get_matches();

    //println!("{:?}", matches);

    let two = match matches.get_one::<String>("two") {
        Some(two) => two,
        None => "",
    };
    let three = match matches.get_one::<String>("three") {
        Some(three) => three,
        None => "",
    };
    let four = match matches.get_one::<String>("four") {
        Some(four) => four,
        None => "",
    };

    if two.len() > 0 {
        println!("two: {}, three: {}", two, three);
    } else if four.len() > 0 {
        println!("four: {}", four);
    } else {
        println!("who knows?");
    }
}
