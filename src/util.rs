pub fn pow_mod(n: u64, m: u64, d: u64) -> u64 {
    if n < 100 && d < 400_000_000 {
        // k * k * n < 2^64 - 1
        pow_mod_u64(n, m, d)
    } else {
        pow_mod_u128(n as u128, m as u128, d as u128) as u64
    }
}

fn pow_mod_u64(n: u64, m: u64, d: u64) -> u64 {
    if m == 0 {
        1 % d
    } else if m == 1 {
        n % d
    } else {
        let k = pow_mod_u64(n, m / 2, d);
        if m % 2 == 0 {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}

// TODO: use generics; num package does not seem to support u128
fn pow_mod_u128(n: u128, m: u128, d: u128) -> u128 {
    if m == 0 {
        1 % d
    } else if m == 1 {
        n % d
    } else {
        let k = pow_mod_u128(n, m / 2, d);
        if m % 2 == 0 {
            (k * k) % d
        } else {
            (k * k * n) % d
        }
    }
}

#[test]
fn pow_mod_test() {
    const TEST_CASES: &[(u64, u64, u64, u64)] = &[
        (0, 0, 7, 1),
        (12, 0, 7, 1),
        (12, 1, 7, 5),
        (12, 2, 7, 4),
        (12, 3, 7, 6),
        (12, 65536, 7, 2),
        (16777215, 16777216, 31, 16),
        (68719476735, 68719476736, 16777215, 32760),
        (68719476735, 68719476736, 68719476734, 1),
        (37, 9007199254740991, 281474976710655, 126094628322028),
        (65535, 4294967295, 281474976710655, 184618294348860),
    ];
    for &(n, m, d, r) in TEST_CASES {
        assert_eq!((n, m, d, pow_mod(n, m, d)), (n, m, d, r));
    }
}
