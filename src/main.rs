use clap::{arg, Command};
use pihex::*;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), err);
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(arg!(--formula <FORMULA> "Formula to use (bbp or bellard)"))
        .arg(arg!([PLACE] "Place of digits to calculate (defaults to 0)"))
        .get_matches();
    let pihex: fn(u64) -> String = match matches.get_one::<String>("formula").map(String::as_ref) {
        Some("bellard") => bellard::pihex,
        Some("bbp") | None => bbp::pihex,
        Some(formula) => return Err(format!("unknown formula: {}", formula)),
    };
    let place: u64 = matches
        .get_one::<String>("PLACE")
        .and_then(|x| x.parse().ok())
        .unwrap_or(0);
    print!("{}:", place);
    for i in 0..8 {
        print!(" {}", pihex(place + 4 * i));
    }
    println!();
    Ok(())
}
