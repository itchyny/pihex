#[macro_use]
extern crate clap;

extern crate pihex;
use pihex::*;

const CMD_NAME: &'static str = "pihex";

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}: {}", CMD_NAME, err);
            1
        }
    });
}

fn run() -> Result<(), String> {
    let matches = clap_app!(pihex =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@arg FORMULA: --formula +takes_value "Formula to use (bbp or bellard)")
        (@arg PLACE: "Place of digits to calculate (defaults to 0)"))
        .get_matches();
    let pihex: fn(u64) -> String = match matches.value_of("FORMULA") {
        Some("bellard") => bellard::pihex,
        Some("bbp") | None => bbp::pihex,
        Some(formula) => {
            return Err(format!("unknown formula: {}", formula));
        },
    };
    let place: u64 = matches
        .value_of("PLACE")
        .and_then(|x| x.parse().ok())
        .unwrap_or(0);
    print!("{}:", place);
    for i in 0..8 {
        print!(" {}", pihex(place + 4 * i));
    }
    print!("\n");
    Ok(())
}
