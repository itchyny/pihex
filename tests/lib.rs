extern crate pihex;
use pihex::*;

const TEST_CASES: &[(u32, &str)] = &[(0, "243f"),
                                     (1, "43f6"),
                                     (2, "3f6a"),
                                     (3, "f6a8"),
                                     (4, "6a88"),
                                     (8, "85a3"),
                                     (12, "08d3"),
                                     (96, "c0ac"),
                                     (128, "9216"),
                                     (1024, "25d4"),
                                     (3704, "34c6"),
                                     (4096, "5a04"),
                                     (4527, "e4f3"),
                                     (8192, "77af"),
                                     (2090, "e674"),
                                     (2944, "5094"),
                                     (11027, "e70f"),
                                     (11742, "41fb"),
                                     (15367, "0c67")];

#[test]
fn bbp_pihex_test() {
    for &(d, hex) in TEST_CASES {
        assert_eq!(bbp::pihex(d), hex);
    }
}

#[test]
fn pihex_bellard_test() {
    for &(d, hex) in TEST_CASES {
        assert_eq!(bellard::pihex(d), hex);
    }
}

#[test]
fn bbp_pihex_bellard_pihex_test() {
    for i in 0..1000 {
        assert_eq!((i, bbp::pihex(i)), (i, bellard::pihex(i)));
    }
}
