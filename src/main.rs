#[macro_use]
extern crate clap;

extern crate pihex;
use pihex::*;

fn main() {
    let matches = clap_app!(pihex =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@arg FORMULA: --formula +takes_value "Formula to use (bbp or bellard)")
        (@arg PLACE: "Place of digits to calculate (defaults to 0)"))
        .get_matches();
    let pihex: fn(u64) -> String = match matches.value_of("FORMULA") {
        Some("bellard") => bellard::pihex,
        _ => bbp::pihex,
    };
    let place: u64 = matches.value_of("PLACE").and_then(|x| x.parse().ok()).unwrap_or(0);
    print!("{}:", place);
    for i in 0..8 {
        print!(" {}", pihex(place + 4 * i));
    }
    print!("\n")
}
