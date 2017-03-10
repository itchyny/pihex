use std::env;

extern crate pihex;
use pihex::*;

fn main() {
    let place: u32 = env::args().nth(1).and_then(|x| x.parse().ok()).unwrap_or(0);
    print!("{}: ", place);
    for i in 0..8 {
        print!("{}", bbp::pihex(place + 4 * i as u32));
        if (i + 1) % 8 == 0 {
            print!("\n")
        } else {
            print!(" ")
        }
    }
}
