use crate::util;
use std::fmt::Write;
use std::sync::mpsc;
use std::thread;

pub fn pihex(d: u64) -> String {
    let (tx, rx) = mpsc::channel();
    for (j, k, l) in [
        (4, 1, -32.0),
        (4, 3, -1.0),
        (10, 1, 256.0),
        (10, 3, -64.0),
        (10, 5, -4.0),
        (10, 7, -4.0),
        (10, 9, 1.0),
    ] {
        let tx = tx.clone();
        thread::spawn(move || tx.send(l * series_sum(d, j, k)).unwrap());
    }
    drop(tx);
    let mut f = rx.iter().sum::<f64>();
    let mut s = String::with_capacity(4);
    for _ in 0..4 {
        f = (f - f.floor()) * 16.0;
        write!(&mut s, "{:x}", f.floor() as u32).unwrap();
    }
    s
}

fn series_sum(d: u64, j: u64, k: u64) -> f64 {
    let fraction1: f64 = (0..(2 * d + 2) / 5)
        .map(|i| {
            (if i % 2 == 0 { 1.0 } else { -1.0 })
                * util::pow_mod(4, 2 * d - 3 - 5 * i, j * i + k) as f64
                / (j * i + k) as f64
        })
        .fold(0.0, |x, y| (x + y).fract());
    let fraction2: f64 = ((2 * d + 2) / 5..)
        .map(|i| -(-4.0_f64).powi(-((5 * i + 3 - 2 * d) as i32)) / ((j * i + k) as f64))
        .take_while(|&x| x.abs() > 1e-13_f64)
        .sum();
    fraction1 + fraction2
}
