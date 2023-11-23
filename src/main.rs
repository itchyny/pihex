use clap::{arg, Command};
use pihex::*;
use thiserror::Error;

fn main() {
    if let Err(err) = run() {
        eprintln!("{}: {}", env!("CARGO_PKG_NAME"), err);
        std::process::exit(1);
    }
}

#[derive(Debug, Error)]
enum Error {
    #[error("unknown formula: {0}")]
    Formula(String),

    #[error("invalid place: {0}")]
    Place(#[from] std::num::ParseIntError),
}

fn run() -> Result<(), Error> {
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
        Some(formula) => return Err(Error::Formula(formula.to_owned())),
    };
    let place: u64 = match matches.get_one::<String>("PLACE") {
        Some(x) => x.parse()?,
        None => 0,
    };
    print!("{}:", place);
    for i in 0..8 {
        print!(" {}", pihex(place + 4 * i));
    }
    println!();
    Ok(())
}
