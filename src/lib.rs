use std::thread;
use std::sync::mpsc;

pub fn pihex(d: u32) -> String {
    let (tx, rx) = mpsc::channel();
    for &(j, k) in &[(1, 4.0), (4, -2.0), (5, -1.0), (6, -1.0)] {
        let tx = tx.clone();
        thread::spawn(move || tx.send(k * series_sum(d, j as u32)).unwrap());
    }
    drop(tx);
    let fraction: f64 = rx.iter().sum();
    (0..4)
        .scan(fraction, |x, _| {
            *x = (*x - x.floor()) * 16.0;
            Some(format!("{:x}", x.floor() as u64))
        })
        .fold(String::new(), |s, t| s + &t)
}

fn series_sum(d: u32, j: u32) -> f64 {
    let fraction1: f64 = (0..d + 1)
        .map(|i| (powmod(16, d - i, (8 * i + j) as u64) as f64) / ((8 * i + j) as f64))
        .sum();
    let fraction2: f64 = (d + 1..)
        .map(|i| 16.0_f64.powi(d as i32 - i as i32) / ((8 * i + j) as f64))
        .take_while(|&x| x > 1e-10_f64)
        .sum();
    (fraction1 + fraction2).fract()
}

fn powmod(n: u64, m: u32, d: u64) -> u64 {
    if m == 0 {
        1 % d
    } else if m == 1 {
        n % d
    } else {
        let k: u64 = powmod(n, m / 2, d);
        if m % 2 == 0 {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}
