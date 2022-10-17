#[macro_use]
extern crate litcrypt;

use_litcrypt!("MY-SECRET-SPELL");

#[test]
pub fn test_literal1() {
    assert_eq!(lc!("Kucing Garong"), "Kucing Garong");
}

#[test]
pub fn test_literal2() {
    assert_eq!(lc!("Very secret word"), "Very secret word");
}

#[test]
pub fn test_env() {
    assert_eq!(lc_env!("SECRET_ENV"), "Shhhhhh");
}
