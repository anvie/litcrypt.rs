#[macro_use]
extern crate litcrypt2;

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

#[test]
pub fn test_raw1() {
    assert_eq!(lc!(r"c:\windows\system32"), r"c:\windows\system32");
}

#[test]
pub fn test_raw2() {
    assert_eq!(lc!(r#"\\machine\share"#), r#"\\machine\share"#);
}

#[test]
pub fn test_raw3() {
    assert_eq!(lc!(r###"String with ##"###), r###"String with ##"###);
}