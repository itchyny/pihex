pub fn powmod(n: u64, m: u64, d: u64) -> u64 {
    if m == 0 {
        1 % d
    } else if m == 1 {
        n % d
    } else {
        let k = powmod(n, m / 2, d);
        if m % 2 == 0 {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}
