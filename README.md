# LITCRYPT [![Build Status](https://travis-ci.org/anvie/litcrypt.rs.svg?branch=master)](https://travis-ci.org/anvie/litcrypt.rs) ![Crates.io](https://img.shields.io/crates/v/litcrypt)

Is a short name of "Literal Encryption", a Rust proc macro that encrypts text
using a basic XOR method. It protect plain text from static analysis tools and
helps keep your important app safe from cracking activity.

LITCRYPT encrypts strings when compiling, keeping them encrypted in both disk
and memory while running, and only decrypting them when needed.

## USAGE

Dependencies:

```rust
[dependencies]
litcrypt = "0.4"
```

Example:

```rust
#[macro_use]
extern crate litcrypt;

use_litcrypt!();

fn main(){
    println!("his name is: {}", lc!("Voldemort"));
}
```

`use_litcrypt!` macro call should be called first for initialization before you
can use `lc!` macro function. The first parameter is your secret key used for
encrypt your literal string. This key is also encrypted and will not visible
under static analyzer.

Please take note that you need to set your encryption key using environment
variable `LITCRYPT_ENCRYPT_KEY` before compile: e.g:

export LITCRYPT_ENCRYPT_KEY="myverysuperdupermegaultrasecretkey"

Litcrypt will encrypt each string written inside `lc!` statically.

Check the output binary using `strings` command to verify:

```
strings target/debug/my_valuable_app | grep Voldemort
```

If the output is blank then your valuable string in your app is safe from static
analyzer tool like Hexeditor etc.

For working example code see `./examples` directory, and test using:

```
cargo run --example simple
```

[] Robin.
