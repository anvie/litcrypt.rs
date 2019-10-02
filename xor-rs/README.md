xor-rs
======
[![Build Status](https://travis-ci.org/zummenix/xor-rs.svg?branch=master)]
(https://travis-ci.org/zummenix/xor-rs)

A repeating-key XOR functions written in Rust.<br />
This functions might be useful to play with
[the matasano crypto challenges](http://cryptopals.com).

###Usage

In your `Cargo.toml` need to add:
```toml
[dependencies.xor]
git = "https://github.com/zummenix/xor-rs/"
```

###Example

This example prints `Hello, world!`
```rust
extern crate xor;

fn main() {
    let source = &[95, 80, 96, 71, 120, 25, 44, 92, 120, 71, 96, 79, 54];
    let result = xor::xor(source, &[23, 53, 12, 43]);
    if let Ok(string) = std::str::from_utf8(&result) {
        println!("{}", string);
    }
}
```

LICENSE
-------

MIT
