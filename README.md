LITCRYPT2 
===========

It's a short name of "Literal Encryption", a Rust proc macro that encrypts text using a basic XOR method. It protect plain text from static analysis tools and helps keep your important app safe from cracking activity.

LITCRYPT2 encrypts strings when compiling, keeping them encrypted in both disk and memory while running, and only decrypting them when needed.

This crate is just a maintained and updated fork of the original crate, **LITCRYPT** by **Robin Syihab (r@ansvia.com)**.

USAGE
-----

Dependencies:

```rust
[dependencies]
litcrypt2 = "0.1"
```

Example:

```rust
#[macro_use]
extern crate litcrypt2;

use_litcrypt!();

fn main()
{
    println!("his name is: {}", lc!("Voldemort"));
}

fn raw_string()
{
    println!("The command line console can be found in the path {}", lc!(r"C:\Windows\System32\cmd.exe"));
}
```

`use_litcrypt!` macro call should be called first for initialization before you can
use `lc!` macro function. 

Please take note that you can set your encryption key to a specific value using the environment variable 
`LITCRYPT_ENCRYPT_KEY` before compile. In case that you don't set this environment variable, the crate
will generate a random encryption key at compilation time:
e.g:

    $ export LITCRYPT_ENCRYPT_KEY="myverysuperdupermegaultrasecretkey"

Litcrypt will encrypt each string written inside `lc!` statically.

Check the output binary using `strings` command to verify:

    $ strings target/debug/my_valuable_app | grep Voldemort

If the output is blank then your valuable string in your app is safe from static analyzer tool
like Hexeditor etc.

For working example code see `./examples` directory, and test using:

    $ cargo run --example simple
