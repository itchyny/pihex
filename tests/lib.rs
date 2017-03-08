extern crate pihex;
use pihex::*;

#[test]
fn pihex_test() {
    assert_eq!(pihex(0), "243f");
    assert_eq!(pihex(1), "43f6");
    assert_eq!(pihex(2), "3f6a");
    assert_eq!(pihex(3), "f6a8");
    assert_eq!(pihex(4), "6a88");
    assert_eq!(pihex(8), "85a3");
    assert_eq!(pihex(12), "08d3");
    assert_eq!(pihex(96), "c0ac");
    assert_eq!(pihex(128), "9216");
    assert_eq!(pihex(1024), "25d4");
    assert_eq!(pihex(4096), "5a04");
    assert_eq!(pihex(8192), "77af");
}
