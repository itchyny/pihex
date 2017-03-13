pub fn pow_mod(n: u128, m: u128, d: u128) -> u128 {
    if m == 0 {
        1 % d
    } else if m == 1 {
        n % d
    } else {
        let k = pow_mod(n, m / 2, d);
        if m % 2 == 0 {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}
