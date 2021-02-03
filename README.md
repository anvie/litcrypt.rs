LITCRYPT
===========

It is abbreviation from "Literal Encryption", a Rust compiler plugin to encrypt
text literal using simple "XOR" algorithm.

LITCRYPT let's you hide your static string literal in the binary from naughty eyes seamlessly
and protect your valuable app from illegal cracking activity.

LITCRYPT works by encrypting string literal during compile time and the encrypted
string remain encrypted in both disk and memory during runtime until it will be used.

USAGE
-----

Dependencies:

```rust
[dependencies]
litcrypt = "0.2"
```

Example:

```rust
#[macro_use]
extern crate litcrypt;

use_litcrypt!("MY-SECRET-SPELL");

fn main(){
    println!("his name is: {}", lc!("Voldemort"));
}
```

`use_litcrypt!` macro call should be called first for initialization before you can
use `lc!` macro function. The first parameter is your secret key used for encrypt your
literal string. This key is also encrypted and will not visible under static analyzer.

You can also override the key using environment variable `LITCRYPT_ENCRYPT_KEY`
e.g:

    $ export LITCRYPT_ENCRYPT_KEY="myverysuperdupermegaultrasecretkey"

Litcrypt will encrypt every string you have written inside `lc!` statically.

Check the output binary using `strings` command, e.g:

    $ strings target/debug/my_valuable_app | grep Voldemort

If the output is blank then your valuable string in your app is safe from static analyzer tool
like Hexeditor etc.

For working example code see `./examples` directory, and test it using:

    $ cargo run --example simple

[] Robin.
