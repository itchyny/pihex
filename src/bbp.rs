use std::fmt::Write;
use std::sync::mpsc;
use std::thread;

use crate::util;

pub fn pihex(d: u64) -> String {
    let (tx, rx) = mpsc::channel();
    for (j, k) in [(1, 4.0), (4, -2.0), (5, -1.0), (6, -1.0)] {
        let tx = tx.clone();
        thread::spawn(move || tx.send(k * series_sum(d, j)).unwrap());
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

fn series_sum(d: u64, j: u64) -> f64 {
    let fraction1: f64 = (0..d + 1)
        .map(|i| util::pow_mod(16, d - i, 8 * i + j) as f64 / (8 * i + j) as f64)
        .fold(0.0, |x, y| (x + y).fract());
    let fraction2: f64 = (d + 1..)
        .map(|i| 16.0_f64.powi(-((i - d) as i32)) / ((8 * i + j) as f64))
        .take_while(|&x| x.abs() > 1e-13_f64)
        .sum();
    fraction1 + fraction2
}
