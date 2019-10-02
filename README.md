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
litcrypt = { git = "https://github.com/anvie/litcrypt.rs" }
```

Example:

```rust
fn main(){
    println!("his name is: {}", lc!("Voldemort"));
}
```

Before compiling, make sure you already set encryption key in `XOR_ENCRYPT_KEY` environment variable,
e.g:

    $ export XOR_ENCRYPT_KEY="myverysuperdupermegaultrasecretkey"

When compiling you may notice output like this:

    [litcrypt] encrypted: `Voldemort` -> `Ow4CEAwOAAAR`

That is LITCRYPT in action encrypting every string literal placed in `lc!`.

Check the output binary using `strings` command, e.g:

    $ strings target/debug/my_valuable_app | grep Voldemort

If the output is blank then your valuable string in your app is safe from static analyzer tool
like Hexeditor etc.


For working example code see `./examples` directory, and test it using:

    $ cargo run --example simple

[] Robin.
