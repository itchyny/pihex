use std::thread;
use std::sync::mpsc;
use util;

pub fn pihex(d: u64) -> String {
    let (tx, rx) = mpsc::channel();
    for &(j, k) in &[(1, 4.0), (4, -2.0), (5, -1.0), (6, -1.0)] {
        let tx = tx.clone();
        thread::spawn(move || tx.send(k * series_sum(d, j)).unwrap());
    }
    drop(tx);
    let fraction: f64 = rx.iter().sum();
    (0..4)
        .scan(fraction, |x, _| {
            *x = (*x - x.floor()) * 16.0;
            Some(format!("{:x}", x.floor() as u32))
        })
        .fold(String::new(), |s, t| s + &t)
}

fn series_sum(d: u64, j: u64) -> f64 {
    let fraction1: f64 = (0..d + 1)
        .map(|i| {
            util::pow_mod(16, (d - i) as u128, (8 * i + j) as u128) as f64 / (8 * i + j) as f64
        })
        .fold(0.0, |x, y| (x + y).fract());
    let fraction2: f64 = (d + 1..)
        .map(|i| 16.0_f64.powi(d as i32 - i as i32) / ((8 * i + j) as f64))
        .take_while(|&x| x > 1e-13_f64)
        .sum();
    fraction1 + fraction2
}
